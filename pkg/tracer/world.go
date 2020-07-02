package tracer

import "sort"

type World struct {
	Objects []Object
	Light   *PointLight
}

func NewWorld() *World {
	return &World{
		Light: &PointLight{Point(-10, 10, -10), Color(1, 1, 1)},
		Objects: []Object{
			func() *Sphere {
				s := NewSphere()
				s.Material.Color = Color(0.8, 1.0, 0.6)
				s.Material.Diffuse = 0.7
				s.Material.Specular = 0.2
				return s
			}(),
			func() *Sphere {
				s := NewSphere()
				s.Transform = ScalingMatrix(.5, .5, .5)
				return s
			}(),
		},
	}
}

func (w *World) Intersect(r *Ray) Intersections {
	i := Intersections{}

	// Might want concurrency here...
	for _, object := range w.Objects {
		i = append(i, object.Intersect(r)...)
	}

	sort.Sort(i)

	return i
}

func (w *World) ShadeHit(c *Computations) *Tup {
	s := w.IsShadowed(c.OverPoint)
	return c.Object.GetMaterial().Lighting(w.Light, c.Point, c.Eye, c.Normal, s)
}

func (w *World) ColorAt(r *Ray) *Tup {
	intersections := w.Intersect(r)

	_, i, ok := intersections.Hit()
	if !ok {
		return Color(0, 0, 0)
	}

	return w.ShadeHit(intersections[i].PrepareComputations(r))
}

func (w *World) IsShadowed(p *Tup) bool {
	v := SubTup(w.Light.Position, p)
	distance := v.Magnitude()
	direction := v.Normalize()

	ray := &Ray{p, direction}

	if hit, _, ok := w.Intersect(ray).Hit(); ok {
		return hit < distance
	}

	return false
}
