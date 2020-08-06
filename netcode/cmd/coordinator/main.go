package main

import (
	"context"
	"flag"
	"io"
	"io/ioutil"
	"log"
	"math/rand"
	"sync"
	"sync/atomic"
	"time"

	"github.com/donotnoot/tracer/netcode/pkg/pb"
	rl "github.com/gen2brain/raylib-go/raylib"
	"github.com/google/uuid"
	"google.golang.org/grpc"
	"gopkg.in/yaml.v2"
)

// Network represents the network config for the render.
type Network struct {
	Workers []WorkerSpec `yaml:"workers"`
	Tiling  TilingSpec   `yaml:"tiling"`
}

type WorkerSpec struct {
	Address string `yaml:"address"`
	Name    string `yaml:"name"`
}

type TilingSpec struct {
	Size int `yaml:"size"`
}

// Scene represents the scene to be rendered.
type Scene struct {
	id         string
	CameraSpec CameraSpec `yaml:"camera"`
}

// CameraSpec represents the specs for the camera in the scene.
type CameraSpec struct {
	Width  float32 `yaml:"width"`
	Height float32 `yaml:"height"`
}

type Worker struct {
	Address        string
	Name           string
	Client         pb.Worker_RenderClient
	CompletedTiles int32
}

var (
	networkFile = flag.String("network", "", "network specification file")
	sceneFile   = flag.String("scene", "", "scene specification file")
)

func main() {
	ctx := context.Background()
	rand.Seed(time.Now().UnixNano())
	flag.Parse()

	var sceneRaw string
	scene := &Scene{id: uuid.New().String()}
	{
		log.Println("reading scene specification...")
		contents, err := ioutil.ReadFile(*sceneFile)
		if err != nil {
			log.Fatal(err)
		}
		if err := yaml.Unmarshal(contents, scene); err != nil {
			log.Fatal("could not read scene spec", err)
		}
		sceneRaw = string(contents)
		log.Println("scene spec OK")
	}
	height := int32(scene.CameraSpec.Height)
	width := int32(scene.CameraSpec.Width)

	network := &Network{}
	{
		log.Println("reading network specification...")
		contents, err := ioutil.ReadFile(*networkFile)
		if err != nil {
			log.Fatal(err)
		}
		if err := yaml.Unmarshal(contents, network); err != nil {
			log.Fatal("could not read network spec", err)
		}
		log.Println("network spec OK")
	}

	workers := make([]*Worker, 0, len(network.Workers))
	for _, worker := range network.Workers {
		worker := worker

		log.Println("dialling", worker.Address)
		connection, err := grpc.Dial(worker.Address, grpc.WithInsecure())
		if err != nil {
			log.Fatalf("fail to dial: %v", err)
		}
		defer func() {
			log.Println("closing connection to", worker.Address, connection.Close())
		}()
		client := pb.NewWorkerClient(connection)

		renderClient, err := client.Render(ctx)
		if err != nil {
			log.Fatalf("could not establish render connection with %s: %s", worker.Address, err)
		}
		defer func() {
			log.Println("closing render client stream from", worker.Address, renderClient.CloseSend())
		}()

		if err := renderClient.Send(&pb.Job{
			Request: &pb.Job_Scene{
				Scene: sceneRaw,
			},
		}); err != nil {
			log.Fatalf("could not send the spec to %s: %s", worker.Address, err)
		}
		log.Printf("scene sent to %s successfully", worker.Address)

		workers = append(workers, &Worker{
			Address:        worker.Address,
			Client:         renderClient,
			Name:           worker.Name,
			CompletedTiles: 0,
		})
	}

	// Make sure all the tiles are dealt with, exactly once.
	tiles := make(chan *pb.Tile)
	tileList := make([]*pb.Tile, 0, int(width*height))
	go func() {
		defer close(tiles)

		for x := int32(0); x < width; x += int32(network.Tiling.Size) {
			for y := int32(0); y < height; y += int32(network.Tiling.Size) {
				tileList = append(tileList, &pb.Tile{X: uint32(x), Y: uint32(y), Size: uint32(network.Tiling.Size)})
			}
		}

		rand.Shuffle(len(tileList), func(a, b int) { tileList[a], tileList[b] = tileList[b], tileList[a] })

		for _, tile := range tileList {
			tiles <- tile
		}
	}()

	wg := &sync.WaitGroup{}
	pixels := make(chan *pb.Pixel)
	startedAt := time.Now()
	go func() {
		wg.Wait()

		close(pixels)
		log.Println("completed in", time.Since(startedAt))
		for _, worker := range workers {
			percentage := float64(worker.CompletedTiles) / float64(len(tileList)) * 100
			log.Printf("%q completed %d tiles, that's %.2f%%", worker.Name, worker.CompletedTiles, percentage)
		}
	}()

	// Read from the pixels channel and append them to the buffer, to later
	// draw them.
	buffer := make([]*pb.Pixel, 0)
	go func() {
		for pixel := range pixels {
			buffer = append(buffer, pixel)
		}
	}()

	// Finally, send the work to the workers!
	wg.Add(len(workers))
	for _, client := range workers {
		go func(worker *Worker) {
			defer wg.Done()

			for tile := range tiles {
				if err := worker.Client.Send(&pb.Job{
					Request: &pb.Job_Tile{
						Tile: tile,
					},
				}); err != nil {
					log.Fatal(err)
				}
				sentAt := time.Now()

				// Wait for the result...
				recv, err := worker.Client.Recv()
				if err != nil {
					if err == io.EOF {
						break
					}
					log.Fatal(err)
				}
				log.Printf("%q processed tile in %v", worker.Name, time.Since(sentAt))
				atomic.AddInt32(&worker.CompletedTiles, 1)
				for _, pixel := range recv.Pixels {
					pixels <- pixel
				}
			}
		}(client)
	}

	rl.InitWindow(width, height, "Distracer!")
	defer rl.CloseWindow()
	rl.SetTargetFPS(144)

	for !rl.WindowShouldClose() {
		rl.BeginDrawing()
		rl.ClearBackground(rl.RayWhite)

		for _, p := range buffer {
			rl.DrawPixel(int32(p.X), int32(p.Y), pixelToRlColor(p.Color))
		}

		rl.EndDrawing()
	}
}

func pixelToRlColor(p uint32) rl.Color {
	return rl.Color{
		R: uint8(p >> 16),
		G: uint8(p >> 8),
		B: uint8(p),
		A: 255,
	}
}
