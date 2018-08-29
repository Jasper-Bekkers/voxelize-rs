extern crate nalgebra;

use nalgebra::Vector3;

struct Aabb {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

impl Aabb {
    fn new() -> Aabb {
        Aabb {
            min: Vector3::new(1000f32, 1000f32, 1000f32),
            max: Vector3::new(-1000f32, -1000f32, -1000f32),
        }
    }

    fn expand(&mut self, p: &Vector3<f32>) {
        if self.min.x > p.x {
            self.min.x = p.x
        }

        if self.min.y > p.y {
            self.min.y = p.y
        }

        if self.min.z > p.z {
            self.min.z = p.z
        }

        if self.max.x < p.x {
            self.max.x = p.x
        }

        if self.max.y < p.y {
            self.max.y = p.y
        }

        if self.max.z < p.z {
            self.max.z = p.z
        }
    }
}

struct Triangle {
    p0: Vector3<f32>,
    p1: Vector3<f32>,
    p2: Vector3<f32>,
}

impl Triangle {
    fn area(&self) -> f32 {
        1f32
    }

    fn new(p0: Vector3<f32>, p1: Vector3<f32>, p2: Vector3<f32>) -> Triangle {
        Triangle { p0, p1, p2 }
    }

    fn aabb(&self) -> Aabb {
        let mut aabb = Aabb::new();
        aabb.expand(&self.p0);
        aabb.expand(&self.p1);
        aabb.expand(&self.p2);
        aabb
    }
}

fn clamp_to_voxel_min(p: f32, size: f32) -> f32 {
    let vox = (p + p.signum() * size * 0.5) / size;
    vox.floor() * size
}

fn clamp_to_voxel_max(p: f32, size: f32) -> f32 {
    let vox = (p + p.signum() * size * 0.5) / size;
    vox.ceil() * size
}

fn find_min_max(x0: f32, x1: f32, x2: f32) -> (f32, f32) {
    let mut min = x0;
    let mut max = x0;

    if x1 < min {
        min = x1;
    }
    if x1 > max {
        max = x1;
    }
    if x2 < min {
        min = x2;
    }
    if x2 > max {
        max = x2;
    }

    (min, max)
}

fn plane_box_overlap(normal: &Vector3<f32>, vert: &Vector3<f32>, maxbox: &Vector3<f32>) -> bool {
    let mut vmin = Vector3::new(0.0f32, 0.0f32, 0.0f32);
    let mut vmax = Vector3::new(0.0f32, 0.0f32, 0.0f32);

    if (normal.x > 0.0) {
        vmin.x = -maxbox.x - vert.x;
        vmax.x = maxbox.x - vert.x;
    } else {
        vmin.x = maxbox.x - vert.x;
        vmax.x = -maxbox.x - vert.x;
    }

    if (normal.y > 0.0) {
        vmin.y = -maxbox.y - vert.y;
        vmax.y = maxbox.y - vert.y;
    } else {
        vmin.y = maxbox.y - vert.y;
        vmax.y = -maxbox.y - vert.y;
    }

    if (normal.z > 0.0) {
        vmin.z = -maxbox.z - vert.z;
        vmax.z = maxbox.z - vert.z;
    } else {
        vmin.z = maxbox.z - vert.z;
        vmax.z = -maxbox.z - vert.z;
    }

    if normal.dot(&vmin) > 0.0 {
        return false;
    }
    if normal.dot(&vmax) >= 0.0 {
        return true;
    }

    false
}

fn axis_test_x01(
    a: f32,
    b: f32,
    fa: f32,
    fb: f32,
    v0: &Vector3<f32>,
    v2: &Vector3<f32>,
    boxhalfsize: &Vector3<f32>,
) -> bool {
    let p0 = a * v0.y - b * v0.z;
    let p2 = a * v2.y - b * v2.z;

    let (min, max) = if p0 < p2 { (p0, p2) } else { (p2, p0) };

    let rad = fa * boxhalfsize.y + fb * boxhalfsize.z;

    if (min > rad || max < -rad) {
        return false;
    }

    true
}

fn axis_test_x2(
    a: f32,
    b: f32,
    fa: f32,
    fb: f32,
    v0: &Vector3<f32>,
    v1: &Vector3<f32>,
    boxhalfsize: &Vector3<f32>,
) -> bool {
    let p0 = a * v0.y - b * v0.z;
    let p1 = a * v1.y - b * v1.z;

    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };

    let rad = fa * boxhalfsize.y + fb * boxhalfsize.z;

    if (min > rad || max < -rad) {
        return false;
    }

    true
}

fn axis_test_y02(
    a: f32,
    b: f32,
    fa: f32,
    fb: f32,
    v0: &Vector3<f32>,
    v2: &Vector3<f32>,
    boxhalfsize: &Vector3<f32>,
) -> bool {
    let p0 = -a * v0.x - b * v0.z;
    let p2 = -a * v2.x - b * v2.z;

    let (min, max) = if p0 < p2 { (p0, p2) } else { (p2, p0) };

    let rad = fa * boxhalfsize.x + fb * boxhalfsize.z;

    if (min > rad || max < -rad) {
        return false;
    }

    true
}

fn axis_test_y1(
    a: f32,
    b: f32,
    fa: f32,
    fb: f32,
    v0: &Vector3<f32>,
    v1: &Vector3<f32>,
    boxhalfsize: &Vector3<f32>,
) -> bool {
    let p0 = -a * v0.x - b * v0.z;
    let p1 = -a * v1.x - b * v1.z;

    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };

    let rad = fa * boxhalfsize.x + fb * boxhalfsize.z;

    if (min > rad || max < -rad) {
        return false;
    }

    true
}

