package main

import (
	"os"

	"github.com/donotnoot/tracer/pkg/tracer"
)

type Projectile struct {
	Position *tracer.Tup
	Velocity *tracer.Tup
}

type Environment struct {
	Gravity *tracer.Tup
	Wind    *tracer.Tup
}

func tick(env *Environment, proj *Projectile) *Projectile {
	position := tracer.AddTup(proj.Position, proj.Velocity)
	velocity := tracer.AddTup(tracer.AddTup(proj.Velocity, env.Gravity), env.Wind)
	return &Projectile{position, velocity}
}

func main() {
	proj := &Projectile{tracer.Point(0, 1, 0), tracer.ScaleTup(tracer.Vector(3, 4, 0).Normalize(), 11.25)}
	env := &Environment{tracer.Vector(0, -.1, 0), tracer.Vector(-.01, 0, 0)}

	canvas := tracer.NewPPMCanvas(900, 550)

	i := 0
	for {
		proj = tick(env, proj)
		canvas.WritePixel(int(proj.Position.X), int(proj.Position.Y), tracer.Color(200, 200, 200))
		if proj.Position.Y <= 0 {
			break
		}
		i++
	}

	os.Stdout.Write(canvas.PPM())
}
