use data_structures::voxel::VoxelData;

use data_structures::geom::{Vector, BoundingBox};

#[allow(dead_code)]
const DEFAULT_BUCKET_SIZE: usize = 8;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Octree {
    bounding_box: BoundingBox,
    bucket_size: usize,
    octants: Option<[Box<Octree>; 8]>,
    bucket: Vec<VoxelData>,
}


#[allow(dead_code)]
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

        }

    }
}


// Private functions
#[allow(dead_code)]
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
