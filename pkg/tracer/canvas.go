package tracer

// Cavas is a general interface for the ray tracer to write pixels to.
type Canvas interface {
	WritePixel(x, y int, color *Tup)
	PixelAt(x, y int) *Tup
}
