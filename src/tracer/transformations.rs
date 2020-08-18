use super::matrix::{Kind, Mat};
use super::tuple;

pub fn translation(x: f32, y: f32, z: f32) -> Mat {
    let mat = [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let kind = Kind::TransformNoScale;

    Mat::new(mat, kind)
}

pub fn scaling(x: f32, y: f32, z: f32) -> Mat {
    let mat = [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let kind = Kind::Transform;

    Mat::new(mat, kind)
}

pub fn rotate_x(rad: f32) -> Mat {
    let sin = rad.sin();
    let cos = rad.cos();
    let mat = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, -sin, 0.0],
        [0.0, sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let kind = Kind::TransformNoScale;

    Mat::new(mat, kind)
}

pub fn rotate_y(rad: f32) -> Mat {
    let sin = rad.sin();
    let cos = rad.cos();
    let mat = [
        [cos, 0.0, sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let kind = Kind::TransformNoScale;

    Mat::new(mat, kind)
}

pub fn rotate_z(rad: f32) -> Mat {
    let sin = rad.sin();
    let cos = rad.cos();
    let mat = [
        [cos, -sin, 0.0, 0.0],
        [sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let kind = Kind::TransformNoScale;

    Mat::new(mat, kind)
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Mat {
    let mat = [
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let kind = Kind::General;

    Mat::new(mat, kind)
}

pub fn view(from: tuple::Tup, to: tuple::Tup, up: tuple::Tup) -> Mat {
    let forward = (&to - &from).normalize();
    let left = tuple::cross(&forward, &up.normalize());
    let true_up = tuple::cross(&left, &forward);

    let mat = [
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let kind = Kind::General;

    Mat::new(mat, kind) * translation(-from.x, -from.y, -from.z)
}
