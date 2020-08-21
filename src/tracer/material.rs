use super::light::*;
use super::objects::Object;
use super::patterns::Pattern;
use super::tuple::{color, dot, Tup};

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Tup,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflectiveness: f32,
    pub pattern: Option<Pattern>,
    pub transparency: f32,
    pub refractive_index: f32,
    pub light_through: bool,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectiveness: 0.0,
            pattern: None,
            transparency: 0.0,
            refractive_index: 1.0,
            light_through: false,
        }
    }

    pub fn lighting(
        &self,
        o: &Object,
        l: &Vec<Light>,
        p: Tup,
        eye: Tup,
        normal: Tup,
        shadow_color: Tup,
    ) -> Tup {
        let object_color = match &self.pattern {
            Some(c) => c.at_object(o, &p),
            None => self.color.clone(), // TODO: no need to clone this...
        };

        l.iter()
            .map(|l| {
                let effective_color = &object_color * l.color();
                let ambient = &effective_color * self.ambient;

                let light = (l.position() - &p).normalize();
                let light_normal_dot = dot(&light, &normal);

                let (diffuse, specular) = if light_normal_dot < 0.0 {
                    let diffuse = color(0.0, 0.0, 0.0);
                    let specular = color(0.0, 0.0, 0.0);
                    (diffuse, specular)
                } else {
                    let diffuse = &effective_color * (self.diffuse * light_normal_dot);
                    let reflect = &(-&light).reflect(&normal);
                    let reflect_dot_eye = dot(&reflect, &eye);

                    if reflect_dot_eye <= 0.0 {
                        let specular = color(0.0, 0.0, 0.0);
                        (diffuse, specular)
                    } else {
                        let factor = reflect_dot_eye.powf(self.shininess);
                        let specular = l.color() * (self.specular * factor);
                        (diffuse, specular)
                    }
                };

                let diffuse = &diffuse * &shadow_color;
                let specular = &specular * &shadow_color;

                ambient + diffuse + specular
            })
            .sum()
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HasMaterial {
    fn material(&self) -> Material;
}

#[cfg(test)]
mod tests {
    use super::super::light::*;
    use super::super::material::Material;
    use super::super::objects::{Geometry, Object, Sphere};
    use super::super::patterns::Pattern;
    use super::super::tuple::{color, point, vector};

    #[test]
    fn eye_between_light_and_surface() {
        let mat = Material::new();
        let pos = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new_point(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let result = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            &vec![light],
            pos,
            eyev,
            normalv,
            color(1., 1., 1.),
        );

        assert_eq!((1.9 - result.x).abs() <= std::f32::EPSILON, true);
        assert_eq!((1.9 - result.y).abs() <= std::f32::EPSILON, true);
        assert_eq!((1.9 - result.z).abs() <= std::f32::EPSILON, true);
    }

    #[test]
    fn eye_between_light_and_surface_45deg_off() {
        let mat = Material::new();
        let pos = point(0.0, 0.0, 0.0);

        let p = 2_f32.sqrt() / 2.0;
        let eyev = vector(0.0, p, p);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new_point(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let result = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            &vec![light],
            pos,
            eyev,
            normalv,
            color(1., 1., 1.),
        );

        assert_eq!((1.0 - result.x).abs() <= std::f32::EPSILON, true);
        assert_eq!((1.0 - result.y).abs() <= std::f32::EPSILON, true);
        assert_eq!((1.0 - result.z).abs() <= std::f32::EPSILON, true);
    }

    #[test]
    fn eye_opposite_surface_light_45_deg() {
        let mat = Material::new();
        let pos = point(0.0, 0.0, 0.0);

        let p = 2_f32.sqrt() / 2.0;

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new_point(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        let result = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            &vec![light],
            pos,
            eyev,
            normalv,
            color(1., 1., 1.),
        );

        let r = 0.1 + p * 0.9;
        assert_eq!((r - result.x).abs() <= std::f32::EPSILON, true);
        assert_eq!((r - result.y).abs() <= std::f32::EPSILON, true);
        assert_eq!((r - result.z).abs() <= std::f32::EPSILON, true);
    }

    #[test]
    fn eye_in_the_path_of_reflection_vector() {
        let mat = Material::new();
        let pos = point(0.0, 0.0, 0.0);

        let p = 2_f32.sqrt() / 2.0;

        let eyev = vector(0.0, -p, -p);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new_point(point(0.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        let result = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            &vec![light],
            pos,
            eyev,
            normalv,
            color(1., 1., 1.),
        );

        let r = 0.1 + 0.9 * p + 0.9;
        assert!((r - result.x).abs() < 10e-4);
        assert!((r - result.y).abs() < 10e-4);
        assert!((r - result.z).abs() < 10e-4);
    }

    #[test]
    fn light_behind_surface() {
        let mat = Material::new();
        let pos = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new_point(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));
        let result = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            &vec![light],
            pos,
            eyev,
            normalv,
            color(1., 1., 1.),
        );

        assert_eq!((0.1 - result.x).abs() <= std::f32::EPSILON, true);
        assert_eq!((0.1 - result.y).abs() <= std::f32::EPSILON, true);
        assert_eq!((0.1 - result.z).abs() <= std::f32::EPSILON, true);
    }

    #[test]
    fn surface_in_shadow() {
        let mat = Material::new();
        let pos = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new_point(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
        let result = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            &vec![light],
            pos,
            eyev,
            normalv,
            color(0., 0., 0.),
        );

        assert_eq!((0.1 - result.x).abs() <= std::f32::EPSILON, true);
        assert_eq!((0.1 - result.y).abs() <= std::f32::EPSILON, true);
        assert_eq!((0.1 - result.z).abs() <= std::f32::EPSILON, true);
    }

    #[test]
    fn lighting_with_stripe_pattern() {
        let mut mat = Material::new();
        mat.pattern = Some(Pattern::Stripe(
            color(1.0, 1.0, 1.0),
            color(0.0, 0.0, 0.0),
            None,
        ));
        mat.ambient = 1.0;
        mat.diffuse = 0.0;
        mat.specular = 0.0;

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = &vec![Light::new_point(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0))];

        let c1 = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            light,
            point(0.9, 0.0, 0.0),
            eyev.clone(),
            normalv.clone(),
            color(1., 1., 1.),
        );
        let c2 = mat.lighting(
            &Object {
                geometry: Geometry::Sphere(Sphere::default()),
                material: Material::new(),
                normal_map: None,
            },
            light,
            point(1.0, 0.0, 0.0),
            eyev,
            normalv,
            color(1., 1., 1.),
        );

        assert_eq!((1.0 - c2.x).abs() <= std::f32::EPSILON, false);
        assert_eq!((1.0 - c2.y).abs() <= std::f32::EPSILON, false);
        assert_eq!((1.0 - c2.z).abs() <= std::f32::EPSILON, false);

        assert_eq!((0.0 - c1.x).abs() <= std::f32::EPSILON, false);
        assert_eq!((0.0 - c1.y).abs() <= std::f32::EPSILON, false);
        assert_eq!((0.0 - c1.z).abs() <= std::f32::EPSILON, false);
    }
}
