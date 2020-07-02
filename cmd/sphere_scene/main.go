package main

import (
	"flag"
	"log"
	"math"
	"os"
	"runtime/pprof"

	"github.com/donotnoot/tracer/pkg/openglcanvas"
	"github.com/donotnoot/tracer/pkg/tracer"
)

var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to `file`")

const (
	width  = 1200
	height = 800
)

func main() {
	flag.Parse()
	if *cpuprofile != "" {
		f, err := os.Create(*cpuprofile)
		if err != nil {
			log.Fatal("could not create CPU profile: ", err)
		}
		defer f.Close()
		if err := pprof.StartCPUProfile(f); err != nil {
			log.Fatal("could not start CPU profile: ", err)
		}
		defer pprof.StopCPUProfile()
	}

	canvas := openglcanvas.NewOpenGLCanvas(width, height, "OpenGL Canvas")

	w := &tracer.World{
		Objects: []tracer.Object{
			func() tracer.Object {
				floor := tracer.NewSphere()
				floor.Transform = tracer.ScalingMatrix(10, .01, 10)
				floor.Material.Color = tracer.Color(1, .9, .9)
				floor.Material.Specular = 0
				return floor
			}(),
			func() tracer.Object {
				left_wall := tracer.NewSphere()
				left_wall.Transform =
					tracer.MatrixMultiply(
						tracer.TranslationMatrix(0, 0, 5),
						tracer.MatrixMultiply(
							tracer.RotateYMatrix(-math.Pi/4),
							tracer.MatrixMultiply(
								tracer.RotateXMatrix(math.Pi/2),
								tracer.ScalingMatrix(100, .01, 100))))
				left_wall.Material.Color = tracer.Color(1, .9, .9)
				left_wall.Material.Specular = 0
				return left_wall
			}(),
			func() tracer.Object {
				right_wall := tracer.NewSphere()
				right_wall.Transform =
					tracer.MatrixMultiply(
						tracer.TranslationMatrix(0, 0, 5),
						tracer.MatrixMultiply(
							tracer.RotateYMatrix(math.Pi/4),
							tracer.MatrixMultiply(
								tracer.RotateXMatrix(math.Pi/2),
								tracer.ScalingMatrix(100, .01, 100))))
				right_wall.Material.Color = tracer.Color(1, .9, .9)
				right_wall.Material.Specular = 0
				return right_wall
			}(),
			func() tracer.Object {
				sphere := tracer.NewSphere()
				sphere.Transform = tracer.TranslationMatrix(-.5, 1, .5)
				sphere.Material.Color = tracer.Color(.1, 1, .5)
				sphere.Material.Diffuse = .7
				sphere.Material.Specular = 0.3
				return sphere
			}(),
		},
		Light: &tracer.PointLight{tracer.Point(-10, 10, -10), tracer.Color(1, 1, 1)},
	}

	c := tracer.NewCamera(width, height, math.Pi/1.2)
	c.Transform = tracer.ViewMatrix(
		tracer.Point(0, 1.5, -5),
		tracer.Point(0, 1, 0),
		tracer.Vector(0, 1, 0),
	)

	go c.Render(w, canvas, 128)

	canvas.Run()
}
