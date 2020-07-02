package main

import (
	"math"
	"os"

	"github.com/donotnoot/tracer/pkg/tracer"
)

func main() {
	canvas := tracer.NewPPMCanvas(800, 800)

	for r := float64(0); r < math.Pi*2; r += math.Pi / float64(6) {
		p := tracer.Point(0, 200, 0)
		rot := tracer.RotateZMatrix(r)
		rotatedPoint := tracer.MatrixTupMultiply(rot, p)
		canvas.WritePixel(
			int(rotatedPoint.X)+400,
			int(rotatedPoint.Y)+400,
			tracer.Color(1, 1, 1))
	}

	os.Stdout.Write(canvas.PPM())
}
