
use cgmath::Vector3;
use data_structures::geom::BoundingBox;

#[derive(Debug)]
pub struct VoxelData {
    pub bounding_box: BoundingBox,
    pub color: u32,
}

#[allow(dead_code)]
impl VoxelData {
    pub fn new(center: Vector3<f64>, width: f64, height: f64, depth: f64, color: u32) -> VoxelData {
        VoxelData {
            bounding_box: BoundingBox::new_from_center(center, width, height, depth),
            color: color,
        }
    }
}
