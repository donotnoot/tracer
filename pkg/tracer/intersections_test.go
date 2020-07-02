package tracer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIntersections(t *testing.T) {
	t.Parallel()

	t.Run("An intersection encapsulates T and Object", func(t *testing.T) {
		s := &Sphere{}
		i := &Intersection{3.5, s}

		assert.Equal(t, 3.5, i.T)
		assert.Equal(t, s, i.Object)
	})

	t.Run("Aggregating intersections", func(t *testing.T) {
		s := &Sphere{}
		i1 := &Intersection{3.5, s}
		i2 := &Intersection{4.5, s}

		xs := Intersections{i1, i2}

		assert.Equal(t, 2, len(xs))
		assert.Equal(t, i1, xs[0])
		assert.Equal(t, i2, xs[1])
	})

	t.Run("Getting the hit", func(t *testing.T) {
		t.Parallel()

		tests := []struct {
			name string
			ixs  Intersections
			hit  int
		}{
			{
				name: "When all ixs have +t",
				ixs: Intersections{
					&Intersection{T: 1.0, Object: nil},
					&Intersection{T: 2.0, Object: nil},
				},
				hit: 0,
			},
			{
				name: "When some are + and some are -",
				ixs: Intersections{
					&Intersection{T: -1.0, Object: nil},
					&Intersection{T: 2.0, Object: nil},
				},
				hit: 1,
			},
			{
				name: "When all the ixs have -t",
				ixs: Intersections{
					&Intersection{T: -1.0, Object: nil},
					&Intersection{T: -2.0, Object: nil},
				},
				hit: -1, // none
			},
			{
				name: "The hit is always the lowest nonnegative ix",
				ixs: Intersections{
					&Intersection{T: -1.0, Object: nil},
					&Intersection{T: 1.0, Object: nil},
					&Intersection{T: -2.0, Object: nil},
					&Intersection{T: 2.0, Object: nil},
				},
				hit: 1,
			},
		}

		for _, test := range tests {
			t.Run(test.name, func(t *testing.T) {
				hit, idx, ok := test.ixs.Hit()
				if test.hit < 0 {
					assert.False(t, ok)
					return
				}
				assert.Equal(t, test.ixs[test.hit].T, hit)
				assert.Equal(t, test.hit, idx)
			})
		}
	})

	t.Run("precomputing the state of an intersection", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		s := NewSphere()
		i := &Intersection{4.0, s}

		c := i.PrepareComputations(r)

		assert.Equal(t, i.T, c.T)
		assert.Equal(t, i.Object, c.Object)
		assert.Equal(t, Point(0, 0, -1), c.Point)
		assert.Equal(t, Vector(0, 0, -1), c.Eye)
		assert.Equal(t, Vector(0, 0, -1), c.Normal)
	})

	t.Run("the hit, when an intersection occurs on the outside", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		s := NewSphere()
		i := &Intersection{4.0, s}
		c := i.PrepareComputations(r)

		assert.False(t, c.Inside)
	})

	t.Run("the hit, when an intersection occurs on the inside", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, 0), Vector(0, 0, 1)}
		s := NewSphere()
		i := &Intersection{1, s}
		c := i.PrepareComputations(r)

		assert.True(t, c.Inside)
		assert.Equal(t, Point(0, 0, 1), c.Point)
		assert.Equal(t, Vector(0, 0, -1), c.Eye)
		assert.Equal(t, Vector(0, 0, -1), c.Normal)
	})

	t.Run("the hit should offset the point", func(t *testing.T) {
		t.Parallel()

		r := &Ray{Point(0, 0, -5), Vector(0, 0, 1)}
		shape := NewSphere()
		shape.Transform = TranslationMatrix(0, 0, 1)
		i := &Intersection{5, shape}
		comps := i.PrepareComputations(r)

		assert.True(t, comps.OverPoint.Z < -10e-5/2)
		assert.True(t, comps.Point.Z > comps.OverPoint.Z)
	})
}
