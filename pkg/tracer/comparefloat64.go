package tracer

// Cmp is a 64-bit float comparator.
type Cmp struct{ Epsilon float64 }

// Equal returns whether two values are equal within the error range of
// Epsilon.
func (c *Cmp) Equal(a, b float64) bool {
	diff := a - b
	if diff < 0 {
		diff = -diff
	}

	return diff < c.Epsilon
}
