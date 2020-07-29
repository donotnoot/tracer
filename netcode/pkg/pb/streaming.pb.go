// Code generated by protoc-gen-go. DO NOT EDIT.
// source: proto/streaming.proto

package pb

import (
	context "context"
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	empty "github.com/golang/protobuf/ptypes/empty"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

// Job is a single job for a worker.
type Job struct {
	// spec is (for now) YAML for the scene.
	Spec string `protobuf:"bytes,1,opt,name=spec,proto3" json:"spec,omitempty"`
	// pixels are the pixels that the worker is supposed to render.
	Pixels               []*Position `protobuf:"bytes,2,rep,name=pixels,proto3" json:"pixels,omitempty"`
	XXX_NoUnkeyedLiteral struct{}    `json:"-"`
	XXX_unrecognized     []byte      `json:"-"`
	XXX_sizecache        int32       `json:"-"`
}

func (m *Job) Reset()         { *m = Job{} }
func (m *Job) String() string { return proto.CompactTextString(m) }
func (*Job) ProtoMessage()    {}
func (*Job) Descriptor() ([]byte, []int) {
	return fileDescriptor_5556cc946f1d51e2, []int{0}
}

func (m *Job) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Job.Unmarshal(m, b)
}
func (m *Job) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Job.Marshal(b, m, deterministic)
}
func (m *Job) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Job.Merge(m, src)
}
func (m *Job) XXX_Size() int {
	return xxx_messageInfo_Job.Size(m)
}
func (m *Job) XXX_DiscardUnknown() {
	xxx_messageInfo_Job.DiscardUnknown(m)
}

var xxx_messageInfo_Job proto.InternalMessageInfo

func (m *Job) GetSpec() string {
	if m != nil {
		return m.Spec
	}
	return ""
}

func (m *Job) GetPixels() []*Position {
	if m != nil {
		return m.Pixels
	}
	return nil
}

// Position represents the position of a single point within the canvas.
type Position struct {
	X                    uint32   `protobuf:"varint,1,opt,name=x,proto3" json:"x,omitempty"`
	Y                    uint32   `protobuf:"varint,2,opt,name=y,proto3" json:"y,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *Position) Reset()         { *m = Position{} }
func (m *Position) String() string { return proto.CompactTextString(m) }
func (*Position) ProtoMessage()    {}
func (*Position) Descriptor() ([]byte, []int) {
	return fileDescriptor_5556cc946f1d51e2, []int{1}
}

func (m *Position) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Position.Unmarshal(m, b)
}
func (m *Position) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Position.Marshal(b, m, deterministic)
}
func (m *Position) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Position.Merge(m, src)
}
func (m *Position) XXX_Size() int {
	return xxx_messageInfo_Position.Size(m)
}
func (m *Position) XXX_DiscardUnknown() {
	xxx_messageInfo_Position.DiscardUnknown(m)
}

var xxx_messageInfo_Position proto.InternalMessageInfo

func (m *Position) GetX() uint32 {
	if m != nil {
		return m.X
	}
	return 0
}

func (m *Position) GetY() uint32 {
	if m != nil {
		return m.Y
	}
	return 0
}

// Pixel represents a single pixel within the canvas.
type Pixel struct {
	R                    float64   `protobuf:"fixed64,1,opt,name=r,proto3" json:"r,omitempty"`
	G                    float64   `protobuf:"fixed64,2,opt,name=g,proto3" json:"g,omitempty"`
	B                    float64   `protobuf:"fixed64,3,opt,name=b,proto3" json:"b,omitempty"`
	Position             *Position `protobuf:"bytes,4,opt,name=position,proto3" json:"position,omitempty"`
	XXX_NoUnkeyedLiteral struct{}  `json:"-"`
	XXX_unrecognized     []byte    `json:"-"`
	XXX_sizecache        int32     `json:"-"`
}

func (m *Pixel) Reset()         { *m = Pixel{} }
func (m *Pixel) String() string { return proto.CompactTextString(m) }
func (*Pixel) ProtoMessage()    {}
func (*Pixel) Descriptor() ([]byte, []int) {
	return fileDescriptor_5556cc946f1d51e2, []int{2}
}

func (m *Pixel) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Pixel.Unmarshal(m, b)
}
func (m *Pixel) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Pixel.Marshal(b, m, deterministic)
}
func (m *Pixel) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Pixel.Merge(m, src)
}
func (m *Pixel) XXX_Size() int {
	return xxx_messageInfo_Pixel.Size(m)
}
func (m *Pixel) XXX_DiscardUnknown() {
	xxx_messageInfo_Pixel.DiscardUnknown(m)
}

var xxx_messageInfo_Pixel proto.InternalMessageInfo

func (m *Pixel) GetR() float64 {
	if m != nil {
		return m.R
	}
	return 0
}

func (m *Pixel) GetG() float64 {
	if m != nil {
		return m.G
	}
	return 0
}

func (m *Pixel) GetB() float64 {
	if m != nil {
		return m.B
	}
	return 0
}

func (m *Pixel) GetPosition() *Position {
	if m != nil {
		return m.Position
	}
	return nil
}

// Score represents the benchmark score of a worker.
type Score struct {
	FinishedIn           float64  `protobuf:"fixed64,1,opt,name=finished_in,json=finishedIn,proto3" json:"finished_in,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *Score) Reset()         { *m = Score{} }
func (m *Score) String() string { return proto.CompactTextString(m) }
func (*Score) ProtoMessage()    {}
func (*Score) Descriptor() ([]byte, []int) {
	return fileDescriptor_5556cc946f1d51e2, []int{3}
}

func (m *Score) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Score.Unmarshal(m, b)
}
func (m *Score) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Score.Marshal(b, m, deterministic)
}
func (m *Score) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Score.Merge(m, src)
}
func (m *Score) XXX_Size() int {
	return xxx_messageInfo_Score.Size(m)
}
func (m *Score) XXX_DiscardUnknown() {
	xxx_messageInfo_Score.DiscardUnknown(m)
}

