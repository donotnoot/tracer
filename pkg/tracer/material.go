package tracer

import "math"

const ()

type Material struct {
	Color                                 *Tup
	Ambient, Diffuse, Specular, Shininess float64
}

func NewMaterial() *Material {
	return &Material{
		Color:     Color(1, 1, 1),
		Ambient:   0.1,
		Diffuse:   0.9,
		Specular:  0.9,
		Shininess: 200,
	}
}

func (m *Material) Lighting(l *PointLight, p *Tup, eyev *Tup, normv *Tup, inShadow bool) *Tup {
	var ambient, diffuse, specular *Tup

	effectiveColor := MulTup(m.Color, l.Intensity)
	ambient = ScaleTup(effectiveColor, m.Ambient)

	if inShadow {
		return ambient
	}

	lightv := SubTup(l.Position, p).Normalize()
	lightDotNormal := DotTup(lightv, normv)

	if lightDotNormal < 0 {
		diffuse = Color(0, 0, 0)
		specular = Color(0, 0, 0)
	} else {
		diffuse = ScaleTup(effectiveColor, m.Diffuse*lightDotNormal)
		reflectv := NegTup(lightv).Reflect(normv)
		reflectDotEye := DotTup(reflectv, eyev)

		if reflectDotEye <= 0 {
			specular = Color(0, 0, 0)
		} else {
			factor := math.Pow(reflectDotEye, m.Shininess)
			specular = ScaleTup(l.Intensity, m.Specular*factor)
		}
	}

	return AddTup(AddTup(ambient, diffuse), specular)
}
