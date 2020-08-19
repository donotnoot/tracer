use super::matrix::Mat;
use super::objects::Tri;
use super::tuple::{point, vector, Tup};
use std::error::Error;
use std::io::prelude::*;
use std::io::{BufReader, Read};

enum Face {
    Face {
        // Face indices
        x: usize,
        y: usize,
        z: usize,
    },
    FaceWithNormal {
        // Face indices
        x: usize,
        y: usize,
        z: usize,
        // Normal indices
        x_n: usize,
        y_n: usize,
        z_n: usize,
    },
}

pub fn parse_obj(r: impl Read, transform: Mat, smooth: bool) -> Result<Vec<Tri>, Box<dyn Error>> {
    let reader = BufReader::new(r);

    let mut vertices: Vec<Tup> = Vec::new();
    let mut normals: Vec<Tup> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();

    for line in reader.lines().into_iter() {
        let line: String = line?;
        let split = line.trim().split(' ').collect::<Vec<&str>>();
        match split.as_slice() {
            &["v", x, y, z] => {
                vertices.push(point(x.parse()?, y.parse()?, z.parse()?));
            }
            &["vn", x, y, z] => {
                normals.push(vector(x.parse()?, y.parse()?, z.parse()?));
            }
            &["f", a, b, c] => {
                let spl_a = a.trim().split('/').collect::<Vec<&str>>();
                let spl_b = b.trim().split('/').collect::<Vec<&str>>();
                let spl_c = c.trim().split('/').collect::<Vec<&str>>();

                match (smooth, spl_a.as_slice(), spl_b.as_slice(), spl_c.as_slice()) {
                    // All have vertex, texture and normal indices
                    (true, &[avi, ati, ani], &[bvi, bti, bni], &[cvi, cti, cni]) => {
                        faces.push(Face::FaceWithNormal {
                            x: avi.parse()?,
                            y: bvi.parse()?,
                            z: cvi.parse()?,
                            x_n: ani.parse()?,
                            y_n: bni.parse()?,
                            z_n: cni.parse()?,
                        });
                    }

                    // With all data, but ignore it for flat shading
                    (false, &[avi, ati, ani], &[bvi, bti, bni], &[cvi, cti, cni]) => {
                        faces.push(Face::Face {
                            x: avi.parse()?,
                            y: bvi.parse()?,
                            z: cvi.parse()?,
                        });
                    }

                    // Only vertex indices
                    (_, &[avi], &[bvi], &[cvi]) => {
                        faces.push(Face::Face {
                            x: avi.parse()?,
                            y: bvi.parse()?,
                            z: cvi.parse()?,
                        });
                    }

                    // Who knows what else could be here!
                    _ => (),
                }
            }
            _ => (),
        }
    }

    Ok(faces
        .into_iter()
        .filter_map(|face| match face {
            Face::Face { x, y, z } => {
                match (
                    vertices.get(x - 1),
                    vertices.get(y - 1),
                    vertices.get(z - 1),
                ) {
                    (Some(x), Some(y), Some(z)) => Some(Tri::new(
                        transform.clone(),
                        x.clone(),
                        y.clone(),
                        z.clone(),
                        None,
                    )),
                    _ => None,
                }
            }
            Face::FaceWithNormal {
                x,
                y,
                z,
                x_n,
                y_n,
                z_n,
            } => {
                match (
                    vertices.get(x - 1),
                    vertices.get(y - 1),
                    vertices.get(z - 1),
                    normals.get(x_n - 1),
                    normals.get(y_n - 1),
                    normals.get(z_n - 1),
                ) {
                    (Some(x), Some(y), Some(z), Some(x_n), Some(y_n), Some(z_n)) => Some(Tri::new(
                        transform.clone(),
                        x.clone(),
                        y.clone(),
                        z.clone(),
                        Some((x_n.clone(), y_n.clone(), z_n.clone())),
                    )),
                    _ => None,
                }
            }
        })
        .collect())
}
