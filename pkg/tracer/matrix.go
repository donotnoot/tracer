package tracer

import "fmt"

type Matrix interface {
	MatrixSize() int
	String() string
}

type (
	Mat4 [4][4]float64
	Mat3 [3][3]float64
	Mat2 [2][2]float64
)

func (m *Mat4) MatrixSize() int { return 4 }
func (m *Mat3) MatrixSize() int { return 3 }
func (m *Mat2) MatrixSize() int { return 2 }
func (m *Mat4) String() string {
	s := ""
	for row := 0; row < 4; row++ {
		for col := 0; col < 4; col++ {
			s += fmt.Sprintf("%+6.2f", m[row][col])
			if col != 3 {
				s += " "
			}
		}
		if row != 3 {
			s += "\n"
		}
	}
	return s
}

func (m *Mat3) String() string {
	s := ""
	for row := 0; row < 3; row++ {
		for col := 0; col < 3; col++ {
			s += fmt.Sprintf("%.3f", m[row][col])
			if col != 2 {
				s += " "
			}
		}
		if row != 2 {
			s += "\n"
		}
	}
	return s
}

func (m *Mat2) String() string {
	s := ""
	for row := 0; row < 2; row++ {
		for col := 0; col < 2; col++ {
			s += fmt.Sprintf("%.3f", m[row][col])
			if col != 1 {
				s += " "
			}
		}
		if row != 2 {
			s += "\n"
		}
	}
	return s
}

var (
	IdMat4 = Mat4{
		{1, 0, 0, 0},
		{0, 1, 0, 0},
		{0, 0, 1, 0},
		{0, 0, 0, 1},
	}
	IdMat3 = Mat3{
		{1, 0, 0},
		{0, 1, 0},
		{0, 0, 1},
	}
	IdMat2 = Mat2{
		{1, 0},
		{0, 1},
	}
)

func MatrixIdentity(dimensions int) Matrix {
	switch dimensions {
	case 2:
		return &Mat2{
			{1, 0},
			{0, 1},
		}
	case 3:
		return &Mat3{
			{1, 0, 0},
			{0, 1, 0},
			{0, 0, 1},
		}
	case 4:
		return &Mat4{
			{1, 0, 0, 0},
			{0, 1, 0, 0},
			{0, 0, 1, 0},
			{0, 0, 0, 1},
		}
	}

	panic("Unsupported matrix size")
}

func MatrixEqual(a, b Matrix, c *Cmp) bool {
	sa, sb := a.MatrixSize(), b.MatrixSize()
	if sa != sb {
		return false
	}
	size := sa

	if c == nil {
		switch size {
		case 4:
			a, b := a.(*Mat4), b.(*Mat4)
			// -funroll-loops :)
			// TODO: SIMD?
			return (a[0][0] == b[0][0] &&
				a[0][1] == b[0][1] &&
				a[0][2] == b[0][2] &&
				a[0][3] == b[0][3] &&
				a[1][0] == b[1][0] &&
				a[1][1] == b[1][1] &&
				a[1][2] == b[1][2] &&
				a[1][3] == b[1][3] &&
				a[2][0] == b[2][0] &&
				a[2][1] == b[2][1] &&
				a[2][2] == b[2][2] &&
				a[2][3] == b[2][3] &&
				a[3][0] == b[3][0] &&
				a[3][1] == b[3][1] &&
				a[3][2] == b[3][2] &&
				a[3][3] == b[3][3])
		case 3:
			a, b := a.(*Mat3), b.(*Mat3)
			return (a[0][0] == b[0][0] &&
				a[0][1] == b[0][1] &&
				a[0][2] == b[0][2] &&
				a[1][0] == b[1][0] &&
				a[1][1] == b[1][1] &&
				a[1][2] == b[1][2] &&
				a[2][0] == b[2][0] &&
				a[2][1] == b[2][1] &&
				a[2][2] == b[2][2])
		case 2:
			a, b := a.(*Mat2), b.(*Mat2)
			return (a[0][0] == b[0][0] &&
				a[0][1] == b[0][1] &&
				a[1][0] == b[1][0] &&
				a[1][1] == b[1][1])
		}
	} else {
		switch size {
		case 4:
			a, b := a.(*Mat4), b.(*Mat4)
			return (c.Equal(a[0][0], b[0][0]) &&
				c.Equal(a[0][1], b[0][1]) &&
				c.Equal(a[0][2], b[0][2]) &&
				c.Equal(a[0][3], b[0][3]) &&
				c.Equal(a[1][0], b[1][0]) &&
				c.Equal(a[1][1], b[1][1]) &&
				c.Equal(a[1][2], b[1][2]) &&
				c.Equal(a[1][3], b[1][3]) &&
				c.Equal(a[2][0], b[2][0]) &&
				c.Equal(a[2][1], b[2][1]) &&
				c.Equal(a[2][2], b[2][2]) &&
				c.Equal(a[2][3], b[2][3]) &&
				c.Equal(a[3][0], b[3][0]) &&
				c.Equal(a[3][1], b[3][1]) &&
				c.Equal(a[3][2], b[3][2]) &&
				c.Equal(a[3][3], b[3][3]))
		case 3:
			a, b := a.(*Mat3), b.(*Mat3)
			return (c.Equal(a[0][0], b[0][0]) &&
				c.Equal(a[0][1], b[0][1]) &&
				c.Equal(a[0][2], b[0][2]) &&
				c.Equal(a[1][0], b[1][0]) &&
				c.Equal(a[1][1], b[1][1]) &&
				c.Equal(a[1][2], b[1][2]) &&
				c.Equal(a[2][0], b[2][0]) &&
				c.Equal(a[2][1], b[2][1]) &&
				a[2][2] == b[2][2])
		case 2:
			a, b := a.(*Mat2), b.(*Mat2)
			return (c.Equal(a[0][0], b[0][0]) &&
				c.Equal(a[0][1], b[0][1]) &&
				c.Equal(a[1][0], b[1][0]) &&
				c.Equal(a[1][1], b[1][1]))
		}
	}

	return false
}

