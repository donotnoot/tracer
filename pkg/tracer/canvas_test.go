package tracer

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCanvas(t *testing.T) {
	t.Parallel()

	t.Run("Creating a canvas", func(t *testing.T) {
		t.Parallel()

		c := NewCanvas(10, 20)
		assert.Equal(t, 10, c.Width)
		assert.Equal(t, 20, c.Height)
		for _, pixel := range c.Pixels {
			assert.True(t, Color(0, 0, 0).Equal(pixel, &Cmp{10e-5}), "all the pixels should be black")
		}
	})

	t.Run("Writing a pixel to a canvas", func(t *testing.T) {
		t.Parallel()

		c := NewCanvas(10, 20)
		c.WritePixel(5, 5, Color(1, 0, 0))

		assert.True(t, Color(1, 0, 0).Equal(c.PixelAt(5, 5), &Cmp{10e-5}), "the pixel must be red")
	})

	t.Run("Constructing the PPM header", func(t *testing.T) {
		t.Parallel()

		c := NewCanvas(5, 3)
		ppm := string(c.PPM())
		lines1to3 := strings.Split(ppm, "\n")[:3]

		assert.Equal(t, "P3", lines1to3[0])
		assert.Equal(t, "5 3", lines1to3[1])
		assert.Equal(t, "255", lines1to3[2])
	})

	t.Run("Constructing the PPM pixel data", func(t *testing.T) {
		t.Parallel()

		c := NewCanvas(5, 3)

		c.WritePixel(0, 0, Color(1.5, 0, 0))
		c.WritePixel(2, 1, Color(0, .5, 0))
		c.WritePixel(4, 2, Color(0, 0, 1))

		ppm := string(c.PPM())
		lines3onwards := strings.Split(ppm, "\n")[3:]

		assert.Equal(t, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", lines3onwards[0])
		assert.Equal(t, "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", lines3onwards[1])
		assert.Equal(t, "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", lines3onwards[2])
	})

	t.Run("Making sure the PPM has no lines over 70 characters", func(t *testing.T) {
		t.Parallel()

		c := NewCanvas(10, 2)
		for i := range c.Pixels {
			c.Pixels[i] = Color(1, 0.8, 0.6)
		}

		ppm := string(c.PPM())
		lines3onwards := strings.Split(ppm, "\n")[3:]

		// First row of pixels split into two lines
		assert.Equal(t, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", lines3onwards[0])
		assert.Equal(t, "153 255 204 153 255 204 153 255 204 153 255 204 153", lines3onwards[1])

		// Second row of...
		assert.Equal(t, "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204", lines3onwards[2])
		assert.Equal(t, "153 255 204 153 255 204 153 255 204 153 255 204 153", lines3onwards[3])
	})

	t.Run("PPM ends with a newline", func(t *testing.T) {
		t.Parallel()

		c := NewCanvas(10, 2)

		ppm := string(c.PPM())
		assert.Equal(t, uint8('\n'), ppm[len(ppm)-1])
	})
}
