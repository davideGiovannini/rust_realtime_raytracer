use data_structures::voxel::VoxelData;
use cgmath::Vector3;

#[allow(dead_code)]
const DEFAULT_BUCKET_SIZE: usize = 8;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Octree {
    center: Vector3<f64>,
    width: f64,
    height: f64,
    depth: f64,
    bucket_size: usize,
    octants: Option<[Box<Octree>; 8]>,
    bucket: Vec<VoxelData>,
}


#[allow(dead_code)]
impl Octree {
    pub fn new(center: Vector3<f64>, scale: f64) -> Octree {
        Octree::new_with(center, scale, scale, scale, DEFAULT_BUCKET_SIZE)
    }

    pub fn new_with(center: Vector3<f64>,
                    width: f64,
                    height: f64,
                    depth: f64,
                    bucket_size: usize)
                    -> Octree {
        Octree {
            center: center,
            width: width,
            height: height,
            depth: depth,
            bucket_size: bucket_size,
            octants: None,
            bucket: Vec::with_capacity(bucket_size),
        }
    }
}

// Private functions
#[allow(dead_code)]
impl Octree {
    fn grow(&mut self) {
        assert!(self.octants.is_none());

        let (w, h, d) = (self.width / 2.0, self.height / 2.0, self.depth / 2.0);
        let (w2, h2, d2) = (self.width / 4.0, self.height / 4.0, self.depth / 4.0);
        let (x, y, z) = (self.center.x, self.center.y, self.center.z);

        self.octants = Some([Box::new(Octree::new_with(Vector3::new(x - w2, y + h2, z + d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector3::new(x + w2, y + h2, z + d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector3::new(x - w2, y + h2, z - d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector3::new(x + w2, y + h2, z - d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector3::new(x - w2, y - h2, z + d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector3::new(x + w2, y - h2, z + d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector3::new(x - w2, y - h2, z - d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size)),
                             Box::new(Octree::new_with(Vector3::new(x + w2, y - h2, z - d2),
                                                       w,
                                                       h,
                                                       d,
                                                       self.bucket_size))])
    }
}