// MatrixMultiply multiplies 2 Mat4's. There's no need to multiply 2's and 3's.
func MatrixMultiply(a, b *Mat4) *Mat4 {
	m := &Mat4{}

	for row := 0; row < 4; row++ {
		for col := 0; col < 4; col++ {
			m[row][col] = a[row][0]*b[0][col] +
				a[row][1]*b[1][col] +
				a[row][2]*b[2][col] +
				a[row][3]*b[3][col]
		}
	}

	return m
}

// MatrixTupMultiply multiplies a matrix by a tuple.
func MatrixTupMultiply(m *Mat4, t *Tup) *Tup {
	tup := [4]float64{t.X, t.Y, t.Z, t.W}
	result := [4]float64{}

	for row := 0; row < 4; row++ {
		result[row] = m[row][0]*tup[0] +
			m[row][1]*tup[1] +
			m[row][2]*tup[2] +
			m[row][3]*tup[3]
	}

	return &Tup{result[0], result[1], result[2], result[3]}
}

// MatrixTranspose returns the transposed version of m.
func MatrixTranspose(m *Mat4) *Mat4 {
	return &Mat4{
		{m[0][0], m[1][0], m[2][0], m[3][0]},
		{m[0][1], m[1][1], m[2][1], m[3][1]},
		{m[0][2], m[1][2], m[2][2], m[3][2]},
		{m[0][3], m[1][3], m[2][3], m[3][3]},
	}
}

// MatrixDeterminant returns the determinant of m.
func MatrixDeterminant(m Matrix) float64 {
	size := m.MatrixSize()

	switch size {
	case 2:
		m := m.(*Mat2)
		return m[0][0]*m[1][1] - m[1][0]*m[0][1]

	case 3:
		m := m.(*Mat3)
		result := float64(0)
		for col := 0; col < 3; col++ {
			cofactor := MatrixCofactor(m, 0, col)
			result += m[0][col] * cofactor
		}
		return result

	case 4:
		m := m.(*Mat4)
		result := float64(0)
		for col := 0; col < 4; col++ {
			cofactor := MatrixCofactor(m, 0, col)
			result += m[0][col] * cofactor
		}
		return result
	}

	panic("Unsupported matrix size")
}

func Submatrix(m Matrix, rowToRemove, colToRemove int) Matrix {
	size := m.MatrixSize()

	switch size {
	case 4:
		m := m.(*Mat4)
		out := &Mat3{}
		outRow := 0
		for row := 0; row < size; row++ {
			if row == rowToRemove {
				continue
			}

			outCol := 0
			for col := 0; col < size; col++ {
				if col == colToRemove {
					continue
				}

				out[outRow][outCol] = m[row][col]
				outCol++
			}
			outRow++
		}
		return out

	case 3:
		m := m.(*Mat3)
		out := &Mat2{}
		outRow := 0
		for row := 0; row < size; row++ {
			if row == rowToRemove {
				continue
			}

			outCol := 0
			for col := 0; col < size; col++ {
				if col == colToRemove {
					continue
				}

				out[outRow][outCol] = m[row][col]
				outCol++
			}
			outRow++
		}
		return out
	}

	panic("Unsupported matrix size")
}

func MatrixMinor(m Matrix, rowToRemove, colToRemove int) float64 {
	return MatrixDeterminant(Submatrix(m, rowToRemove, colToRemove))
}

func MatrixCofactor(m Matrix, rowToRemove, colToRemove int) float64 {
	determinant := MatrixDeterminant(Submatrix(m, rowToRemove, colToRemove))

	if (rowToRemove+colToRemove)&1 == 0 {
		return determinant
	}

	return -determinant
}

func MatrixIsInversible(m Matrix) bool {
	return MatrixDeterminant(m) != 0
}

func MatrixInverse(m Matrix) Matrix {
	if !MatrixIsInversible(m) {
		panic("non-inversable matrix")
	}
	size := m.MatrixSize()

	determinant := MatrixDeterminant(m)

	switch size {
	case 2:
		result := &Mat2{}
		for row := 0; row < 2; row++ {
			for col := 0; col < 2; col++ {
				cofactor := MatrixCofactor(m, row, col)
				result[col][row] = cofactor / determinant
			}
		}
		return result

	case 3:
		result := &Mat3{}
		for row := 0; row < 3; row++ {
			for col := 0; col < 3; col++ {
				cofactor := MatrixCofactor(m, row, col)
				result[col][row] = cofactor / determinant
			}
		}
		return result

	case 4:
		result := &Mat4{}
		for row := 0; row < 4; row++ {
			for col := 0; col < 4; col++ {
				cofactor := MatrixCofactor(m, row, col)
				result[col][row] = cofactor / determinant
			}
		}
		return result
	}

	panic("Unsupported matrix size")
}
