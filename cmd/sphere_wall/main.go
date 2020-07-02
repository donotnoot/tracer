package main

import (
	"flag"
	"log"
	"os"
	"runtime/pprof"

	"github.com/donotnoot/tracer/pkg/openglcanvas"
	"github.com/donotnoot/tracer/pkg/tracer"
)

var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to `file`")

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

	canvasPixels := 1000
	canvas := openglcanvas.NewOpenGLCanvas(canvasPixels, canvasPixels, "OpenGL Canvas")

	rayOrigin := tracer.Point(0, 0, -5)
	wallZ := 10.0
	wallSize := 7.0
	pixelSize := wallSize / float64(canvasPixels)
	halfWall := 7.0 / 2.0

	sphere := tracer.NewSphere()
	sphere.Material = tracer.NewMaterial()
	sphere.Material.Color = tracer.Color(1, .2, 1)

	light := &tracer.PointLight{
		Position:  tracer.Point(-10, 10, -10),
		Intensity: tracer.Color(1, 1, 1),
	}

	pace := make(chan struct{}, 256)

	go func() {
		for y := 0; y < canvasPixels; y++ {
			worldY := halfWall - pixelSize*float64(y)
			for x := 0; x < canvasPixels; x++ {
				worldX := -halfWall + pixelSize*float64(x)
				wallPoint := tracer.Point(worldX, worldY, wallZ)

				// Trace rays concurrently
				go func(x, y int) {
					pace <- struct{}{}
					defer func() { <-pace }()

					ray := &tracer.Ray{rayOrigin, tracer.SubTup(wallPoint, rayOrigin).Normalize()}
					xs := sphere.Intersect(ray)
					if xs != nil {
						color := sphere.Material.Lighting(
							light,
							ray.Position(xs[0].T).Normalize(),
							tracer.NegTup(ray.Direction),
							sphere.Normal(ray.Position(xs[0].T)),
						)
						canvas.WritePixel(x, y, color)
					}
				}(x, y)
			}
		}
	}()

	// Must run on the main thread, hence the work being done on the gorountine
	// above.
	canvas.Run()
}
