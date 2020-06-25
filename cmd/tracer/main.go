package main

import (
	"fmt"

	"github.com/donotnoot/tracer/pkg/tracer"
)

func main() {
	fmt.Println(tracer.IdMat4)
	fmt.Println()
	fmt.Println(tracer.MatrixInverse(&tracer.IdMat4))
	fmt.Println()
	m := &tracer.Mat4{
		{-5, 2, 6, -8},
		{1, -5, 1, 8},
		{7, 7, -6, -7},
		{1, -3, 7, 4},
	}
	fmt.Println(tracer.MatrixMultiply(tracer.MatrixInverse(m).(*tracer.Mat4), m))
}
