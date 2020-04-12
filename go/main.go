// Package main implements a server for DB.
package main

import (
	"context"
	"log"
	"math/rand"
	"net"
	"sync"

	"google.golang.org/grpc"
	empty "github.com/golang/protobuf/ptypes/empty"
	pb "./pb"
)

const (
	port = ":50051"
)

// Server is used to implement db.DBServer.
type Server struct {
	sync.RWMutex
	records []int32
}

// Initialize with random records.
func NewServer () *Server {
	const N int32 = 1e6
	records := make([]int32, N)
	var i int32 = 0
	for ;i < N; i++ {
		records[i] = rand.Int31n(N / 10)
	}
	return &Server{records: records}
}

// Implement RPC methods.
func (s *Server) Search(ctx context.Context, in *pb.Value) (*pb.Records, error) {
	query := in.GetValue()
	records := make([]*pb.Record, 0)
	s.RLock()
	for index, value := range s.records {
		if value == query {
			records = append(records, &pb.Record{Index: int32(index), Value: value})
		}
	}
	s.RUnlock()
	return &pb.Records{Records: records}, nil
}

func (s *Server) Add(ctx context.Context, in *pb.Values) (*empty.Empty, error) {
	s.Lock()
	for _, value := range in.GetValues() {
		s.records = append(s.records, value)
	}
	s.Unlock()
	return &empty.Empty{}, nil
}

func (s *Server) Update(ctx context.Context, in *pb.Records) (*empty.Empty, error) {
	s.Lock()
	for _, record := range in.GetRecords() {
		s.records[record.GetIndex()] = record.GetValue()
	}
	s.Unlock()
	return &empty.Empty{}, nil
}

func (s *Server) Delete(ctx context.Context, in *pb.Indexes) (*empty.Empty, error) {
	s.Lock()
	for _, index := range in.GetIndexes() {
		s.records = append(s.records[:index], s.records[index + 1:]...)
	}
	s.Unlock()
	return &empty.Empty{}, nil
}

// Run gRPC server.
func main() {
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	pb.RegisterDBServer(s, NewServer())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
