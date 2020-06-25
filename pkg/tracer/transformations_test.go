package tracer

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTransformations(t *testing.T) {
	t.Parallel()

	t.Run("Multiplying by a translation matrix", func(t *testing.T) {
		t.Parallel()

		transform := TranslationMatrix(5, -3, 2)
		point := Point(-3, 4, 5)
		point = MatrixTupMultiply(transform, point)
		assert.Equal(t, float64(2), point.X)
		assert.Equal(t, float64(1), point.Y)
		assert.Equal(t, float64(7), point.Z)
	})

	t.Run("Multiplying by a inverse translation matrix", func(t *testing.T) {
		t.Parallel()

		transform := MatrixInverse(TranslationMatrix(5, -3, 2))
		point := Point(-3, 4, 5)
		point = MatrixTupMultiply(transform.(*Mat4), point)
		assert.Equal(t, float64(-8), point.X)
		assert.Equal(t, float64(7), point.Y)
		assert.Equal(t, float64(3), point.Z)
	})

	t.Run("Translation does not affect vectors", func(t *testing.T) {
		t.Parallel()

		transform := TranslationMatrix(5, -2, 2)
		vector := Vector(-3, 4, 5)
		vector = MatrixTupMultiply(transform, vector)
		assert.Equal(t, float64(-3), vector.X)
		assert.Equal(t, float64(4), vector.Y)
		assert.Equal(t, float64(5), vector.Z)
	})

	t.Run("A scaling matrix applied to a point", func(t *testing.T) {
		t.Parallel()

		transform := ScalingMatrix(2, 3, 4)
		point := Point(-4, 6, 8)
		point = MatrixTupMultiply(transform, point)
		assert.Equal(t, float64(-8), point.X)
		assert.Equal(t, float64(18), point.Y)
		assert.Equal(t, float64(32), point.Z)
	})

	t.Run("A scaling matrix applied to a vector", func(t *testing.T) {
		t.Parallel()

		transform := ScalingMatrix(2, 3, 4)
		vector := Vector(-4, 6, 8)
		vector = MatrixTupMultiply(transform, vector)
		assert.Equal(t, float64(-8), vector.X)
		assert.Equal(t, float64(18), vector.Y)
		assert.Equal(t, float64(32), vector.Z)
	})

	t.Run("An inverted scaling matrix applied to a tuple", func(t *testing.T) {
		t.Parallel()

		transform := MatrixInverse(ScalingMatrix(2, 3, 4))
		vector := Vector(-4, 6, 8)
		vector = MatrixTupMultiply(transform.(*Mat4), vector)
		assert.Equal(t, float64(-2), vector.X)
		assert.Equal(t, float64(2), vector.Y)
		assert.Equal(t, float64(2), vector.Z)
	})

	t.Run("Reflection is scaling by a negative value", func(t *testing.T) {
		t.Parallel()

		transform := ScalingMatrix(-1, 1, 1)
		vector := Point(2, 3, 4)
		vector = MatrixTupMultiply(transform, vector)
		assert.Equal(t, float64(-2), vector.X)
		assert.Equal(t, float64(3), vector.Y)
		assert.Equal(t, float64(4), vector.Z)
	})

	t.Run("Rotating a point around the X axis", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-5}

		point := Point(0, 1, 0)

		eighth := RotateXMatrix(math.Pi / 4)
		rEighth := MatrixTupMultiply(eighth, point)
		assert.True(t, cmp.Equal(float64(0), rEighth.X))
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, rEighth.Y))
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, rEighth.Z))

		fourth := RotateXMatrix(math.Pi / 2)
		rFourth := MatrixTupMultiply(fourth, point)
		assert.True(t, cmp.Equal(float64(0), rFourth.X))
		assert.True(t, cmp.Equal(float64(0), rFourth.Y))
		assert.True(t, cmp.Equal(float64(1), rFourth.Z))
	})

	t.Run("Rotating a point around the X axis inverting the matrix", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-4}

		point := Point(0, 1, 0)

		eighth := MatrixInverse(RotateXMatrix(math.Pi / 4))
		rEighth := MatrixTupMultiply(eighth.(*Mat4), point)
		assert.True(t, cmp.Equal(float64(0), rEighth.X))
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, rEighth.Y))
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, -rEighth.Z))

		fourth := MatrixInverse(RotateXMatrix(math.Pi / 2))
		rFourth := MatrixTupMultiply(fourth.(*Mat4), point)
		assert.True(t, cmp.Equal(float64(0), rFourth.X))
		assert.True(t, cmp.Equal(float64(0), rFourth.Y))
		assert.True(t, cmp.Equal(float64(1), -rFourth.Z))
	})

	t.Run("Rotating a point around the Y axis", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-5}

		point := Point(0, 0, 1)

		eighth := RotateYMatrix(math.Pi / 4)
		rEighth := MatrixTupMultiply(eighth, point)
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, rEighth.X))
		assert.True(t, cmp.Equal(float64(0), rEighth.Y))
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, rEighth.Z))

		fourth := RotateYMatrix(math.Pi / 2)
		rFourth := MatrixTupMultiply(fourth, point)
		assert.True(t, cmp.Equal(float64(1), rFourth.X))
		assert.True(t, cmp.Equal(float64(0), rFourth.Y))
		assert.True(t, cmp.Equal(float64(0), rFourth.Z))
	})

	t.Run("Rotating a point around the Y axis", func(t *testing.T) {
		t.Parallel()
		cmp := &Cmp{10e-5}

		point := Point(0, 1, 0)

		eighth := RotateZMatrix(math.Pi / 4)
		rEighth := MatrixTupMultiply(eighth, point)
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, -rEighth.X))
		assert.True(t, cmp.Equal(math.Sqrt(2)/2, rEighth.Y))
		assert.True(t, cmp.Equal(float64(0), rEighth.Z))

		fourth := RotateZMatrix(math.Pi / 2)
		rFourth := MatrixTupMultiply(fourth, point)
		assert.True(t, cmp.Equal(float64(-1), rFourth.X))
		assert.True(t, cmp.Equal(float64(0), rFourth.Y))
		assert.True(t, cmp.Equal(float64(0), rFourth.Z))
	})

	t.Run("Shearing transforms", func(t *testing.T) {
		t.Parallel()

		tests := []struct {
			name    string
			xy, xz  float64
			yx, yz  float64
			zx, zy  float64
			in, out *Tup
		}{
			{
				xy: 1.0, xz: 0.0,
				yx: 0.0, yz: 0.0,
				zx: 0.0, zy: 0.0,
				in:   Point(2, 3, 4),
				out:  Point(5, 3, 4),
				name: "Shearing moves X in proportion to Y",
			},
			{
				xy: 0.0, xz: 1.0,
				yx: 0.0, yz: 0.0,
				zx: 0.0, zy: 0.0,
				in:   Point(2, 3, 4),
				out:  Point(6, 3, 4),
				name: "Shearing moves X in proportion to Z",
			},
			{
				xy: 0.0, xz: 0.0,
				yx: 1.0, yz: 0.0,
				zx: 0.0, zy: 0.0,
				in:   Point(2, 3, 4),
				out:  Point(2, 5, 4),
				name: "Shearing moves Y in proportion to X",
			},
			{
				xy: 0.0, xz: 0.0,
				yx: 0.0, yz: 1.0,
				zx: 0.0, zy: 0.0,
				in:   Point(2, 3, 4),
				out:  Point(2, 7, 4),
				name: "Shearing moves Y in proportion to Z",
			},
			{
				xy: 0.0, xz: 0.0,
				yx: 0.0, yz: 0.0,
				zx: 1.0, zy: 0.0,
				in:   Point(2, 3, 4),
				out:  Point(2, 3, 6),
				name: "Shearing moves Z in proportion to X",
			},
			{
				xy: 0.0, xz: 0.0,
				yx: 0.0, yz: 0.0,
				zx: 0.0, zy: 1.0,
				in:   Point(2, 3, 4),
				out:  Point(2, 3, 7),
				name: "Shearing moves Z in proportion to Y",
			},
		}

		for _, test := range tests {
			t.Run(test.name, func(t *testing.T) {
				t.Parallel()

				mat := ShearingMatrix(
					test.xy, test.xz,
					test.yx, test.yz,
					test.zx, test.zy)
				startingPoint := Point(test.in.X, test.in.Y, test.in.Z)
				newPoint := MatrixTupMultiply(mat, startingPoint)

				assert.Equal(t, test.out.X, newPoint.X)
				assert.Equal(t, test.out.Y, newPoint.Y)
				assert.Equal(t, test.out.Z, newPoint.Z)
			})
		}
	})

	// Skipped tests
	// "Individual transformations are applied in sequence" p. 54
	// "Chained transformations must be applied in reverse order" p. 54
}
