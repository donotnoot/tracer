package tracer

import "math"

type Intersections []*Intersection

type Intersection struct {
	T      float64
	Object Object
}

// Hit returns the intersection with the smallest non-negative t.
// Optimize me :)
func (i Intersections) Hit() (float64, bool) {
	min := math.Inf(1)

	hit := false
	for _, elem := range i {
		if elem.T < 0 {
			continue
		}
		if elem.T < min {
			hit = true
			min = elem.T
		}
	}

	return min, hit
}
