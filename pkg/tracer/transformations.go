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

func ViewMatrix(from, to, up *Tup) *Mat4 {
	forward := SubTup(to, from).Normalize()
	left := CrossVec(forward, up.Normalize())
	trueUp := CrossVec(left, forward)

	m := MatrixIdentity(4).(*Mat4)

	m[0][0], m[0][1], m[0][2] = left.X, left.Y, left.Z
	m[1][0], m[1][1], m[1][2] = trueUp.X, trueUp.Y, trueUp.Z

	m[2][0], m[2][1], m[2][2] = -forward.X, -forward.Y, -forward.Z

	return MatrixMultiply(m, TranslationMatrix(-from.X, -from.Y, -from.Z))
}
