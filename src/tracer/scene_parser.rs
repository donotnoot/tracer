use super::camera::Camera;
use super::light::{Light, LightKind};
use super::material::Material;
use super::matrix;
use super::matrix::Mat;
use super::objects::{Cube, Geometry, Object, Plane, Sphere, Tri};
use super::patterns::*;
use super::transformations::*;
use super::tuple::{color, color_u8, point, vector, Tup};
use super::world::World;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct SceneFile {
    camera: CameraSection,
    background_color: ColorSpec,

    #[serde(default)]
    light: LightSpec,

    #[serde(default)]
    rendering: RenderingSpec,

    #[serde(default)]
    objects: Vec<ObjectSpec>,

    #[serde(default)]
    textures: HashMap<String, TextureSpec>,

    #[serde(default)]
    colors: HashMap<String, ColorSpec>,

    #[serde(default)]
    materials: HashMap<String, MaterialSpec>,

    #[serde(default)]
    patterns: HashMap<String, PatternSpec>,
}

#[derive(Debug, Deserialize)]
pub struct LightSpec {
    pub position: [f32; 3],
    pub intensity: ColorSpec,
    pub kind: LightKindSpec,
}

impl Default for LightSpec {
    fn default() -> Self {
        LightSpec {
            position: [-10., 10., -10.],
            intensity: ColorSpec::Floats(1.0, 1.0, 1.0),
            kind: LightKindSpec::Point,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LightKindSpec {
    Point,
    Area {
        corner: [f32; 3],
        uvec: [f32; 3],
        vvec: [f32; 3],
        usteps: u32,
        vsteps: u32,
    },
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RenderingSpec {
    pub max_bounces: u32,
    pub randomize_rays: bool,
    pub antialias: u32,
    pub partial_render: Option<Vec<(u32, u32)>>,
}

impl Default for RenderingSpec {
    fn default() -> Self {
        RenderingSpec {
            max_bounces: 64,
            randomize_rays: false,
            antialias: 0,
            partial_render: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CameraSection {
    width: f32,
    height: f32,
    fov: f32,
    from: [f32; 3],
    to: [f32; 3],
    up: [f32; 3],
}

#[derive(Debug, Deserialize)]
#[serde(tag = "shape")]
enum ObjectSpec {
    Sphere(SphereSpec),
    Plane(PlaneSpec),
    Cube(CubeSpec),
    Tri(TriSpec),
}

#[derive(Debug, Deserialize)]
struct PlaneSpec {
    transform: Vec<TransformSpec>,
    material: MaterialSpec,
}

#[derive(Debug, Deserialize)]
struct SphereSpec {
    transform: Vec<TransformSpec>,
    material: MaterialSpec,
}

#[derive(Debug, Deserialize)]
struct CubeSpec {
    transform: Vec<TransformSpec>,
    material: MaterialSpec,
}

#[derive(Debug, Deserialize)]
struct TriSpec {
    transform: Vec<TransformSpec>,
    material: MaterialSpec,
    p1: (f32, f32, f32),
    p2: (f32, f32, f32),
    p3: (f32, f32, f32),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MaterialSpec {
    Reference(String),
    Phong(Phong),
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Phong {
    color: ColorSpec,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    reflectiveness: f32,
    transparency: f32,
    refractive_index: f32,
    pattern: Option<PatternSpec>,
}

impl Default for Phong {
    fn default() -> Self {
        Phong {
            color: ColorSpec::Ints(255, 255, 255),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectiveness: 0.0,
            pattern: None,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ColorSpec {
    Reference(String),
    Ints(u8, u8, u8),
    Floats(f32, f32, f32),
    Hex(u32),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum PatternSpec {
    Reference {
        name: String,
    },
    Stripe {
        color_a: ColorSpec,
        color_b: ColorSpec,
        transform: Option<Vec<TransformSpec>>,
    },
    Gradient {
        color_a: ColorSpec,
        color_b: ColorSpec,
        transform: Option<Vec<TransformSpec>>,
    },
    Checker {
        color_a: ColorSpec,
        color_b: ColorSpec,
        transform: Option<Vec<TransformSpec>>,
    },
    UV {
        mapping: UVMappingSpec,
        pattern: UVPatternSpec,
    },
    Ring {
        color_a: ColorSpec,
        color_b: ColorSpec,
        transform: Option<Vec<TransformSpec>>,
    },
    Mandelbrot {
        color: ColorSpec,
        transform: Option<Vec<TransformSpec>>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum UVPatternSpec {
    Checker {
        color_a: ColorSpec,
        color_b: ColorSpec,
        width: f32,
        height: f32,
    },
    Image {
        texture: TextureSpec,
    },
    CubeImage {
        top: TextureSpec,
        bottom: TextureSpec,
        left: TextureSpec,
        right: TextureSpec,
        front: TextureSpec,
        back: TextureSpec,
    },
}

#[derive(Debug, Deserialize)]
enum UVMappingSpec {
    Spherical,
    Planar,
    Cubical,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TextureSpec {
    Reference { name: String },
    File { path: String },
    B64 { data: String },
}

#[derive(Debug, Deserialize)]
enum TransformSpec {
    Translation(f32, f32, f32),
    Scaling(f32, f32, f32),
    RotateX(f32),
    RotateY(f32),
    RotateZ(f32),
    Rotate(f32, f32, f32),
    Shearing(f32, f32, f32, f32, f32, f32),
}

pub fn from_reader(
    r: impl std::io::Read,
) -> Result<(World, Camera, RenderingSpec), Box<dyn Error>> {
    let scene: SceneFile = serde_yaml::from_reader(r)?;

    let mut world = World::new();

    let light_position = point(
        scene.light.position[0],
        scene.light.position[1],
        scene.light.position[2],
    );

    let light_intensity = scene.process_color(&scene.light.intensity)?;

    world.light = match scene.light.kind {
        LightKindSpec::Point => Light {
            position: light_position,
            intensity: light_intensity,
            kind: LightKind::Point,
        },
        LightKindSpec::Area {
            corner,
            uvec,
            vvec,
            usteps,
            vsteps,
        } => Light::new_area(
            &light_intensity,
            &f32x3_to_point(corner),
            &f32x3_to_vec(uvec),
            usteps,
            &f32x3_to_vec(vvec),
            vsteps,
        ),
    };

    let mut camera = Camera::new(
        scene.camera.width,
        scene.camera.height,
        deg2rad(scene.camera.fov),
        scene.rendering.antialias,
        scene.rendering.max_bounces,
    );
    camera.set_transform(view(
        point(
            scene.camera.from[0],
            scene.camera.from[1],
            scene.camera.from[2],
        ),
        point(scene.camera.to[0], scene.camera.to[1], scene.camera.to[2]),
        vector(scene.camera.up[0], scene.camera.up[1], scene.camera.up[2]),
    ));

    world.background_color = scene.process_color(&scene.background_color)?;

    let results: Result<Vec<Object>, Box<dyn Error>> = scene
        .objects
        .iter()
        .map(|spec| match spec {
            ObjectSpec::Sphere(spec) => {
                let sphere = Sphere::new(scene.process_transformations(&spec.transform));
                Ok(Object {
                    geometry: Geometry::Sphere(sphere),
                    material: scene.process_material(&spec.material)?,
                })
            }
            ObjectSpec::Plane(spec) => {
                let plane = Plane::new(scene.process_transformations(&spec.transform));
                Ok(Object {
                    geometry: Geometry::Plane(plane),
                    material: scene.process_material(&spec.material)?,
                })
            }
            ObjectSpec::Cube(spec) => {
                let cube = Cube::new(scene.process_transformations(&spec.transform));
                Ok(Object {
                    geometry: Geometry::Cube(cube),
                    material: scene.process_material(&spec.material)?,
                })
            }
            ObjectSpec::Tri(spec) => {
                let tri = Tri::new(
                    scene.process_transformations(&spec.transform),
                    point(spec.p1.0, spec.p1.1, spec.p1.2),
                    point(spec.p2.0, spec.p2.1, spec.p2.2),
                    point(spec.p3.0, spec.p3.1, spec.p3.2),
                );
                Ok(Object {
                    geometry: Geometry::Tri(tri),
                    material: scene.process_material(&spec.material)?,
                })
            }
        })
        .collect();

    world.objects = match results {
        Ok(results) => Ok(results),
        Err(error) => Err(error),
    }?;

    Ok((world, camera, scene.rendering))
}

impl SceneFile {
    fn process_transformations(&self, t: &[TransformSpec]) -> Mat {
        let mut m = matrix::identity();

        for transform in t.iter() {
            m = m * self.process_transform(transform);
        }

        m
    }

    fn process_transform(&self, t: &TransformSpec) -> Mat {
        match t {
            TransformSpec::Translation(x, y, z) => translation(*x, *y, *z),
            TransformSpec::RotateX(deg) => rotate_x(deg2rad(*deg)),
            TransformSpec::RotateY(deg) => rotate_y(deg2rad(*deg)),
            TransformSpec::RotateZ(deg) => rotate_z(deg2rad(*deg)),
            TransformSpec::Rotate(x, y, z) => {
                rotate_z(deg2rad(*z)) * rotate_y(deg2rad(*y)) * rotate_x(deg2rad(*x))
            }
            TransformSpec::Scaling(x, y, z) => scaling(*x, *y, *z),
            TransformSpec::Shearing(xy, xz, yx, yz, zx, zy) => {
                shearing(*xy, *xz, *yx, *yz, *zx, *zy)
            }
        }
    }

    fn process_pattern(&self, spec: &PatternSpec) -> Result<Pattern, Box<dyn Error>> {
        match spec {
            PatternSpec::Stripe {
                color_a,
                color_b,
                transform,
            } => Ok(Pattern::Stripe(
                self.process_color(color_a)?,
                self.process_color(color_b)?,
                match transform {
                    Some(t) => Some(self.process_transformations(t)),
                    None => None,
                },
            )),
            PatternSpec::Checker {
                color_a,
                color_b,
                transform,
            } => Ok(Pattern::Checker(
                self.process_color(color_a)?,
                self.process_color(color_b)?,
                match transform {
                    Some(t) => Some(self.process_transformations(t)),
                    None => None,
                },
            )),
            PatternSpec::Gradient {
                color_a,
                color_b,
                transform,
            } => Ok(Pattern::Gradient(
                self.process_color(color_a)?,
                self.process_color(color_b)?,
                match transform {
                    Some(t) => Some(self.process_transformations(t)),
                    None => None,
                },
            )),
            PatternSpec::UV { mapping, pattern } => {
                let mapping = match mapping {
                    UVMappingSpec::Spherical => UVMapping::Spherical,
                    UVMappingSpec::Planar => UVMapping::Planar,
                    UVMappingSpec::Cubical => UVMapping::Cubical,
                };
                let pattern = match pattern {
                    UVPatternSpec::Checker {
                        color_a,
                        color_b,
                        width,
                        height,
                    } => UVPattern::Checker(
                        self.process_color(color_a)?,
                        self.process_color(color_b)?,
                        *width,
                        *height,
                    ),
                    UVPatternSpec::Image { texture } => {
                        let texture = self.process_texture(texture)?;
                        UVPattern::Image(texture)
                    }
                    UVPatternSpec::CubeImage {
                        top,
                        bottom,
                        left,
                        right,
                        front,
                        back,
                    } => {
                        let top = self.process_texture(top)?;
                        let bottom = self.process_texture(bottom)?;
                        let left = self.process_texture(left)?;
                        let right = self.process_texture(right)?;
                        let front = self.process_texture(front)?;
                        let back = self.process_texture(back)?;
                        UVPattern::CubeImage {
                            top,
                            bottom,
                            left,
                            right,
                            front,
                            back,
                        }
                    }
                };

                Ok(Pattern::UV(mapping, pattern))
            }
            PatternSpec::Ring {
                color_a,
                color_b,
                transform,
            } => Ok(Pattern::Ring(
                self.process_color(color_a)?,
                self.process_color(color_b)?,
                match transform {
                    Some(t) => Some(self.process_transformations(t)),
                    None => None,
                },
            )),
            PatternSpec::Mandelbrot { color, transform } => Ok(Pattern::Mandelbrot(
                self.process_color(color)?,
                match transform {
                    Some(t) => Some(self.process_transformations(t)),
                    None => None,
                },
            )),
            PatternSpec::Reference { name } => match self.patterns.get(name) {
                Some(name) => Ok(self.process_pattern(name)?),
                None => Err(format!("could not find pattern with name '{}'", name).into()),
            },
        }
    }

    fn process_material(&self, spec: &MaterialSpec) -> Result<Material, Box<dyn Error>> {
        match spec {
            MaterialSpec::Phong(phong) => Ok(self.phong_to_material(phong)?),
            MaterialSpec::Reference(name) => match self.materials.get(name) {
                Some(material) => Ok(self.process_material(material)?),
                None => Err(format!("could not find material with name '{}'", name).into()),
            },
        }
    }

    fn phong_to_material(&self, p: &Phong) -> Result<Material, Box<dyn Error>> {
        Ok(Material {
            color: self.process_color(&p.color)?,
            ambient: p.ambient,
            diffuse: p.diffuse,
            specular: p.specular,
            shininess: p.shininess,
            reflectiveness: p.reflectiveness,
            pattern: match &p.pattern {
                Some(p) => Some(self.process_pattern(&p)?),
                None => None,
            },
            transparency: p.transparency,
            refractive_index: p.refractive_index,
        })
    }

    fn process_color(&self, c: &ColorSpec) -> Result<Tup, Box<dyn Error>> {
        match c {
            ColorSpec::Ints(r, g, b) => Ok(color_u8(*r, *g, *b)),
            ColorSpec::Floats(r, g, b) => Ok(color(*r, *g, *b)),
            ColorSpec::Reference(name) => match self.colors.get(name) {
                Some(color) => Ok(self.process_color(color)?),
                None => Err(format!("could not find material with name '{}'", name).into()),
            },
            ColorSpec::Hex(hex) => Ok(color_u8((*hex >> 16) as u8, (*hex >> 8) as u8, *hex as u8)),
        }
    }

    fn process_texture(&self, t: &TextureSpec) -> Result<Texture, Box<dyn Error>> {
        match t {
            TextureSpec::File { path } => Ok(Texture::read(std::fs::File::open(path)?)?),
            TextureSpec::B64 { data } => Ok(Texture::read(base64::decode(data)?.as_slice())?),
            TextureSpec::Reference { name } => match self.textures.get(name) {
                Some(color) => Ok(self.process_texture(color)?),
                None => Err(format!("could not find texture with name '{}'", name).into()),
            },
        }
    }
}

fn deg2rad(a: f32) -> f32 {
    a * std::f32::consts::PI / 180.
}

fn f32x3_to_point(i: [f32; 3]) -> Tup {
    point(i[0], i[1], i[2])
}

fn f32x3_to_vec(i: [f32; 3]) -> Tup {
    vector(i[0], i[1], i[2])
}
