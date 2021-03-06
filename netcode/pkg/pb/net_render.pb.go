// Code generated by protoc-gen-go. DO NOT EDIT.
// source: proto/net_render.proto

package pb

import (
	context "context"
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
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

// A tile represents a single renderable tile of pixels. Starts at (x,y), ends
// at (x+size, y+size).
type Tile struct {
	X                    uint32   `protobuf:"varint,1,opt,name=x,proto3" json:"x,omitempty"`
	Y                    uint32   `protobuf:"varint,2,opt,name=y,proto3" json:"y,omitempty"`
	Size                 uint32   `protobuf:"varint,3,opt,name=size,proto3" json:"size,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *Tile) Reset()         { *m = Tile{} }
func (m *Tile) String() string { return proto.CompactTextString(m) }
func (*Tile) ProtoMessage()    {}
func (*Tile) Descriptor() ([]byte, []int) {
	return fileDescriptor_9ea114fcbf489839, []int{0}
}

func (m *Tile) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Tile.Unmarshal(m, b)
}
func (m *Tile) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Tile.Marshal(b, m, deterministic)
}
func (m *Tile) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Tile.Merge(m, src)
}
func (m *Tile) XXX_Size() int {
	return xxx_messageInfo_Tile.Size(m)
}
func (m *Tile) XXX_DiscardUnknown() {
	xxx_messageInfo_Tile.DiscardUnknown(m)
}

var xxx_messageInfo_Tile proto.InternalMessageInfo

func (m *Tile) GetX() uint32 {
	if m != nil {
		return m.X
	}
	return 0
}

func (m *Tile) GetY() uint32 {
	if m != nil {
		return m.Y
	}
	return 0
}

func (m *Tile) GetSize() uint32 {
	if m != nil {
		return m.Size
	}
	return 0
}

// Pixels is the result of a single job processed by a worker.
type Pixels struct {
	Pixels               []*Pixel `protobuf:"bytes,1,rep,name=pixels,proto3" json:"pixels,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *Pixels) Reset()         { *m = Pixels{} }
func (m *Pixels) String() string { return proto.CompactTextString(m) }
func (*Pixels) ProtoMessage()    {}
func (*Pixels) Descriptor() ([]byte, []int) {
	return fileDescriptor_9ea114fcbf489839, []int{1}
}

func (m *Pixels) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_Pixels.Unmarshal(m, b)
}
func (m *Pixels) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_Pixels.Marshal(b, m, deterministic)
}
func (m *Pixels) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Pixels.Merge(m, src)
}
func (m *Pixels) XXX_Size() int {
	return xxx_messageInfo_Pixels.Size(m)
}
func (m *Pixels) XXX_DiscardUnknown() {
	xxx_messageInfo_Pixels.DiscardUnknown(m)
}

var xxx_messageInfo_Pixels proto.InternalMessageInfo

func (m *Pixels) GetPixels() []*Pixel {
	if m != nil {
		return m.Pixels
	}
	return nil
}

