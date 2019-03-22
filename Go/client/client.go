package main

import (
	pb "TalkProject"
	"bufio"
	"context"
	"fmt"
	"log"
	"os"
	"time"

	"google.golang.org/grpc"
)

const (
	address = "127.0.0.1:50051"
)

func main() {
	// Set up a connection to the server.
	conn, err := grpc.Dial(address, grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewGreeterClient(conn)

	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Enter your name: ")
	name, _ := reader.ReadString('\n')
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	r, err := c.SayHello(ctx, &pb.HelloRequest{Name: name, Date: time.Now().String()})
	if err != nil {
		log.Fatalf("could not send: %v", err)
	}
	log.Printf("Response: %s", r.Message)
}
