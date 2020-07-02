package tracer

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMaterial(t *testing.T) {
	t.Parallel()

	t.Run("the default material", func(t *testing.T) {
		t.Parallel()

		m := NewMaterial()

		assert.Equal(t, Color(1, 1, 1), m.Color)
		assert.Equal(t, .1, m.Ambient)
		assert.Equal(t, .9, m.Diffuse)
		assert.Equal(t, .9, m.Specular)
		assert.Equal(t, 200.0, m.Shininess)
	})

	mat := NewMaterial()
	pos := Point(0, 0, 0)
	cmp := &Cmp{10e-5}

	t.Run("lighting with the eye between the light and the surface", func(t *testing.T) {
		t.Parallel()

		eyev := Vector(0, 0, -1)
		normalv := Vector(0, 0, -1)
		light := &PointLight{Point(0, 0, -10), Color(1, 1, 1)}
		result := mat.Lighting(light, pos, eyev, normalv, false)

		assert.Equal(t, Color(1.9, 1.9, 1.9), result)
	})

	t.Run("lighting with the eye between the light and the surface", func(t *testing.T) {
		t.Parallel()

		p := math.Sqrt(2) / 2.0
		eyev := Vector(0, p, p)
		normalv := Vector(0, 0, -1)
		light := &PointLight{Point(0, 0, -10), Color(1, 1, 1)}
		result := mat.Lighting(light, pos, eyev, normalv, false)

		assert.Equal(t, Color(1.0, 1.0, 1.0), result)
	})

	t.Run("lighting with the eye opposite surface, light at 45deg", func(t *testing.T) {
		t.Parallel()

		p := math.Sqrt(2) / 2.0

		eyev := Vector(0, 0, -1)
		normalv := Vector(0, 0, -1)
		light := &PointLight{Point(0, 10, -10), Color(1, 1, 1)}
		result := mat.Lighting(light, pos, eyev, normalv, false)

		r := .1 + p*.9
		assert.True(t, cmp.Equal(r, result.Red()))
		assert.True(t, cmp.Equal(r, result.Green()))
		assert.True(t, cmp.Equal(r, result.Blue()))
	})

	t.Run("lighting with the eye in the path of the reflection vector", func(t *testing.T) {
		t.Parallel()

		p := math.Sqrt(2) / 2.0

		eyev := Vector(0, -p, -p)
		normalv := Vector(0, 0, -1)
		light := &PointLight{Point(0, 10, -10), Color(1, 1, 1)}
		result := mat.Lighting(light, pos, eyev, normalv, false)

		r := .1 + .9*p + .9
		assert.Equal(t, Color(r, r, r), result)
	})

	t.Run("lighting with the light behind the surface", func(t *testing.T) {
		t.Parallel()

		eyev := Vector(0, 0, -1)
		normalv := Vector(0, 0, -1)
		light := &PointLight{Point(0, 0, 10), Color(1, 1, 1)}
		result := mat.Lighting(light, pos, eyev, normalv, false)

		assert.Equal(t, Color(.1, .1, .1), result)
	})

	t.Run("lighting with the surface in shadow", func(t *testing.T) {
		t.Parallel()

		eyev := Vector(0, 0, -1)
		normalv := Vector(0, 0, -1)
		light := &PointLight{Point(0, 0, -10), Color(1, 1, 1)}
		result := mat.Lighting(light, pos, eyev, normalv, true)

		assert.Equal(t, Color(.1, .1, .1), result)
	})
}
