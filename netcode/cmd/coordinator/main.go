package main

import (
	"context"
	"flag"
	"io/ioutil"
	"log"
	"math/rand"
	"sync"
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

var (
	networkFile = flag.String("network", "", "network specification file")
	sceneFile   = flag.String("scene", "", "scene specification file")
)

func main() {
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

	connections := make([]pb.WorkerClient, 0, len(network.Workers))
	for _, workerSpec := range network.Workers {
		log.Println("dialling...", workerSpec.Address)
		connection, err := grpc.Dial(workerSpec.Address, grpc.WithInsecure())
		if err != nil {
			log.Fatalf("fail to dial: %v", err)
		}
		defer connection.Close()
		client := pb.NewWorkerClient(connection)
		log.Println("sending spec to worker...", scene.id)
		_, err = client.Scene(context.Background(), &pb.Spec{
			Id:   scene.id,
			Spec: sceneRaw,
		})
		if err != nil {
			log.Fatal("could not send scene to worker", err)
		}
		connections = append(connections, client)
	}

	tiles := make([]*pb.Tile, 0)
	{
		for x := int32(0); x < width; x += int32(network.Tiling.Size) {
			for y := int32(0); y < height; y += int32(network.Tiling.Size) {
				tiles = append(tiles, &pb.Tile{X: uint32(x), Y: uint32(y), Size: uint32(network.Tiling.Size)})
			}
		}
	}
	tilesPerWorker := len(tiles) / len(network.Workers)
	rand.Shuffle(len(tiles), func(a, b int) { tiles[a], tiles[b] = tiles[b], tiles[a] })

	jobs := make(map[string]*pb.Job)
	workerAddresses := make([]string, 0, len(network.Workers))
	for _, worker := range network.Workers {
		jobs[worker.Address] = &pb.Job{
			SceneId: scene.id,
			Tiles:   make([]*pb.Tile, 0, tilesPerWorker),
		}
		workerAddresses = append(workerAddresses, worker.Address)
	}

	currentWorker := 0
	for _, tile := range tiles {
		jobs[workerAddresses[currentWorker]].Tiles = append(jobs[workerAddresses[currentWorker]].Tiles, tile)
		if len(jobs[workerAddresses[currentWorker]].Tiles) >= tilesPerWorker {
			currentWorker++
		}
	}

	startedAt := time.Now()

	pixels := make(chan *pb.Pixel)
	wg := &sync.WaitGroup{}
	wg.Add(len(network.Workers))
	for i, client := range connections {
		go func(i int, client pb.WorkerClient) {
			defer wg.Done()

			stream, err := client.Work(context.Background(), jobs[workerAddresses[i]])
			if err != nil {
				panic(err)
			}

			for {
				jobResult, err := stream.Recv()
				if err != nil {
					log.Println(err)
					break
				}
				for _, pixel := range jobResult.Pixels {
					pixels <- pixel
				}
			}
		}(i, client)
	}

	buffer := make([]*pb.Pixel, 0)

	go func() {
		wg.Wait()
		close(pixels)
		log.Println("completed in", time.Since(startedAt))
	}()

	go func() {
		for pixel := range pixels {
			buffer = append(buffer, pixel)
		}
	}()

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
