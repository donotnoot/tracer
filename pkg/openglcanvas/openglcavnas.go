package openglcanvas

import (
	"math"
	"runtime"
	"sync"

	"github.com/donotnoot/tracer/pkg/tracer"
	rl "github.com/gen2brain/raylib-go/raylib"
)

// Due to limitations in OpenGL, the Run func must run in the main thread.
func init() { runtime.LockOSThread() }

// OpenGLCanvas is an implementation of Canvas that draws to an OpenGL window.
type OpenGLCanvas struct {
	pixels []*tracer.Tup
	height int
	width  int
	title  string

	tx   rl.RenderTexture2D
	stop chan struct{}
	mu   *sync.Mutex
}

func NewOpenGLCanvas(width, height int, title string) *OpenGLCanvas {
	canvas := &OpenGLCanvas{
		pixels: func() []*tracer.Tup {
			pixels := make([]*tracer.Tup, width*height)
			for i := range pixels {
				pixels[i] = tracer.Color(0, 0, 0)
			}
			return pixels
		}(),
		height: height,
		width:  width,
		title:  title,
		stop:   make(chan struct{}),
		mu:     &sync.Mutex{},
	}

	return canvas
}

// Run starts the OpenGL window and continues drawing until Stop is
// concurrently called. It *must* run in the main thread. If the program needs
// to to any more work while Run is running, it must do so in a separate
// goroutine.
func (c *OpenGLCanvas) Run() {
	rl.InitWindow(int32(c.width), int32(c.height), c.title)
	rl.SetTargetFPS(60)

	c.tx = rl.LoadRenderTexture(int32(c.width), int32(c.height))

out:
	for !rl.WindowShouldClose() {
		select {
		case <-c.stop:
			break out
		default:
		}

		rl.BeginDrawing()
		rl.ClearBackground(rl.Black)
		for x := 0; x < c.width; x++ {
			for y := 0; y < c.height; y++ {
				rl.DrawPixel(int32(x), int32(y), tupToRlColor(c.PixelAt(x, y)))
			}
		}
		rl.EndDrawing()
	}

	rl.UnloadRenderTexture(c.tx)
	rl.CloseWindow()
}

// Stop gracefully stops the OpenGL canvas process.
func (c *OpenGLCanvas) Stop() { c.stop <- struct{}{} }

func tupToRlColor(t *tracer.Tup) rl.Color {
	f := func(f float64) uint8 {
		c := int(math.Ceil(255 * f))
		if c > 255 {
			c = 255
		}
		if c < 0 {
			c = 0
		}
		return uint8(c)
	}
	return rl.Color{f(t.Red()), f(t.Green()), f(t.Blue()), 255}
}

// offset returns an offset in the pixel slice from x, y coordinates.
func (c *OpenGLCanvas) offset(x, y int) int { return y*c.width + x }

// PixelAt returns a pointer to the pixel at the specified coordinates.
func (c *OpenGLCanvas) PixelAt(x, y int) *tracer.Tup { return c.pixels[c.offset(x, y)] }

// WritePixel sets the pixel at x, y to the Color tuple provided. Ignores writes out of bounds.
func (c *OpenGLCanvas) WritePixel(x, y int, color *tracer.Tup) {
	offset := c.offset(x, y)
	l := len(c.pixels)
	if offset > l {
		return
	}
	if offset < 0 {
		return
	}

	c.mu.Lock()
	defer c.mu.Unlock()
	c.pixels[offset] = color
}
