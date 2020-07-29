PROTOC_INCLUDES= -I. \
	-I/usr/local/include

GO_MODULE_ROOT=github.com/donotnoot/tracer

default: netcode/pkg/pb/net_render.pb.go

clean:
	rm -rf netcode/bin netcode/pkg/pb

netcode/pkg/pb/%.pb.go:
	protoc ${PROTOC_INCLUDES} \
		--go_out=plugins=grpc:. \
		./proto/$*.proto
	mv ${GO_MODULE_ROOT}/netcode/pkg/* netcode/pkg
	rm -rf github.com
