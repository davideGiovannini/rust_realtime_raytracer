
use cgmath::Vector3;


#[derive(Debug)]
pub struct VoxelData {
    center: Vector3<f64>,
    width: f64,
    height: f64,
    depth: f64,
}

#[allow(dead_code)]
impl VoxelData {
    fn new(center: Vector3<f64>, width: f64, height: f64, depth: f64) -> VoxelData {
        VoxelData {
            center: center,
            width: width,
            height: height,
            depth: depth,
        }
    }
}
