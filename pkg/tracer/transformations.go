package tracer

import "math"

func TranslationMatrix(x, y, z float64) *Mat4 {
	m := MatrixIdentity(4).(*Mat4)
	m[0][3] = x
	m[1][3] = y
	m[2][3] = z
	return m
}

func ScalingMatrix(x, y, z float64) *Mat4 {
	m := MatrixIdentity(4).(*Mat4)
	m[0][0] = x
	m[1][1] = y
	m[2][2] = z
	return m
}

func RotateXMatrix(rad float64) *Mat4 {
	m := MatrixIdentity(4).(*Mat4)
	sin, cos := math.Sin(rad), math.Cos(rad)
	m[1][1] = cos
	m[1][2] = -sin
	m[2][1] = sin
	m[2][2] = cos
	return m
}

func RotateYMatrix(rad float64) *Mat4 {
	m := MatrixIdentity(4).(*Mat4)
	sin, cos := math.Sin(rad), math.Cos(rad)
	m[0][0] = cos
	m[0][2] = sin
	m[2][0] = -sin
	m[2][2] = cos
	return m
}

func RotateZMatrix(rad float64) *Mat4 {
	m := MatrixIdentity(4).(*Mat4)
	sin, cos := math.Sin(rad), math.Cos(rad)
	m[0][0] = cos
	m[0][1] = -sin
	m[1][0] = sin
	m[1][1] = cos
	return m
}

func ShearingMatrix(xy, xz, yx, yz, zx, zy float64) *Mat4 {
	m := MatrixIdentity(4).(*Mat4)
	m[0][1] = xy
	m[0][2] = xz
	m[1][0] = yx
	m[1][2] = yz
	m[2][0] = zx
	m[2][1] = zy
	return m
}
