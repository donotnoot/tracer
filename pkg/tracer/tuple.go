package tracer

import "math"

// Tup is a 4-float64 tuple.
type Tup struct{ X, Y, Z, W float64 }

// Point allocates a new Tup of type Point
func Point(x, y, z float64) *Tup { return &Tup{x, y, z, 1} }

// Vector allocates a new Tup of type Vector
func Vector(x, y, z float64) *Tup { return &Tup{x, y, z, 0} }

// Color creates a new tuple representing a color
func Color(r, g, b float64) *Tup { return Vector(r, g, b) }

// Hmmm, why not, sure. Let's do this.
func (t *Tup) Red() float64   { return t.X }
func (t *Tup) Green() float64 { return t.Y }
func (t *Tup) Blue() float64  { return t.Z }

// IsPoint returns whether the tuple is a point.
// TODO: maybe can cause precision problems?
func (t *Tup) IsPoint() bool { return t.W == 1 }

// IsVector returns whether the tuple is a vector.
// TODO: maybe can cause precision problems?
func (t *Tup) IsVector() bool { return t.W == 0 }

// EqTup compares tuples a and b, returns true if they're equal.
func EqTup(a, b *Tup, c *Cmp) bool {
	if c == nil {
		return a.X == b.X && a.Y == b.Y && a.Z == b.Z && a.W == b.W
	} else {
		return (c.Equal(a.X, b.X) && c.Equal(a.Y, b.Y) &&
			c.Equal(a.Z, b.Z) && c.Equal(a.W, b.W))
	}
}

// Equal compares t with a using c and returns true if they're equal.
func (t *Tup) Equal(a *Tup, c *Cmp) bool { return EqTup(t, a, c) }

// Add a to t in place.
func (t *Tup) Add(a *Tup) { t.X += a.X; t.Y += a.Y; t.Z += a.Z; t.W += a.W }

// AddTup returns a tuple representing the addtion of a and b.
func AddTup(a, b *Tup) *Tup { return &Tup{a.X + b.X, a.Y + b.Y, a.Z + b.Z, a.W + b.W} }

// Subtract a from t in place.
func (t *Tup) Subtract(a *Tup) { t.X -= a.X; t.Y -= a.Y; t.Z -= a.Z; t.W -= a.W }

// SubTup returns a new tuple representing the substraction of b from a.
func SubTup(a, b *Tup) *Tup { return &Tup{a.X - b.X, a.Y - b.Y, a.Z - b.Z, a.W - b.W} }

// NegTup returs a new tuple representing the negation of t.
func NegTup(t *Tup) *Tup { return &Tup{-t.X, -t.Y, -t.Z, -t.W} }

// Negate negates the tuple in place.
func (t *Tup) Negate() { t.X, t.Y, t.Z, t.W = -t.X, -t.Y, -t.Z, -t.W }

// MulTup returns a new tuple representing it's multiplication by factor f.
func ScaleTup(t *Tup, f float64) *Tup { return &Tup{t.X * f, t.Y * f, t.Z * f, t.W * f} }

// Scale multiplies the tuple t by factor f, in place.
func (t *Tup) Scale(f float64) { t.X *= f; t.Y *= f; t.Z *= f; t.W *= f }

// DivTup returns a new tuple representing it's division by factor f.
func DivTup(t *Tup, f float64) *Tup { return &Tup{t.X / f, t.Y / f, t.Z / f, t.W / f} }

// Divide multiplies the tuple t by factor f, in place.
func (t *Tup) Divide(f float64) { t.X /= f; t.Y /= f; t.Z /= f; t.W /= f }

// Magnitude returns the magnitude of t.
func (t *Tup) Magnitude() float64 {
	return math.Sqrt(t.X*t.X + t.Y*t.Y + t.Z*t.Z + t.W*t.W)
}

// Normalize returns the normalised tuple of t.
func (t *Tup) Normalize() *Tup {
	mag := t.Magnitude()
	return &Tup{t.X / mag, t.Y / mag, t.Z / mag, t.W / mag}
}

// DotTup returns a new tuple representing the dot product of a and b.
func DotTup(a, b *Tup) float64 {
	return a.X*b.X + a.Y*b.Y + a.Z*b.Z + a.W*b.W
}

// CrossVec returns the cross product of vectors a and b.
func CrossVec(a, b *Tup) *Tup {
	return Vector(a.Y*b.Z-a.Z*b.Y, a.Z*b.X-a.X*b.Z, a.X*b.Y-a.Y*b.X)
}

// MulTup returns a tuple that represents the Hadamard Product of a and b.
func MulTup(a, b *Tup) *Tup {
	return &Tup{a.X * b.X, a.Y * b.Y, a.Z * b.Z, a.W * b.W}
}

// Multiply applies the a Hadamard Product to t in place.
func (t *Tup) Multiply(a *Tup) { t.X *= a.X; t.Y *= a.Y; t.Z *= a.Z; t.W *= a.W }

func (t *Tup) Reflect(normal *Tup) *Tup {
	return SubTup(t, ScaleTup(ScaleTup(normal, 2), DotTup(t, normal)))
}
