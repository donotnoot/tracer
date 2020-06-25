package tracer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMatrix(t *testing.T) {
	t.Parallel()

	t.Run("Matrix equality with identical matrices (4)", func(t *testing.T) {
		t.Parallel()

		a, b := &Mat4{
			{1, 2, 3, 4},
			{2, 3, 4, 5},
			{3, 4, 5, 6},
			{4, 5, 6, 7},
		}, &Mat4{
			{1, 2, 3, 4},
			{2, 3, 4, 5},
			{3, 4, 5, 6},
			{4, 5, 6, 7},
		}

		assert.True(t, MatrixEqual(a, b, nil), "Should be equal")
		assert.True(t, MatrixEqual(a, b, &Cmp{10e-10}), "Should be equal")
	})

	t.Run("Matrix equality with identical matrices (3)", func(t *testing.T) {
		t.Parallel()

		a, b := &Mat3{
			{1, 2, 3},
			{2, 3, 4},
			{3, 4, 5},
		}, &Mat3{
			{1, 2, 3},
			{2, 3, 4},
			{3, 4, 5},
		}

		assert.True(t, MatrixEqual(a, b, nil), "Should be equal")
		assert.True(t, MatrixEqual(a, b, &Cmp{10e-10}), "Should be equal")
	})

	t.Run("Matrix equality with identical matrices (2)", func(t *testing.T) {
		t.Parallel()

		a, b := &Mat2{
			{1, 2},
			{2, 3},
		}, &Mat2{
			{1, 2},
			{2, 3},
		}

		assert.True(t, MatrixEqual(a, b, nil), "Should be equal")
		assert.True(t, MatrixEqual(a, b, &Cmp{10e-10}), "Should be equal")
	})

	t.Run("Matrix multiplication", func(t *testing.T) {
		t.Parallel()

		a, b, expected := &Mat4{
			{1, 2, 3, 4},
			{5, 6, 7, 8},
			{9, 8, 7, 6},
			{5, 4, 3, 2},
		}, &Mat4{
			{-2, 1, 2, 3},
			{3, 2, 1, -1},
			{4, 3, 6, 5},
			{1, 2, 7, 8},
		}, &Mat4{
			{20, 22, 50, 48},
			{44, 54, 114, 108},
			{40, 58, 110, 102},
			{16, 26, 46, 42},
		}

		result := MatrixMultiply(a, b)
		assert.True(t, MatrixEqual(result, expected, nil), "Should be equal to the expected result")
	})

	t.Run("Matrix tuple multiplication", func(t *testing.T) {
		t.Parallel()

		a, b, expected := &Mat4{
			{1, 2, 3, 4},
			{2, 4, 4, 2},
			{8, 6, 4, 1},
			{0, 0, 0, 1},
		},
			&Tup{1, 2, 3, 1},
			&Tup{18, 24, 33, 1}

		result := MatrixTupMultiply(a, b)
		assert.True(t, EqTup(result, expected, nil), "Should be equal to the expected result")
	})

	t.Run("Multiplying by the identity matrix returns the original matrix", func(t *testing.T) {
		t.Parallel()

		in := &Mat4{
			{1, 2, 3, 4},
			{5, 6, 7, 8},
			{9, 8, 7, 6},
			{5, 4, 3, 2},
		}

		result := MatrixMultiply(in, &IdMat4)
		assert.True(t, MatrixEqual(result, in, nil), "Should be equal to the expected result")
	})

	t.Run("Transposing a matrix", func(t *testing.T) {
		t.Parallel()

		in, out := &Mat4{
			{1, 2, 3, 4},
			{5, 6, 7, 8},
			{9, 8, 7, 6},
			{5, 4, 3, 2},
		}, &Mat4{
			{1, 5, 9, 5},
			{2, 6, 8, 4},
			{3, 7, 7, 3},
			{4, 8, 6, 2},
		}

		result := MatrixTranspose(in)
		assert.True(t, MatrixEqual(result, out, nil), "Should be equal to the expected result")
	})

	t.Run("Transposing a the identity matrix returns the identity matrix", func(t *testing.T) {
		t.Parallel()

		result := MatrixTranspose(&IdMat4)
		assert.True(t, MatrixEqual(&IdMat4, result, nil), "Should be equal to the expected result")
	})

	t.Run("Calculating the determinant of a 2x2 matrix", func(t *testing.T) {
		t.Parallel()

		result := MatrixDeterminant(&Mat2{
			{1, 5},
			{-3, 2},
		})
		assert.Equal(t, float64(17), result)
	})

	t.Run("A submatrix of a 3x3 matrix is a 2x2 matrix", func(t *testing.T) {
		t.Parallel()

		in, row, col, expected :=
			&Mat3{
				{1, 5, 0},
				{-3, 2, 7},
				{0, 6, -3},
			},
			0, 2,
			&Mat2{
				{-3, 2},
				{0, 6},
			}

		result := Submatrix(in, row, col)
		assert.True(t, MatrixEqual(result, expected, nil), "Should be equal to the expected result")
	})

	t.Run("A submatrix of a 4x4 matrix is a 3x3 matrix", func(t *testing.T) {
		t.Parallel()

		in, row, col, expected :=
			&Mat4{
				{1, 5, 0, 1},
				{-3, 2, 7, 10},
				{0, 6, -3, 5},
				{1, 7, -3, 9},
			},
			1, 2,
			&Mat3{
				{1, 5, 1},
				{0, 6, 5},
				{1, 7, 9},
			}

		result := Submatrix(in, row, col)
		assert.True(t, MatrixEqual(result, expected, nil), "Should be equal to the expected result")
	})

	t.Run("Calculating the minor of a 3x3 matrix", func(t *testing.T) {
		t.Parallel()

		in, row, col, expected := &Mat3{
			{1, 5, 1},
			{0, 6, 5},
			{1, 7, 9},
		}, 1, 2, 2

		result := MatrixMinor(in, row, col)
		assert.Equal(t, float64(expected), result)
	})

	t.Run("Calculating the cofactor of a 3x3 matrix", func(t *testing.T) {
		t.Parallel()

		mat := &Mat3{
			{3, 5, 0},
			{2, -1, -7},
			{6, -1, 5},
		}

		assert.Equal(t, float64(-12), MatrixMinor(mat, 0, 0))
		assert.Equal(t, float64(-12), MatrixCofactor(mat, 0, 0))

		assert.Equal(t, float64(25), MatrixMinor(mat, 1, 0))
		assert.Equal(t, float64(-25), MatrixCofactor(mat, 1, 0))
	})

	t.Run("Calculating the determinant of a 3x3 matrix", func(t *testing.T) {
		t.Parallel()

		mat := &Mat3{
			{1, 2, 6},
			{-5, 8, -4},
			{2, 6, 4},
		}

		assert.Equal(t, float64(56), MatrixCofactor(mat, 0, 0))
		assert.Equal(t, float64(12), MatrixCofactor(mat, 0, 1))
		assert.Equal(t, float64(-46), MatrixCofactor(mat, 0, 2))
		assert.Equal(t, float64(-196), MatrixDeterminant(mat))
	})

	t.Run("Calculating the determinant of a 4x4 matrix", func(t *testing.T) {
		t.Parallel()

		mat := &Mat4{
			{-2, -8, 3, 5},
			{-3, 1, 7, 3},
			{1, 2, -9, 6},
			{-6, 7, 7, -9},
		}

		assert.Equal(t, float64(690), MatrixCofactor(mat, 0, 0))
		assert.Equal(t, float64(447), MatrixCofactor(mat, 0, 1))
		assert.Equal(t, float64(210), MatrixCofactor(mat, 0, 2))
		assert.Equal(t, float64(51), MatrixCofactor(mat, 0, 3))
		assert.Equal(t, float64(-4071), MatrixDeterminant(mat))
	})

	t.Run("Testing an invertible matrix for invertibility", func(t *testing.T) {
		t.Parallel()

		mat := &Mat4{
			{6, 4, 4, 4},
			{5, 5, 7, 6},
			{4, -9, 3, -7},
			{9, 1, 7, -6},
		}

		assert.True(t, MatrixIsInversible(mat))
	})

	t.Run("Testing an noninvertible matrix for invertibility", func(t *testing.T) {
		t.Parallel()

		mat := &Mat4{
			{-4, 2, -2, -3},
			{9, 6, 2, 6},
			{0, -5, 1, -5},
			{0, 0, 0, 0},
		}

		assert.False(t, MatrixIsInversible(mat))
	})

	t.Run("Calculating the inverse of a matrix", func(t *testing.T) {
		t.Parallel()

		in := &Mat4{
			{-5, 2, 6, -8},
			{1, -5, 1, 8},
			{7, 7, -6, -7},
			{1, -3, 7, 4},
		}
		inDet := MatrixDeterminant(in)
		inCof1 := MatrixCofactor(in, 2, 3)
		inCof2 := MatrixCofactor(in, 3, 2)

		inv := MatrixInverse(in)

		assert.Equal(t, inCof1/inDet, inv.(*Mat4)[3][2])
		assert.Equal(t, inCof2/inDet, inv.(*Mat4)[2][3])
	})
}
