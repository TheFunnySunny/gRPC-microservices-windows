package main

import (
	pb "TalkProject"
	"context"
	"log"
	"net"

	"google.golang.org/grpc"
)

type server struct{}

func (s *server) SayHello(ctx context.Context, in *pb.HelloRequest) (*pb.HelloReply, error) {
	log.Printf("Received message from %v at %v", in.Name, in.Date)
	repl := "Hello " + in.Name
	return &pb.HelloReply{Message: repl}, nil
}

func main() {
	lis, err := net.Listen("tcp", "0.0.0.0:50051")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	log.Printf("Server listening on: " + lis.Addr().String())
	pb.RegisterGreeterServer(s, &server{})
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