// Pixel represents a single pixel within the canvas.
type Pixel struct {
	X                    uint32   `protobuf:"varint,1,opt,name=x,proto3" json:"x,omitempty"`
	Y                    uint32   `protobuf:"varint,2,opt,name=y,proto3" json:"y,omitempty"`
	Color                uint32   `protobuf:"fixed32,3,opt,name=color,proto3" json:"color,omitempty"`
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *Pixel) Reset()         { *m = Pixel{} }
func (m *Pixel) String() string { return proto.CompactTextString(m) }
func (*Pixel) ProtoMessage()    {}
func (*Pixel) Descriptor() ([]byte, []int) {
	return fileDescriptor_9ea114fcbf489839, []int{2}
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

func (m *Pixel) GetX() uint32 {
	if m != nil {
		return m.X
	}
	return 0
}

func (m *Pixel) GetY() uint32 {
	if m != nil {
		return m.Y
	}
	return 0
}

func (m *Pixel) GetColor() uint32 {
	if m != nil {
		return m.Color
	}
	return 0
}

// Job is the stream of messgaes for the server to consume. Scene will be sent
// once, on the first message. The rest of the messages will all be of type
// Tile, asking for a tile to be rendered.
type Job struct {
	// Types that are valid to be assigned to Request:
	//	*Job_Scene
	//	*Job_Tile
	Request              isJob_Request `protobuf_oneof:"request"`
	XXX_NoUnkeyedLiteral struct{}      `json:"-"`
	XXX_unrecognized     []byte        `json:"-"`
	XXX_sizecache        int32         `json:"-"`
}

func (m *Job) Reset()         { *m = Job{} }
func (m *Job) String() string { return proto.CompactTextString(m) }
func (*Job) ProtoMessage()    {}
func (*Job) Descriptor() ([]byte, []int) {
	return fileDescriptor_9ea114fcbf489839, []int{3}
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

type isJob_Request interface {
	isJob_Request()
}

type Job_Scene struct {
	Scene string `protobuf:"bytes,1,opt,name=scene,proto3,oneof"`
}

type Job_Tile struct {
	Tile *Tile `protobuf:"bytes,2,opt,name=tile,proto3,oneof"`
}

func (*Job_Scene) isJob_Request() {}

func (*Job_Tile) isJob_Request() {}

func (m *Job) GetRequest() isJob_Request {
	if m != nil {
		return m.Request
	}
	return nil
}

func (m *Job) GetScene() string {
	if x, ok := m.GetRequest().(*Job_Scene); ok {
		return x.Scene
	}
	return ""
}

func (m *Job) GetTile() *Tile {
	if x, ok := m.GetRequest().(*Job_Tile); ok {
		return x.Tile
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*Job) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*Job_Scene)(nil),
		(*Job_Tile)(nil),
	}
}

func init() {
	proto.RegisterType((*Tile)(nil), "net_render.Tile")
	proto.RegisterType((*Pixels)(nil), "net_render.Pixels")
	proto.RegisterType((*Pixel)(nil), "net_render.Pixel")
	proto.RegisterType((*Job)(nil), "net_render.Job")
}

func init() { proto.RegisterFile("proto/net_render.proto", fileDescriptor_9ea114fcbf489839) }

var fileDescriptor_9ea114fcbf489839 = []byte{
	// 277 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x7c, 0x90, 0xcf, 0x4b, 0xfb, 0x40,
	0x10, 0xc5, 0xbb, 0xdf, 0xfc, 0x28, 0x9d, 0x7e, 0x45, 0x5d, 0xa4, 0x04, 0x4f, 0x25, 0x07, 0x89,
	0x22, 0x89, 0xa6, 0x20, 0x88, 0xb7, 0x9e, 0x4a, 0x4f, 0x65, 0x11, 0x04, 0x2f, 0x62, 0x92, 0xa1,
	0x86, 0xc6, 0x4c, 0xdc, 0xdd, 0x42, 0xea, 0x5f, 0x2f, 0x99, 0x08, 0x06, 0x04, 0x6f, 0xf3, 0x3e,
	0xfb, 0x66, 0xdf, 0xcc, 0xc0, 0xac, 0xd1, 0x64, 0x29, 0xa9, 0xd1, 0xbe, 0x68, 0xac, 0x0b, 0xd4,
	0x31, 0x03, 0x09, 0x3f, 0x24, 0xbc, 0x03, 0xf7, 0xb1, 0xac, 0x50, 0xfe, 0x07, 0xd1, 0x06, 0x62,
	0x2e, 0xa2, 0x23, 0x25, 0xda, 0x4e, 0x1d, 0x82, 0x7f, 0xbd, 0x3a, 0x48, 0x09, 0xae, 0x29, 0x3f,
	0x31, 0x70, 0x18, 0x70, 0x1d, 0x2e, 0xc0, 0xdf, 0x94, 0x2d, 0x56, 0x46, 0x5e, 0x82, 0xdf, 0x70,
	0x15, 0x88, 0xb9, 0x13, 0x4d, 0xd3, 0xd3, 0x78, 0x10, 0xc8, 0x1e, 0xf5, 0x6d, 0x08, 0xef, 0xc1,
	0x63, 0xf0, 0x67, 0xda, 0x19, 0x78, 0x39, 0x55, 0xa4, 0x39, 0x6e, 0xac, 0x7a, 0x11, 0x6e, 0xc0,
	0x59, 0x53, 0x26, 0x67, 0xe0, 0x99, 0x1c, 0x6b, 0xe4, 0xe6, 0xc9, 0x6a, 0xa4, 0x7a, 0x29, 0x2f,
	0xc0, 0xb5, 0x65, 0x85, 0xfc, 0xcb, 0x34, 0x3d, 0x19, 0x8e, 0xd0, 0xad, 0xb7, 0x1a, 0x29, 0x7e,
	0x5f, 0x4e, 0x60, 0xac, 0xf1, 0x63, 0x8f, 0xc6, 0xa6, 0x0f, 0xe0, 0x3f, 0x91, 0xde, 0xa1, 0x96,
	0xb7, 0xe0, 0x2b, 0xf6, 0xca, 0xe3, 0x61, 0xe3, 0x9a, 0xb2, 0x73, 0xf9, 0x6b, 0x19, 0x13, 0x89,
	0x1b, 0xb1, 0xbc, 0x7e, 0xbe, 0xda, 0x96, 0xf6, 0x6d, 0x9f, 0xc5, 0x39, 0xbd, 0x27, 0x05, 0xd5,
	0x64, 0x6b, 0x22, 0x9b, 0x58, 0xfd, 0x9a, 0xa3, 0xee, 0x4e, 0x9e, 0x53, 0x81, 0x49, 0xb3, 0xdb,
	0x26, 0x4d, 0x96, 0xf9, 0x7c, 0xf7, 0xc5, 0x57, 0x00, 0x00, 0x00, 0xff, 0xff, 0xeb, 0xdb, 0x2d,
	0xc0, 0x91, 0x01, 0x00, 0x00,
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// WorkerClient is the client API for Worker service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type WorkerClient interface {
	Render(ctx context.Context, opts ...grpc.CallOption) (Worker_RenderClient, error)
}

