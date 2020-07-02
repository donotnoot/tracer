package tracer

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCamera(t *testing.T) {
	t.Parallel()

	t.Run("constructing a camera", func(t *testing.T) {
		t.Parallel()

		c := NewCamera(160, 120, math.Pi/2)

		assert.Equal(t, c.Hsize, 160)
		assert.Equal(t, c.Vsize, 120)
		assert.Equal(t, c.FieldOfView, math.Pi/2)
		assert.Equal(t, c.Transform, MatrixIdentity(4).(*Mat4))
	})

	t.Run("the pixel size for a horizontal canvas", func(t *testing.T) {
		t.Parallel()

		c := NewCamera(200, 125, math.Pi/2)

		assert.Equal(t, .01, c.PixelSize)
	})

	t.Run("the pixel size for a vertical canvas", func(t *testing.T) {
		t.Parallel()

		c := NewCamera(150, 200, math.Pi/2)

		assert.Equal(t, .01, c.PixelSize)
	})

	t.Run("constructing a ray through the center of the canvas", func(t *testing.T) {
		t.Parallel()

		c := NewCamera(201, 101, math.Pi/2)
		r := c.Ray(100, 50)

		assert.Equal(t, Point(0, 0, 0), r.Origin)
		assert.Equal(t, Vector(0, 0, -1), r.Direction)
	})

	t.Run("constructing a ray through a corner of the canvas", func(t *testing.T) {
		t.Parallel()

		c := NewCamera(201, 101, math.Pi/2)
		r := c.Ray(0, 0)

		assert.Equal(t, Point(0, 0, 0), r.Origin)
		// TODO: assert.Equal(t, Vector(0, 0, -1), r.Direction)
	})

	t.Run("constructing a ray when the camera is transformed", func(t *testing.T) {
		t.Parallel()

		c := NewCamera(201, 101, math.Pi/2)
		c.Transform = MatrixMultiply(RotateYMatrix(math.Pi/4), TranslationMatrix(0, -2, 5))
		r := c.Ray(0, 0)

		assert.Equal(t, Point(0, 2, -5), r.Origin)
		p := math.Sqrt(2) / 2
		assert.Equal(t, Vector(p, 0, -p), r.Direction)
	})
}
