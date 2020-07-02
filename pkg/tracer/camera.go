package tracer

import (
	"math"
	"math/rand"
	"sync"
	"time"
)

type Camera struct {
	AspectRatio     float64
	FieldOfView     float64
	Hsize, Vsize    float64
	Hwidth, Hheight float64
	PixelSize       float64
	Transform       *Mat4
}

func NewCamera(hsize, vsize, fov float64) *Camera {
	c := &Camera{
		AspectRatio: hsize / vsize,
		FieldOfView: fov,
		Hsize:       hsize,
		Vsize:       vsize,
		Transform:   MatrixIdentity(4).(*Mat4),
	}

	half := math.Tan(fov / 2)
	if c.AspectRatio >= 1 {
		c.Hwidth = half
		c.Hheight = half / c.AspectRatio
	} else {
		c.Hwidth = half * c.AspectRatio
		c.Hheight = half
	}

	c.PixelSize = (c.Hwidth * 2) / c.Hsize

	return c
}

func (c *Camera) Ray(x, y float64) *Ray {
	// Offset from edge of canvas to pixel center
	xoff, yoff := (x+.5)*c.PixelSize, (y+.5)*c.PixelSize

	// Untransformed coordinates of the pixel in world space
	wx, wy := c.Hwidth-xoff, c.Hheight-yoff

	// TODO: Should pre bake this into struct field
	transformInverse := MatrixInverse(c.Transform).(*Mat4)

	pixel := MatrixTupMultiply(transformInverse, Point(wx, wy, -1))
	origin := MatrixTupMultiply(transformInverse, Point(0, 0, 0))
	direction := SubTup(pixel, origin).Normalize()

	return &Ray{origin, direction}
}

type rendererData struct {
	world  *World
	canvas Canvas
	x, y   float64
}

func (c *Camera) renderer(wg *sync.WaitGroup, jobs <-chan rendererData) {
	defer wg.Done()
	for job := range jobs {
		job.canvas.WritePixel(int(job.x), int(job.y), job.world.ColorAt(c.Ray(job.x, job.y)))
	}
}

func (c *Camera) Render(world *World, canvas Canvas, numWorkers int) {
	jobs := make(chan rendererData, numWorkers)
	wg := &sync.WaitGroup{}

	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go c.renderer(wg, jobs)
	}

	locations := make([][2]int, 0, int(c.Vsize*c.Hsize))

	for y := 0.0; y < c.Vsize-1; y++ {
		for x := 0.0; x < c.Hsize-1; x++ {
			locations = append(locations, [2]int{int(x), int(y)})
		}
	}

	rand.Seed(time.Now().UnixNano())
	rand.Shuffle(len(locations), func(a, b int) { locations[a], locations[b] = locations[b], locations[a] })

	for _, l := range locations {
		jobs <- rendererData{
			world:  world,
			canvas: canvas,
			x:      float64(l[0]),
			y:      float64(l[1]),
		}
	}

	close(jobs)
	wg.Wait()
}
