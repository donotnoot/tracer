package tracer

import "math"

type Intersection struct {
	T      float64
	Object Object
}

func (i *Intersection) PrepareComputations(r *Ray) *Computations {
	c := &Computations{T: i.T, Object: i.Object}
	c.Point = r.Position(c.T)
	c.Eye = NegTup(r.Direction)
	c.Normal = c.Object.Normal(c.Point)

	if DotTup(c.Normal, c.Eye) < 0 {
		c.Inside = true
		c.Normal.Negate()
	}

	return c
}

type Intersections []*Intersection

func (i Intersections) Len() int           { return len(i) }
func (i Intersections) Less(a, b int) bool { return i[a].T < i[b].T }
func (i Intersections) Swap(a, b int)      { i[a], i[b] = i[b], i[a] }

// Hit returns the intersection with the smallest non-negative t.
// Optimize me :)
func (i Intersections) Hit() (float64, int, bool) {
	min, index, hit := math.Inf(1), 0, false

	for i, elem := range i {
		if elem.T < 0 {
			continue
		}
		if elem.T < min {
			hit = true
			min = elem.T
			index = i
		}
	}

	return min, index, hit
}

type Computations struct {
	T      float64
	Object Object
	Inside bool
	Point  *Tup
	Eye    *Tup
	Normal *Tup
}
