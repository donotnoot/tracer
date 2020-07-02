package tracer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestWorld(t *testing.T) {
	t.Parallel()

	t.Run("creating a world", func(t *testing.T) {
		t.Parallel()

		w := &World{}

		assert.Zero(t, w.Objects)
		assert.Nil(t, w.Light)
	})

	t.Run("the default world", func(t *testing.T) {
		t.Parallel()

		w := NewWorld()

		assert.Equal(t, &PointLight{Point(-10, 10, -10), Color(1, 1, 1)}, w.Light)
		assert.Equal(t, 2, len(w.Objects))
		assert.Equal(t, 1.0, w.Objects[0].(*Sphere).Material.Color.Green())
		assert.Equal(t, 0.7, w.Objects[0].(*Sphere).Material.Diffuse)
		assert.Equal(t, 0.2, w.Objects[0].(*Sphere).Material.Specular)
		assert.Equal(t, ScalingMatrix(.5, .5, .5), w.Objects[1].(*Sphere).Transform)
	})

	t.Run("Intersect the world with a ray", func(t *testing.T) {
		t.Parallel()

		w := NewWorld()
		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}

		intersections := w.Intersect(r)

		assert.Equal(t, 4, len(intersections))
		assert.Equal(t, 4.0, intersections[0].T)
		assert.Equal(t, 4.5, intersections[1].T)
		assert.Equal(t, 5.5, intersections[2].T)
		assert.Equal(t, 6.0, intersections[3].T)
	})

	t.Run("shading an intersection", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-1}

		w := NewWorld()
		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		i := w.Intersect(r)
		c := i[0].PrepareComputations(r)
		h := w.ShadeHit(c)

		assert.True(t, cmp.Equal(h.Red(), .37))
		assert.True(t, cmp.Equal(h.Green(), .47))
		assert.True(t, cmp.Equal(h.Blue(), .28))
	})

	t.Run("shading an intersection from the inside", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-1}

		w := NewWorld()
		w.Light = &PointLight{Point(0, .25, 0), Color(1, 1, 1)}
		r := &Ray{Point(0, 0, 0), Vector(0, 0, 1)}
		i := w.Intersect(r)
		c := i[2].PrepareComputations(r)
		h := w.ShadeHit(c)

		assert.True(t, cmp.Equal(h.Red(), .9))
		assert.True(t, cmp.Equal(h.Green(), .9))
		assert.True(t, cmp.Equal(h.Blue(), .9))
	})

	t.Run("the color when a ray misses", func(t *testing.T) {
		t.Parallel()

		w := NewWorld()
		r := &Ray{Point(0, 0, -5), Vector(0, 1, 0)}
		c := w.ColorAt(r)

		assert.Equal(t, Color(0, 0, 0), c)
	})

	t.Run("the color when a ray hits", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-1}

		w := NewWorld()
		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		c := w.ColorAt(r)

		assert.True(t, cmp.Equal(c.Red(), .9))
		assert.True(t, cmp.Equal(c.Green(), .9))
		assert.True(t, cmp.Equal(c.Blue(), .9))
	})

	t.Run("the color with an intersection behind the ray", func(t *testing.T) {
		t.Parallel()

		w := NewWorld()
		w.Objects[0].GetMaterial().Ambient = 1
		w.Objects[1].GetMaterial().Ambient = 1
		r := &Ray{Point(0, 0, .75), Vector(0, 0, -1)}
		c := w.ColorAt(r)

		assert.Equal(t, w.Objects[1].GetMaterial().Color, c)
	})
}
