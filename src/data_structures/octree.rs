

use cgmath::InnerSpace;
use data_structures::voxel::VoxelData;

use data_structures::geom::{Vector, BoundingBox, Ray};

const DEFAULT_BUCKET_SIZE: usize = 8;

#[derive(Debug)]
pub struct Octree {
    pub bounding_box: BoundingBox,
    bucket_size: usize,
    octants: Option<[Box<Octree>; 8]>,
    bucket: Vec<VoxelData>,
}


impl Octree {
    pub fn new(center: Vector, scale: f64) -> Octree {
        Octree::new_with(center, scale, scale, scale, DEFAULT_BUCKET_SIZE)
    }

    pub fn new_with(center: Vector,
                    width: f64,
                    height: f64,
                    depth: f64,
                    bucket_size: usize)
                    -> Octree {
        Octree {
            bounding_box: BoundingBox::new_from_center(center, width, height, depth),
            bucket_size: bucket_size,
            octants: None,
            bucket: Vec::with_capacity(bucket_size),
        }
    }

    pub fn add_voxel(&mut self, voxel: VoxelData) {
        // TODO do nothing if outside of bounding box
        if self.bucket.len() < self.bucket.capacity() {
            self.bucket.push(voxel);
            return;
        } else {
            if self.octants.is_none() {
                self.grow()
            }
            // TODO add in right octant, also clean up bucket ?
        }
    }

    pub fn raycast(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> Option<&VoxelData> {
        if self.bounding_box.intersect_ray(ray, min_distance, max_distance) {
            let result = self.bucket
                .iter()
                .filter(|data| data.bounding_box.intersect_ray(ray, min_distance, max_distance))
                .min_by_key(|data| {
                    (data.bounding_box.center() - ray.get_origin()).magnitude2() as u32
                });
            if result.is_none() && self.octants.is_some() {
                self.octants
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter_map(|octant| octant.raycast(ray, min_distance, max_distance))
                    .min_by_key(|data| {
                        (data.bounding_box.center() - ray.get_origin()).magnitude2() as u32
                    })
            } else {
                return result;
            }
        } else {
            None
        }
    }
}


// Private functions
impl Octree {
    fn grow(&mut self) {
        assert!(self.octants.is_none());

        let half_size = self.bounding_box.size() / 2.0;
        let (w2, h2, d2) = (half_size.x / 2.0, half_size.y / 2.0, half_size.z / 2.0);
        let c = self.bounding_box.center();

        self.octants = Some([Box::new(Octree::new_with(Vector::new(c.x - w2, c.y + h2, c.z + d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector::new(c.x + w2, c.y + h2, c.z + d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector::new(c.x - w2, c.y + h2, c.z - d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector::new(c.x + w2, c.y + h2, c.z - d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector::new(c.x - w2, c.y - h2, c.z + d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector::new(c.x + w2, c.y - h2, c.z + d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector::new(c.x - w2, c.y - h2, c.z - d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector::new(c.x + w2, c.y - h2, c.z - d2),
                                                       half_size.x,
                                                       half_size.y,
                                                       half_size.z,
                                                       self.bucket_size))])
    }
}