var xxx_messageInfo_Score proto.InternalMessageInfo

func (m *Score) GetFinishedIn() float64 {
	if m != nil {
		return m.FinishedIn
	}
	return 0
}

func init() {
	proto.RegisterType((*Job)(nil), "streaming.Job")
	proto.RegisterType((*Position)(nil), "streaming.Position")
	proto.RegisterType((*Pixel)(nil), "streaming.Pixel")
	proto.RegisterType((*Score)(nil), "streaming.Score")
}

func init() { proto.RegisterFile("proto/streaming.proto", fileDescriptor_5556cc946f1d51e2) }

var fileDescriptor_5556cc946f1d51e2 = []byte{
	// 317 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x74, 0x90, 0x4f, 0x4b, 0x03, 0x31,
	0x10, 0xc5, 0x49, 0xff, 0xd1, 0x4e, 0x55, 0x24, 0xa2, 0x2c, 0xf5, 0x60, 0xd9, 0x83, 0x94, 0x2a,
	0x1b, 0xa9, 0xf8, 0x05, 0x0a, 0x0a, 0xf6, 0x54, 0xd6, 0x83, 0xe0, 0x45, 0x9a, 0x6d, 0x9a, 0x86,
	0xed, 0x66, 0x42, 0x36, 0x85, 0xf6, 0xdb, 0x4b, 0xb2, 0xdd, 0x5a, 0x0f, 0xde, 0xf2, 0x7b, 0x79,
	0xf3, 0x66, 0x78, 0x70, 0x6d, 0x2c, 0x3a, 0x64, 0xa5, 0xb3, 0x62, 0x51, 0x28, 0x2d, 0x93, 0xc0,
	0xb4, 0x77, 0x14, 0x06, 0xb7, 0x12, 0x51, 0x6e, 0x04, 0x0b, 0x1f, 0x7c, 0xbb, 0x62, 0xa2, 0x30,
	0x6e, 0x5f, 0xf9, 0xe2, 0x37, 0x68, 0xce, 0x90, 0x53, 0x0a, 0xad, 0xd2, 0x88, 0x2c, 0x22, 0x43,
	0x32, 0xea, 0xa5, 0xe1, 0x4d, 0x1f, 0xa0, 0x63, 0xd4, 0x4e, 0x6c, 0xca, 0xa8, 0x31, 0x6c, 0x8e,
	0xfa, 0x93, 0xab, 0xe4, 0x77, 0xc9, 0x1c, 0x4b, 0xe5, 0x14, 0xea, 0xf4, 0x60, 0x89, 0xef, 0xa1,
	0x5b, 0x6b, 0xf4, 0x0c, 0xc8, 0x2e, 0x24, 0x9d, 0xa7, 0x64, 0xe7, 0x69, 0x1f, 0x35, 0x2a, 0xda,
	0xc7, 0x1c, 0xda, 0x73, 0x3f, 0xe1, 0x65, 0x1b, 0x4c, 0x24, 0x25, 0xd6, 0x93, 0x0c, 0x26, 0x92,
	0x12, 0xe9, 0x89, 0x47, 0xcd, 0x8a, 0x38, 0x65, 0xd0, 0x35, 0x87, 0xe8, 0xa8, 0x35, 0x24, 0xff,
	0x5d, 0x72, 0x34, 0xc5, 0x23, 0x68, 0x7f, 0x64, 0x68, 0x05, 0xbd, 0x83, 0xfe, 0x4a, 0x69, 0x55,
	0xae, 0xc5, 0xf2, 0x5b, 0xe9, 0xc3, 0x36, 0xa8, 0xa5, 0x77, 0x3d, 0xc9, 0xa1, 0xf3, 0x89, 0x36,
	0x17, 0x96, 0x8e, 0xa1, 0xe5, 0x5f, 0xf4, 0xe2, 0x24, 0x7a, 0x86, 0x7c, 0x70, 0x79, 0xba, 0xca,
	0x1f, 0xfe, 0x44, 0xe8, 0x0b, 0xf4, 0xa6, 0x42, 0x67, 0xeb, 0x62, 0x61, 0x73, 0x7a, 0x93, 0x54,
	0xf5, 0x26, 0x75, 0xbd, 0xc9, 0xab, 0xaf, 0xf7, 0xcf, 0x60, 0xb8, 0x66, 0xfa, 0xf8, 0x35, 0x96,
	0xca, 0xad, 0xb7, 0x3c, 0xc9, 0xb0, 0x60, 0x4b, 0xd4, 0xe8, 0x34, 0xa2, 0x63, 0xce, 0x2e, 0x32,
	0x61, 0x99, 0x16, 0x2e, 0xc3, 0xa5, 0x60, 0x26, 0x97, 0xcc, 0x70, 0xde, 0x09, 0x79, 0xcf, 0x3f,
	0x01, 0x00, 0x00, 0xff, 0xff, 0x79, 0xb0, 0xf4, 0xb5, 0xe0, 0x01, 0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConnInterface

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion6

// WorkerClient is the client API for Worker service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type WorkerClient interface {
	// Work accepts jobs and returns streams of rendered pixels.
	Work(ctx context.Context, in *Job, opts ...grpc.CallOption) (Worker_WorkClient, error)
	// Benchmark returns the benchmark score of the worker.
	Benchmark(ctx context.Context, in *empty.Empty, opts ...grpc.CallOption) (*Score, error)
}