type workerClient struct {
	cc *grpc.ClientConn
}

func NewWorkerClient(cc *grpc.ClientConn) WorkerClient {
	return &workerClient{cc}
}

func (c *workerClient) Render(ctx context.Context, opts ...grpc.CallOption) (Worker_RenderClient, error) {
	stream, err := c.cc.NewStream(ctx, &_Worker_serviceDesc.Streams[0], "/net_render.Worker/Render", opts...)
	if err != nil {
		return nil, err
	}
	x := &workerRenderClient{stream}
	return x, nil
}

type Worker_RenderClient interface {
	Send(*Job) error
	Recv() (*Pixels, error)
	grpc.ClientStream
}

type workerRenderClient struct {
	grpc.ClientStream
}

func (x *workerRenderClient) Send(m *Job) error {
	return x.ClientStream.SendMsg(m)
}

func (x *workerRenderClient) Recv() (*Pixels, error) {
	m := new(Pixels)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// WorkerServer is the server API for Worker service.
type WorkerServer interface {
	Render(Worker_RenderServer) error
}

// UnimplementedWorkerServer can be embedded to have forward compatible implementations.
type UnimplementedWorkerServer struct {
}

func (*UnimplementedWorkerServer) Render(srv Worker_RenderServer) error {
	return status.Errorf(codes.Unimplemented, "method Render not implemented")
}

func RegisterWorkerServer(s *grpc.Server, srv WorkerServer) {
	s.RegisterService(&_Worker_serviceDesc, srv)
}

func _Worker_Render_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(WorkerServer).Render(&workerRenderServer{stream})
}

type Worker_RenderServer interface {
	Send(*Pixels) error
	Recv() (*Job, error)
	grpc.ServerStream
}

type workerRenderServer struct {
	grpc.ServerStream
}

func (x *workerRenderServer) Send(m *Pixels) error {
	return x.ServerStream.SendMsg(m)
}

func (x *workerRenderServer) Recv() (*Job, error) {
	m := new(Job)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

var _Worker_serviceDesc = grpc.ServiceDesc{
	ServiceName: "net_render.Worker",
	HandlerType: (*WorkerServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Render",
			Handler:       _Worker_Render_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
	},
	Metadata: "proto/net_render.proto",
}
