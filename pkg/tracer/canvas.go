package tracer

import (
	"bytes"
	"fmt"
	"math"
	"sync"
)

// Canvas represents a canvas made out of pixels (colors).
type Canvas struct {
	Pixels []*Tup
	Height int
	Width  int

	m *sync.Mutex
}

// NewCanvas initializes a new canvas of specified dimensions.
func NewCanvas(width, height int) *Canvas {
	return &Canvas{
		Pixels: func() []*Tup {
			pixels := make([]*Tup, width*height)
			for i := range pixels {
				pixels[i] = Color(0, 0, 0)
			}
			return pixels
		}(),
		Height: height,
		Width:  width,
		m:      &sync.Mutex{},
	}
}

// offset returns an offset in the pixel slice from x, y coordinates.
func (c *Canvas) offset(x, y int) int { return y*c.Width + x }

// PixelAt returns a pointer to the pixel at the specified coordinates.
func (c *Canvas) PixelAt(x, y int) *Tup { return c.Pixels[c.offset(x, y)] }

// WritePixel sets the pixel at x, y to the Color tuple provided. Ignores writes out of bounds.
func (c *Canvas) WritePixel(x, y int, color *Tup) {
	offset := c.offset(x, y)
	l := len(c.Pixels)
	if offset > l {
		return
	}
	if offset < 0 {
		return
	}
	c.m.Lock()
	defer c.m.Unlock()
	c.Pixels[offset] = color
}

// WritePixelUnsafely writes a pixel to the canvas but it's not threadsafe.
func (c *Canvas) WritePixelUnsafely(x, y int, color *Tup) {
	offset := c.offset(x, y)
	l := len(c.Pixels)
	if offset > l {
		return
	}
	if offset < 0 {
		return
	}
	c.Pixels[offset] = color
}

// PPM generates a PPM byte array representing the canvas
func (c *Canvas) PPM() []byte {
	c.m.Lock()
	defer c.m.Unlock()

	buffer := &bytes.Buffer{}

	fmt.Fprintf(buffer, "P3\n%d %d\n255\n", c.Width, c.Height)

	scale := func(p *Tup) [3]int {
		s := func(f float64) int {
			c := int(math.Ceil(255 * f))
			if c > 255 {
				c = 255
			}
			if c < 0 {
				c = 0
			}
			return c
		}

		return [3]int{s(p.Red()), s(p.Green()), s(p.Blue())}
	}

	intlen := func(i int) int {
		if i < 10 {
			return 1
		}
		if i < 100 {
			return 2
		}
		return 3
	}

	charsWrittenToLine := 0
	intsWritten := 0
	for _, p := range c.Pixels {
		for _, num := range scale(p) {
			l := intlen(num)

			lineLenAfterWrite := charsWrittenToLine + l + 1
			if lineLenAfterWrite > 70 {
				charsWrittenToLine = 0
				fmt.Fprintln(buffer)
			} else if charsWrittenToLine != 0 {
				n, _ := fmt.Fprint(buffer, " ")
				charsWrittenToLine += n
			}

			n, _ := fmt.Fprint(buffer, num)
			charsWrittenToLine += n
			intsWritten++

			if intsWritten/3 == c.Width {
				intsWritten = 0
				charsWrittenToLine = 0
				fmt.Fprintln(buffer)
			}
		}
	}

	fmt.Fprintln(buffer)
	return buffer.Bytes()
}
