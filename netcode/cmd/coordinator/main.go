package main

import (
	"context"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"

	"github.com/donotnoot/tracer/netcode/pkg/pb"
	"google.golang.org/grpc"
)

type Coordinator struct{}

var (
	address = flag.String("port", "localhost:9000", "port to connect to")
	command = flag.String("command", "distracer", "commad to run distracer")
)

func main() {
	flag.Parse()

	conn, err := grpc.Dial(*address, grpc.WithInsecure())
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	defer conn.Close()
	client := pb.NewWorkerClient(conn)

	spec, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		log.Fatal("coould not read stdin", err)
	}

	stream, err := client.Work(context.Background(), &pb.Job{
		Spec: string(spec),
		Pixels: []*pb.Position{
			{X: 100, Y: 100},
		},
	})

	for {
		pixel, err := stream.Recv()
		if err != nil {
			panic(err)
		}
		fmt.Println(pixel)
	}
}
