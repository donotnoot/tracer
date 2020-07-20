use super::matrix::{Mat, Kind, identity};
use super::tuple;

pub fn translation(x: f32, y: f32, z: f32) -> Mat {
    let mut m = identity(4);
    m.kind = Kind::TransformNoScale;

    m.mat[0][3] = x;
    m.mat[1][3] = y;
    m.mat[2][3] = z;
    m
}

pub fn scaling(x: f32, y: f32, z: f32) -> Mat {
    let mut m = identity(4);
    m.kind = Kind::Transform;

    m.mat[0][0] = x;
    m.mat[1][1] = y;
    m.mat[2][2] = z;
    m
}

pub fn rotate_x(rad: f32) -> Mat {
    let mut m = identity(4);
    m.kind = Kind::TransformNoScale;

    let sin = rad.sin();
    let cos = rad.cos();
    m.mat[1][1] = cos;
    m.mat[1][2] = -sin;
    m.mat[2][1] = sin;
    m.mat[2][2] = cos;
    m
}

pub fn rotate_y(rad: f32) -> Mat {
    let mut m = identity(4);
    m.kind = Kind::TransformNoScale;

    let sin = rad.sin();
    let cos = rad.cos();
    m.mat[0][0] = cos;
    m.mat[0][2] = sin;
    m.mat[2][0] = -sin;
    m.mat[2][2] = cos;
    m
}

pub fn rotate_z(rad: f32) -> Mat {
    let mut m = identity(4);
    m.kind = Kind::TransformNoScale;

    let sin = rad.sin();
    let cos = rad.cos();
    m.mat[0][0] = cos;
    m.mat[0][1] = -sin;
    m.mat[1][0] = sin;
    m.mat[1][1] = cos;
    m
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Mat {
    let mut m = identity(4);
    m.kind = Kind::TransformNoScale;

    m.mat[0][1] = xy;
    m.mat[0][2] = xz;
    m.mat[1][0] = yx;
    m.mat[1][2] = yz;
    m.mat[2][0] = zx;
    m.mat[2][1] = zy;
    m
}

pub fn view(from: tuple::Tup, to: tuple::Tup, up: tuple::Tup) -> Mat {
    let mut m = identity(4);
    m.kind = Kind::General;

    let forward = (&to - &from).normalize();
    let left = tuple::cross(&forward, &up.normalize());
    let true_up = tuple::cross(&left, &forward);

    m.mat[0][0] = left.x;
    m.mat[0][1] = left.y;
    m.mat[0][2] = left.z;

    m.mat[1][0] = true_up.x;
    m.mat[1][1] = true_up.y;
    m.mat[1][2] = true_up.z;

    m.mat[2][0] = -forward.x;
    m.mat[2][1] = -forward.y;
    m.mat[2][2] = -forward.z;

    &m * &(translation(-from.x, -from.y, -from.z))
}

// TODO: testy times
