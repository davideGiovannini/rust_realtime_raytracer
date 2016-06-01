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

// // compute the near and far intersections of the cube (stored in the x and y components) using the slab method
// // no intersection means vec.x > vec.y (really tNear > tFar)
// vec2 intersectAABB(vec3 rayOrigin, vec3 rayDir, vec3 boxMin, vec3 boxMax) {
//     vec3 tMin = (boxMin - rayOrigin) / rayDir;
//     vec3 tMax = (boxMax - rayOrigin) / rayDir;
//     vec3 t1 = min(tMin, tMax);
//     vec3 t2 = max(tMin, tMax);
//     float tNear = max(max(t1.x, t1.y), t1.z);
//     float tFar = min(min(t2.x, t2.y), t2.z);
//     return vec2(tNear, tFar);
// };

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
