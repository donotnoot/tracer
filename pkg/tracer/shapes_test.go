package tracer

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSpheres(t *testing.T) {
	t.Parallel()

	cmp := &Cmp{10e-3}

	t.Run("A spheres default transform", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		assert.Equal(t, MatrixIdentity(4).(*Mat4), s.Transform)
	})

	t.Run("A spheres default material", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		assert.Equal(t, NewMaterial(), s.Material)
	})

	t.Run("Changing a spheres trasformation", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		s.Transform = &Mat4{}
		assert.Equal(t, &Mat4{}, s.Transform)
		assert.NotEqual(t, MatrixIdentity(4).(*Mat4), s.Transform)
	})

	t.Run("Intersecting a scaled sphere with a ray", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		s := NewSphere()
		s.Transform = ScalingMatrix(2, 2, 2)
		xs := s.Intersect(r)

		assert.NotNil(t, xs)
		assert.Equal(t, 3.0, xs[0].T)
		assert.Equal(t, 7.0, xs[1].T)
	})

	t.Run("Intersecting a translated sphere with a ray", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		s := NewSphere()
		s.Transform = TranslationMatrix(5, 0, 0)
		xs := s.Intersect(r)

		assert.Nil(t, xs)
	})

	t.Run("The normal on a sphere at a point on the x axis", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		normal := s.Normal(Point(1, 0, 0))
		assert.Equal(t, Vector(1, 0, 0), normal)
	})
	t.Run("The normal on a sphere at a point on the y axis", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		normal := s.Normal(Point(0, 1, 0))
		assert.Equal(t, Vector(0, 1, 0), normal)
	})
	t.Run("The normal on a sphere at a point on the z axis", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		normal := s.Normal(Point(0, 0, 1))
		assert.Equal(t, Vector(0, 0, 1), normal)
	})
	t.Run("The normal on a sphere at a point on a nonaxial point", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		p := math.Sqrt(3) / 3.0
		normal := s.Normal(Point(p, p, p))
		assert.Equal(t, Vector(p, p, p), normal)
	})
	t.Run("The normals should be normalised", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		p := math.Sqrt(3) / 3.0
		normal := s.Normal(Point(p, p, p))
		assert.Equal(t, Vector(p, p, p).Normalize(), normal)
	})
	t.Run("Computing the normal on a translated sphere", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		s.Transform = TranslationMatrix(0, 1, 0)

		normal := s.Normal(Point(0, 1.70711, -0.70711))

		assert.True(t, cmp.Equal(0, normal.X))
		assert.True(t, cmp.Equal(0.70711, normal.Y))
		assert.True(t, cmp.Equal(-0.70711, normal.Z))
	})
	t.Run("Computing the normal on a transformed sphere", func(t *testing.T) {
		t.Parallel()

		s := NewSphere()
		s.Transform = MatrixMultiply(ScalingMatrix(1, 0.5, 1), RotateZMatrix(math.Pi/5))

		p := math.Sqrt(2) / 2.0
		normal := s.Normal(Point(0, p, -p))

		assert.True(t, cmp.Equal(0, normal.X))
		assert.True(t, cmp.Equal(0.97014, normal.Y))
		assert.True(t, cmp.Equal(-0.24254, normal.Z))
	})
}
