SHELL=bash

BUILD_DIR = bin

all: bin/tracer

default: all

clean:
	rm -rf bin

test:
	go test ./... -race -parallel=$(shell nproc) -count=3

$(BUILD_DIR)/%:
	go build -o $@ ./cmd/$*/*.go