fn axis_test_z12(
    a: f32,
    b: f32,
    fa: f32,
    fb: f32,
    v1: &Vector3<f32>,
    v2: &Vector3<f32>,
    boxhalfsize: &Vector3<f32>,
) -> bool {
    let p1 = a * v1.x - b * v1.y;
    let p2 = a * v2.x - b * v2.y;

    let (min, max) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

    let rad = fa * boxhalfsize.x + fb * boxhalfsize.y;

    if (min > rad || max < -rad) {
        return false;
    }

    true
}

fn axis_test_z0(
    a: f32,
    b: f32,
    fa: f32,
    fb: f32,
    v0: &Vector3<f32>,
    v1: &Vector3<f32>,
    boxhalfsize: &Vector3<f32>,
) -> bool {
    let p0 = a * v0.x - b * v0.y;
    let p1 = a * v1.x - b * v1.y;

    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };

    let rad = fa * boxhalfsize.x + fb * boxhalfsize.y;

    if (min > rad || max < -rad) {
        return false;
    }

    true
}

fn triangle_box_overlap(
    boxcenter: &Vector3<f32>,
    boxhalfsize: &Vector3<f32>,
    tri: &Triangle,
) -> bool {
    let v0 = tri.p0 - boxcenter;
    let v1 = tri.p1 - boxcenter;
    let v2 = tri.p2 - boxcenter;
    let e0 = v1 - v0;
    let e1 = v2 - v1;
    let e2 = v0 - v2;

    {
        let fex = e0.x.abs();
        let fey = e0.y.abs();
        let fez = e0.z.abs();
        if !axis_test_x01(e0.z, e0.y, fez, fey, &v0, &v2, &boxhalfsize) {
            return false;
        }
        if !axis_test_y02(e0.z, e0.x, fez, fex, &v0, &v2, &boxhalfsize) {
            return false;
        }
        if !axis_test_z12(e0.y, e0.x, fey, fex, &v1, &v2, &boxhalfsize) {
            return false;
        }
    }

    {
        let fex = e1.x.abs();
        let fey = e1.y.abs();
        let fez = e1.z.abs();
        if !axis_test_x01(e1.z, e1.y, fez, fey, &v0, &v2, &boxhalfsize) {
            return false;
        }
        if !axis_test_y02(e1.z, e1.x, fez, fex, &v0, &v2, &boxhalfsize) {
            return false;
        }
        if !axis_test_z0(e1.y, e1.x, fey, fex, &v0, &v1, &boxhalfsize) {
            return false;
        }
    }

    {
        let fex = e2.x.abs();
        let fey = e2.y.abs();
        let fez = e2.z.abs();
        if !axis_test_x2(e2.z, e2.y, fez, fey, &v0, &v1, &boxhalfsize) {
            return false;
        }
        if !axis_test_y1(e2.z, e2.x, fez, fex, &v0, &v1, &boxhalfsize) {
            return false;
        }
        if !axis_test_z12(e2.y, e2.x, fey, fex, &v1, &v2, &boxhalfsize) {
            return false;
        }
    }

    let (min, max) = find_min_max(v0.x, v1.x, v2.x);
    if min > boxhalfsize.x || max < -boxhalfsize.x {
        return false;
    }

    let (min, max) = find_min_max(v0.y, v1.y, v2.y);
    if min > boxhalfsize.y || max < -boxhalfsize.y {
        return false;
    }

    let (min, max) = find_min_max(v0.z, v1.z, v2.z);
    if min > boxhalfsize.z || max < -boxhalfsize.z {
        return false;
    }

    let normal = e0.cross(&e1);
    if !plane_box_overlap(&normal, &v0, &boxhalfsize) {
        return false;
    }

    true
}

fn voxelize(triangles: &Vec<Triangle>, size: &Vector3<f32>) {
    let hs = size * 0.5;
    for tri in triangles {
        let mut aabb = tri.aabb();

        aabb.min.x = clamp_to_voxel_min(aabb.min.x, size.x);
        aabb.min.y = clamp_to_voxel_min(aabb.min.y, size.y);
        aabb.min.z = clamp_to_voxel_min(aabb.min.z, size.z);
        aabb.max.x = clamp_to_voxel_max(aabb.max.x, size.x);
        aabb.max.y = clamp_to_voxel_max(aabb.max.y, size.y);
        aabb.max.z = clamp_to_voxel_max(aabb.max.z, size.z);

        let range_x = ((aabb.max.x - aabb.min.x) / size.x) as i32;
        let range_y = ((aabb.max.y - aabb.min.y) / size.y) as i32;
        let range_z = ((aabb.max.z - aabb.min.z) / size.z) as i32;

        for iz in 0..range_z {
            let z = aabb.min.z + (iz as f32) * size.z;
            for iy in 0..range_y {
                let y = aabb.min.y + (iy as f32) * size.y;
                for ix in 0..range_x {
                    let x = aabb.min.x + (ix as f32) * size.x;

                    let local_aabb = Aabb {
                        min: Vector3::new(x - hs.x, y - hs.y, z - hs.z),
                        max: Vector3::new(x + hs.x, y + hs.y, z + hs.z),
                    };

                    let halfsize = (local_aabb.max - local_aabb.min) * 0.5;
                    let center = local_aabb.min + halfsize;

                    if triangle_box_overlap(&center, &halfsize, &tri) {
                        println!("{}, {}, {}", x, y, z);
                    }
                }
            }
        }
    }
}

fn main() {
    let tri = Triangle::new(
        Vector3::new(0f32, 0f32, 0f32),
        Vector3::new(0f32, 10f32, 0f32),
        Vector3::new(0f32, 0f32, 10f32),
    );

    voxelize(&vec![tri], &Vector3::new(0.1, 0.1, 0.1));
}
