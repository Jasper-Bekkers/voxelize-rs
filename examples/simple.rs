use voxelize_rs::*;

fn main() {
    let tri = Triangle::new(
        Vector3::new(0f32, 0f32, 0f32),
        Vector3::new(0f32, 1f32, 0f32),
        Vector3::new(0f32, 0f32, 1f32),
    );

    let coords = voxelize(&vec![tri], &Vector3::new(0.1, 0.1, 0.1));
    dbg!(coords);
}
