package tracer

import "math"

type Object interface {
	ObjectType() ObjectType
}

type ObjectType uint32

const ObjectTypeSphere ObjectType = iota

type Sphere struct {
	Transform *Mat4
	Material  *Material
}

func NewSphere() *Sphere {
	return &Sphere{
		Transform: MatrixIdentity(4).(*Mat4),
		Material:  NewMaterial(),
	}
}

func (s *Sphere) ObjectType() ObjectType { return ObjectTypeSphere }

func (s *Sphere) Intersect(r *Ray) Intersections {
	transformedRay := TransformRay(r, MatrixInverse(s.Transform).(*Mat4))

	sphereToRay := SubTup(transformedRay.Origin, Point(0, 0, 0))

	a := DotTup(transformedRay.Direction, transformedRay.Direction)
	b := 2.0 * DotTup(transformedRay.Direction, sphereToRay)
	c := DotTup(sphereToRay, sphereToRay) - 1

	discriminant := b*b - 4*a*c

	if discriminant < 0 {
		return nil
	}

	t1 := (-b - math.Sqrt(discriminant)) / (2 * a)
	t2 := (-b + math.Sqrt(discriminant)) / (2 * a)

	if t1 > t2 {
		t2, t1 = t1, t2
	}

	return Intersections{
		&Intersection{
			T:      t1,
			Object: s,
		}, &Intersection{
			T:      t2,
			Object: s,
		},
	}
}

func (s *Sphere) Normal(point *Tup) *Tup {
	objectPoint := MatrixTupMultiply(MatrixInverse(s.Transform).(*Mat4), point)
	objectNormal := SubTup(objectPoint, Point(0, 0, 0))
	worldNormal := MatrixTupMultiply(MatrixTranspose(MatrixInverse(s.Transform).(*Mat4)), objectNormal)
	worldNormal.W = 0
	return worldNormal.Normalize()
}
