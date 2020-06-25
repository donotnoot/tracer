package tracer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLight(t *testing.T) {
	t.Parallel()

	t.Run("A point light has a position and a intensity", func(t *testing.T) {
		t.Parallel()

		light := &PointLight{Point(0, 0, 0), Color(1, 1, 1)}

		assert.Equal(t, Point(0, 0, 0), light.Position)
		assert.Equal(t, Color(1, 1, 1), light.Intensity)
	})
}
