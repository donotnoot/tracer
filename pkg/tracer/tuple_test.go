package tracer

import (
	"fmt"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTuple(t *testing.T) {
	t.Parallel()

	t.Run("A tuple with W=1 is a point", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{W: 1}
		assert.True(t, tup.IsPoint())
		assert.False(t, tup.IsVector(), "it's should also not be a vector")
	})

	t.Run("A tuple with W=0 is a vector", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{W: 0}
		assert.True(t, tup.IsVector())
		assert.False(t, tup.IsPoint(), "it should also not be a point")
	})

	t.Run("Point() creates new *Tup of type Point", func(t *testing.T) {
		t.Parallel()

		tup := Point(1, 2, 3)
		assert.Equal(t, float64(1), tup.X)
		assert.Equal(t, float64(2), tup.Y)
		assert.Equal(t, float64(3), tup.Z)
		assert.True(t, tup.IsPoint())
	})

	t.Run("Vector() creates new *Tup of type Vector", func(t *testing.T) {
		t.Parallel()

		tup := Vector(1, 2, 3)
		assert.Equal(t, float64(1), tup.X)
		assert.Equal(t, float64(2), tup.Y)
		assert.Equal(t, float64(3), tup.Z)
		assert.True(t, tup.IsVector())
	})

	t.Run("Adding two tuples", func(t *testing.T) {
		t.Parallel()

		a, b := &Tup{3, -2, 5, 1}, &Tup{-2, 3, 1, 0}
		c := AddTup(a, b)
		assert.Equal(t, float64(1), c.X)
		assert.Equal(t, float64(1), c.Y)
		assert.Equal(t, float64(6), c.Z)
		assert.Equal(t, float64(1), c.W)
	})

	t.Run("Adding to a tuple in place", func(t *testing.T) {
		t.Parallel()

		a, b := &Tup{1, 2, 3, 4}, &Tup{5, 6, 7, 8}
		a.Add(b)
		assert.Equal(t, float64(6), a.X)
		assert.Equal(t, float64(8), a.Y)
		assert.Equal(t, float64(10), a.Z)
		assert.Equal(t, float64(12), a.W)
	})

	t.Run("Substracting from a tuple in place", func(t *testing.T) {
		t.Parallel()

		a, b := &Tup{1, 2, 3, 4}, &Tup{5, 6, 7, 8}
		a.Subtract(b)
		assert.Equal(t, float64(-4), a.X)
		assert.Equal(t, float64(-4), a.Y)
		assert.Equal(t, float64(-4), a.Z)
		assert.Equal(t, float64(-4), a.W)
	})

	t.Run("Subtracting two points", func(t *testing.T) {
		t.Parallel()

		a, b := Point(3, 2, 1), Point(5, 6, 7)
		c := SubTup(a, b)
		assert.Equal(t, float64(-2), c.X)
		assert.Equal(t, float64(-4), c.Y)
		assert.Equal(t, float64(-6), c.Z)
		assert.True(t, c.IsVector(), "should yield a vector")
	})

	t.Run("Subtracting a vector from a point", func(t *testing.T) {
		t.Parallel()

		p, v := Point(3, 2, 1), Vector(5, 6, 7)
		c := SubTup(p, v)
		assert.Equal(t, float64(-2), c.X)
		assert.Equal(t, float64(-4), c.Y)
		assert.Equal(t, float64(-6), c.Z)
		assert.True(t, c.IsPoint(), "should yield a point")
	})

	t.Run("Subtracting two vectors", func(t *testing.T) {
		t.Parallel()

		a, b := Vector(3, 2, 1), Vector(5, 6, 7)
		c := SubTup(a, b)
		assert.Equal(t, float64(-2), c.X)
		assert.Equal(t, float64(-4), c.Y)
		assert.Equal(t, float64(-6), c.Z)
		assert.True(t, c.IsVector(), "should yield a vector")
	})

	t.Run("Negating a tuple", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{1, 2, 3, 4}
		negated := NegTup(tup)
		assert.Equal(t, float64(-1), negated.X)
		assert.Equal(t, float64(-2), negated.Y)
		assert.Equal(t, float64(-3), negated.Z)
		assert.Equal(t, float64(-4), negated.W)
	})

	t.Run("Negating a tuple in place", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{1, 2, 3, 4}
		tup.Negate()
		assert.Equal(t, float64(-1), tup.X)
		assert.Equal(t, float64(-2), tup.Y)
		assert.Equal(t, float64(-3), tup.Z)
		assert.Equal(t, float64(-4), tup.W)
	})

	t.Run("Multiplying a tuple by a scalar", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{1, 2, 3, 4}
		mul := ScaleTup(tup, 2)
		assert.Equal(t, float64(2), mul.X)
		assert.Equal(t, float64(4), mul.Y)
		assert.Equal(t, float64(6), mul.Z)
		assert.Equal(t, float64(8), mul.W)
	})

	t.Run("Multiplying a tuple by a scalar in place", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{1, 2, 3, 4}
		tup.Scale(2)
		assert.Equal(t, float64(2), tup.X)
		assert.Equal(t, float64(4), tup.Y)
		assert.Equal(t, float64(6), tup.Z)
		assert.Equal(t, float64(8), tup.W)
	})

	t.Run("Multiplying a tuple by a tuple", func(t *testing.T) {
		t.Parallel()

		mul := MulTup(&Tup{1, 2, 3, 4}, &Tup{1, 2, 3, 4})
		assert.Equal(t, float64(1), mul.X)
		assert.Equal(t, float64(4), mul.Y)
		assert.Equal(t, float64(9), mul.Z)
		assert.Equal(t, float64(16), mul.W)
	})

	t.Run("Multiplying a tuple by a tuple in place", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{1, 2, 3, 4}
		tup.Multiply(&Tup{1, 2, 3, 4})
		assert.Equal(t, float64(1), tup.X)
		assert.Equal(t, float64(4), tup.Y)
		assert.Equal(t, float64(9), tup.Z)
		assert.Equal(t, float64(16), tup.W)
	})

	t.Run("Dividing a tuple", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{1, 2, 3, 4}
		mul := DivTup(tup, 2)
		assert.Equal(t, float64(.5), mul.X)
		assert.Equal(t, float64(1), mul.Y)
		assert.Equal(t, float64(1.5), mul.Z)
		assert.Equal(t, float64(2), mul.W)
	})

	t.Run("Dividing a tuple in place", func(t *testing.T) {
		t.Parallel()

		tup := &Tup{1, 2, 3, 4}
		tup.Divide(2)
		assert.Equal(t, float64(.5), tup.X)
		assert.Equal(t, float64(1), tup.Y)
		assert.Equal(t, float64(1.5), tup.Z)
		assert.Equal(t, float64(2), tup.W)
	})

	t.Run("Computing the magnitude of a vector", func(t *testing.T) {
		t.Parallel()
		c := &Cmp{10e-5}

		tests := []struct {
			t *Tup
			m float64
		}{
			{t: Vector(1, 0, 0), m: 1.0},
			{t: Vector(0, 1, 0), m: 1.0},
			{t: Vector(0, 0, 1), m: 1.0},
			{t: Vector(1, 2, 3), m: math.Sqrt(14)},
			{t: Vector(-1, -2, -3), m: math.Sqrt(14)},
		}

		for _, test := range tests {
			t.Run(fmt.Sprint(test.t), func(t *testing.T) {
				t.Parallel()

				if m := test.t.Magnitude(); !c.Equal(m, test.m) {
					t.Errorf("%#v: expected: %v, got: %v", test.t, test.m, m)
				}
			})
		}
	})

	t.Run("Normalizing vectors", func(t *testing.T) {
		t.Parallel()
		c := &Cmp{10e-5}

		tests := []struct {
			in  *Tup
			out *Tup
		}{
			{in: Vector(4, 0, 0), out: Vector(1, 0, 0)},
			{in: Vector(1, 2, 3), out: Vector(1/math.Sqrt(14), 2/math.Sqrt(14), 3/math.Sqrt(14))},
		}

		for _, test := range tests {
			t.Run(fmt.Sprint(test.in), func(t *testing.T) {
				t.Parallel()

				if n := test.in.Normalize(); !n.Equal(n, c) {
					t.Errorf("%#v: expected: %#v, got: %#v", test.in, test.out, n)
				}
			})
		}
	})

	t.Run("The dot product of two tuples", func(t *testing.T) {
		t.Parallel()

		a, b := Vector(1, 2, 3), Vector(2, 3, 4)
		dot := DotTup(a, b)
		assert.Equal(t, float64(20), dot)
	})

	t.Run("The cross product of two vectors", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-5}

		a, b := Vector(1, 2, 3), Vector(2, 3, 4)
		dotab, dotba := CrossVec(a, b), CrossVec(b, a)

		assert.True(t, dotab.Equal(Vector(-1, 2, -1), cmp))
		assert.True(t, dotba.Equal(Vector(1, -2, 1), cmp))
	})

	t.Run("Colors are (red green blue) tuples", func(t *testing.T) {
		t.Parallel()

		color := Color(1, 2, 3)
		assert.Equal(t, float64(1), color.Red())
		assert.Equal(t, float64(2), color.Green())
		assert.Equal(t, float64(3), color.Blue())
	})

	// Tests for add, sub, mul colors by scalars skipped, they're the same as tuples.
	// Test for multiply color skipped, it's just a normal Hadarmard product.

	t.Run("Reflecting a vector off a 45deg normal", func(t *testing.T) {
		t.Parallel()

		vector := Vector(1, -1, 0)
		normal := Vector(0, 1, 0)

		reflection := vector.Reflect(normal)

		assert.Equal(t, Vector(1, 1, 0), reflection)
	})

	t.Run("Reflecting a vector off a slanted surface", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-5}

		vector := Vector(0, -1, 0)
		p := math.Sqrt(2) / 2.0
		normal := Vector(p, p, 0)

		reflection := vector.Reflect(normal)

		assert.True(t, cmp.Equal(1, reflection.X))
		assert.True(t, cmp.Equal(0, reflection.Y))
		assert.True(t, cmp.Equal(0, reflection.Z))
	})
}
