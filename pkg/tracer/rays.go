package tracer

type Ray struct {
	Origin    *Tup
	Direction *Tup
}

func (r *Ray) Position(t float64) *Tup {
	return AddTup(r.Origin, ScaleTup(r.Direction, t))
}

func TransformRay(r *Ray, m *Mat4) *Ray {
	return &Ray{
		Origin:    MatrixTupMultiply(m, r.Origin),
		Direction: MatrixTupMultiply(m, r.Direction),
	}
}
