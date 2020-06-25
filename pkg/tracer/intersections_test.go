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
				hit, ok := test.ixs.Hit()
				if test.hit < 0 {
					assert.False(t, ok)
					return
				}
				assert.Equal(t, test.ixs[test.hit].T, hit)
			})
		}
	})
}