type workerClient struct {
	cc grpc.ClientConnInterface
}

func NewWorkerClient(cc grpc.ClientConnInterface) WorkerClient {
	return &workerClient{cc}
}

func (c *workerClient) Work(ctx context.Context, in *Job, opts ...grpc.CallOption) (Worker_WorkClient, error) {
	stream, err := c.cc.NewStream(ctx, &_Worker_serviceDesc.Streams[0], "/streaming.Worker/Work", opts...)
	if err != nil {
		return nil, err
	}
	x := &workerWorkClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type Worker_WorkClient interface {
	Recv() (*Pixel, error)
	grpc.ClientStream
}

type workerWorkClient struct {
	grpc.ClientStream
}

func (x *workerWorkClient) Recv() (*Pixel, error) {
	m := new(Pixel)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *workerClient) Benchmark(ctx context.Context, in *empty.Empty, opts ...grpc.CallOption) (*Score, error) {
	out := new(Score)
	err := c.cc.Invoke(ctx, "/streaming.Worker/Benchmark", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// WorkerServer is the server API for Worker service.
type WorkerServer interface {
	// Work accepts jobs and returns streams of rendered pixels.
	Work(*Job, Worker_WorkServer) error
	// Benchmark returns the benchmark score of the worker.
	Benchmark(context.Context, *empty.Empty) (*Score, error)
}

// UnimplementedWorkerServer can be embedded to have forward compatible implementations.
type UnimplementedWorkerServer struct {
}

func (*UnimplementedWorkerServer) Work(req *Job, srv Worker_WorkServer) error {
	return status.Errorf(codes.Unimplemented, "method Work not implemented")
}
func (*UnimplementedWorkerServer) Benchmark(ctx context.Context, req *empty.Empty) (*Score, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Benchmark not implemented")
}

func RegisterWorkerServer(s *grpc.Server, srv WorkerServer) {
	s.RegisterService(&_Worker_serviceDesc, srv)
}

func _Worker_Work_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(Job)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(WorkerServer).Work(m, &workerWorkServer{stream})
}

type Worker_WorkServer interface {
	Send(*Pixel) error
	grpc.ServerStream
}

type workerWorkServer struct {
	grpc.ServerStream
}

func (x *workerWorkServer) Send(m *Pixel) error {
	return x.ServerStream.SendMsg(m)
}

func _Worker_Benchmark_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(empty.Empty)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WorkerServer).Benchmark(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/streaming.Worker/Benchmark",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WorkerServer).Benchmark(ctx, req.(*empty.Empty))
	}
	return interceptor(ctx, in, info, handler)
}

var _Worker_serviceDesc = grpc.ServiceDesc{
	ServiceName: "streaming.Worker",
	HandlerType: (*WorkerServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Benchmark",
			Handler:    _Worker_Benchmark_Handler,
		},
	},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Work",
			Handler:       _Worker_Work_Handler,
			ServerStreams: true,
		},
	},
	Metadata: "proto/streaming.proto",
}