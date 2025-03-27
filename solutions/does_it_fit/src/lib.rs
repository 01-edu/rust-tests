pub mod areas_volumes;

pub use areas_volumes::*;

#[inline]
fn generic_fits(max_size: usize, size: f64, times: usize) -> bool {
    times as f64 * size <= max_size as f64
}

#[inline]
pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    generic_fits(
        x * y,
        match kind {
            GeometricalShapes::Square => square_area(a) as _,
            GeometricalShapes::Circle => circle_area(a),
            GeometricalShapes::Rectangle => rectangle_area(a, b) as _,
            GeometricalShapes::Triangle => triangle_area(a, b),
        },
        times,
    )
}

#[inline]
pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    generic_fits(
        x * y * z,
        match kind {
            GeometricalVolumes::Cube => cube_volume(a) as _,
            GeometricalVolumes::Sphere => sphere_volume(a),
            GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as _,
            GeometricalVolumes::TriangularPyramid => {
                triangular_pyramid_volume(triangle_area(a, b), c)
            }
            GeometricalVolumes::Cone => cone_volume(a, b),
        },
        times,
    )
}
