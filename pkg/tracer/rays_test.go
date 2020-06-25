package tracer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRays(t *testing.T) {
	t.Parallel()

	t.Run("creating and querying a ray", func(t *testing.T) {
		t.Parallel()

		origin := Point(1, 2, 3)
		direction := Vector(4, 5, 6)

		r := &Ray{origin, direction}
		assert.Equal(t, r.Origin, origin)
		assert.Equal(t, r.Direction, direction)
	})

	t.Run("Computing a point from a distance", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(2, 3, 4), Vector(1, 0, 0)}

		assert.Equal(t, r.Position(0), Point(2, 3, 4))
		assert.Equal(t, r.Position(1), Point(3, 3, 4))
		assert.Equal(t, r.Position(-1), Point(1, 3, 4))
		assert.Equal(t, r.Position(2.5), Point(4.5, 3, 4))
	})

	t.Run("A ray intersects a sphere at two points", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		s := NewSphere()
		is := s.Intersect(r)

		assert.NotNil(t, is)
		assert.Equal(t, 2, len(is))
		assert.Equal(t, 4.0, is[0].T)
		assert.Equal(t, 6.0, is[1].T)
	})

	t.Run("A ray intersects a sphere at a tangent", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 1, -5), Vector(0, 0, 1)}
		s := NewSphere()
		is := s.Intersect(r)

		assert.NotNil(t, is)
		assert.Equal(t, 2, len(is))
		assert.Equal(t, 5.0, is[0].T)
		assert.Equal(t, 5.0, is[1].T)
	})

	t.Run("A ray misses a sphere", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 2, -5), Vector(0, 0, 1)}
		s := NewSphere()
		is := s.Intersect(r)

		assert.Nil(t, is)
	})

	t.Run("A ray originates inside a sphere", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, 0), Vector(0, 0, 1)}
		s := NewSphere()
		is := s.Intersect(r)

		assert.NotNil(t, is)
		assert.Equal(t, 2, len(is))
		assert.Equal(t, -1.0, is[0].T)
		assert.Equal(t, 1.0, is[1].T)
	})

	t.Run("A sphere is behind a ray", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, 5), Vector(0, 0, 1)}
		s := NewSphere()
		is := s.Intersect(r)

		assert.NotNil(t, is)
		assert.Equal(t, 2, len(is))
		assert.Equal(t, -6.0, is[0].T)
		assert.Equal(t, -4.0, is[1].T)
	})

	t.Run("Intersect sets the object on the intersection", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		s := NewSphere()
		is := s.Intersect(r)

		assert.NotNil(t, is)
		assert.Equal(t, 2, len(is))
		assert.Equal(t, s, is[0].Object)
		assert.Equal(t, s, is[1].Object)
	})

	t.Run("Translating a ray", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(1, 2, 3), Vector(0, 1, 0)}
		m := TranslationMatrix(3, 4, 5)
		transformedRay := TransformRay(r, m)

		assert.Equal(t, transformedRay.Origin, Point(4, 6, 8))
		assert.Equal(t, transformedRay.Direction, Vector(0, 1, 0))
	})

	t.Run("Scaling a ray", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(1, 2, 3), Vector(0, 1, 0)}
		m := ScalingMatrix(2, 3, 4)
		transformedRay := TransformRay(r, m)

		assert.Equal(t, transformedRay.Origin, Point(2, 6, 12))
		assert.Equal(t, transformedRay.Direction, Vector(0, 3, 0))
	})
}
