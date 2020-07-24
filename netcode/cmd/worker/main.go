package main

import (
	"bufio"
	"context"
	"flag"
	"fmt"
	"log"
	"net"
	"os/exec"
	"strings"

	"github.com/donotnoot/tracer/netcode/pkg/pb"
	"github.com/golang/protobuf/ptypes/empty"
	"google.golang.org/grpc"
	"gopkg.in/yaml.v2"
)

type Server struct {
	command string
}

// Work accepts jobs and returns streams of rendered pixels.
func (s *Server) Work(job *pb.Job, stream pb.Worker_WorkServer) error {
	log.Println("accepted job, starting distracer")

	yamlFile := make(map[string]interface{})
	err := yaml.Unmarshal([]byte(job.Spec), &yamlFile)
	if err != nil {
		log.Fatalf("error: %v", err)
	}
	yamlFile["rendering"].(map[interface{}]interface{})["partial_render"] = func() [][2]uint32 {
		r := make([][2]uint32, 0)
		for _, pos := range job.Pixels {
			r = append(r, [2]uint32{pos.X, pos.Y})
		}
		return r
	}()
	yamlFilePartialRender, err := yaml.Marshal(&yamlFile)
	if err != nil {
		log.Fatalf("error: %v", err)
	}

	spl := strings.Split(s.command, " ")
	cmd := exec.Command(spl[0], spl[1:]...)
	stdin, err := cmd.StdinPipe()
	if err != nil {
		log.Println("could not open stdin pipe", err)
		return err
	}

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		log.Println("could not open stdout pipe", err)
		return err
	}
	defer stdout.Close()
	scanner := bufio.NewScanner(stdout)

	if err := cmd.Start(); err != nil {
		log.Println("could not start command", err)
		return err
	}

	if _, err := stdin.Write(yamlFilePartialRender); err != nil {
		log.Println("could not write string", err)
		return err
	}
	if err := stdin.Close(); err != nil {
		log.Println("could not close stdin", err)
		return err
	}

	completed := 0
	for scanner.Scan() {
		line := scanner.Text()
		pixel := &pb.Pixel{
			R: 0.0,
			G: 0.0,
			B: 0.0,
			Position: &pb.Position{
				X: 0,
				Y: 0,
			},
		}

		fmt.Sscanf(line, "%d %d %f %f %f",
			&pixel.Position.X,
			&pixel.Position.Y,
			&pixel.R,
			&pixel.G,
			&pixel.B)

		if err := stream.Send(pixel); err != nil {
			log.Println("could not receive pixel", err)
			return err
		}

		completed++

		if completed == len(job.Pixels) {
			break
		}
	}

	if err := cmd.Wait(); err != nil {
		log.Println("could not wait for command to finish", err)
		return err
	}

	return nil
}

// Benchmark returns the benchmark score of the worker.
func (s *Server) Benchmark(ctx context.Context, _ *empty.Empty) (*pb.Score, error) {
	panic("not implemented") // TODO: Implement
}

var (
	port    = flag.Int("port", 9000, "port to listen on")
	command = flag.String("command", "distracer", "commad to run distracer")
)

func main() {
	flag.Parse()

	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Printf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()
	pb.RegisterWorkerServer(grpcServer, &Server{*command})

	if err := grpcServer.Serve(lis); err != nil {
		panic(err)
	}
}
