
use cgmath::Vector3;


pub type Vector = Vector3<f64>;


#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    min_point: Vector,
    max_point: Vector,
}

impl BoundingBox {
    pub fn new_from2points(min_point: Vector, max_point: Vector) -> BoundingBox {
        BoundingBox {
            min_point: min_point,
            max_point: max_point,
        }
    }
    pub fn new_from_center(center: Vector, width: f64, height: f64, depth: f64) -> BoundingBox {
        let size = Vector::new(width / 2.0, height / 2.0, depth / 2.0);

        BoundingBox {
            min_point: center - size,
            max_point: center + size,
        }
    }

    pub fn center(&self) -> Vector {
        let size = self.size() / 2.0;
        self.min_point + size
    }

    pub fn size(&self) -> Vector {
        self.max_point - self.min_point
    }

    pub fn intersect_ray(&self, ray: &Ray, min_distance: f64, max_distance: f64) -> bool {
        let mut tmin = ((if ray.sign_is_positive[0] {
                self.min_point
            } else {
                self.max_point
            })
            .x - ray.origin.x) * ray.inv_direction.x;
        let mut tmax = ((if ray.sign_is_positive[0] {
                self.max_point
            } else {
                self.min_point
            })
            .x - ray.origin.x) * ray.inv_direction.x;

        let tymin = ((if ray.sign_is_positive[1] {
                self.min_point
            } else {
                self.max_point
            })
            .y - ray.origin.y) * ray.inv_direction.y;
        let tymax = ((if ray.sign_is_positive[1] {
                self.max_point
            } else {
                self.min_point
            })
            .y - ray.origin.y) * ray.inv_direction.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }
        let tzmin = ((if ray.sign_is_positive[2] {
                self.min_point
            } else {
                self.max_point
            })
            .z - ray.origin.z) * ray.inv_direction.z;
        let tzmax = ((if ray.sign_is_positive[2] {
                self.max_point
            } else {
                self.min_point
            })
            .z - ray.origin.z) * ray.inv_direction.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }
        return (tmin < max_distance) && (tmax > min_distance);
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
    inv_direction: Vector,
    sign_is_positive: [bool; 3],
}


impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        let inv_dir = Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        Ray {
            origin: origin,
            direction: direction,
            inv_direction: inv_dir,
            sign_is_positive: [inv_dir.x.is_sign_positive(),
                               inv_dir.y.is_sign_positive(),
                               inv_dir.z.is_sign_positive()],
        }
    }

    pub fn set(&mut self, origin: Vector, direction: Vector) {
        self.origin = origin;
        self.direction = direction;
        let inv_dir = Vector3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        self.inv_direction = inv_dir;

        self.sign_is_positive[0] = inv_dir.x.is_sign_positive();
        self.sign_is_positive[1] = inv_dir.y.is_sign_positive();
        self.sign_is_positive[2] = inv_dir.z.is_sign_positive();
    }
}







#[cfg(test)]
mod test_raycasting {
    use super::*;

    #[test]
    fn test_raycast_hit() {
        let rays = [Ray::new(Vector::new(0.0, 0.0, -2.0), Vector::new(0.0, 0.0, 1.0)),
                    Ray::new(Vector::new(0.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
                    Ray::new(Vector::new(1.0, 0.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
                    Ray::new(Vector::new(-1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0))];
        let bbox = BoundingBox::new_from2points(Vector::new(-0.5, -0.5, -0.5),
                                                Vector::new(0.5, 0.5, 0.5));

        for ray in rays.iter() {
            assert!(bbox.intersect_ray(ray, 0.0, 100.0))
        }
    }

    #[test]
    fn test_raycast_miss() {
        let rays = [Ray::new(Vector::new(0.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0)),
                    Ray::new(Vector::new(-3.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0))];
        let bbox = BoundingBox::new_from2points(Vector::new(-0.5, -0.5, -0.5),
                                                Vector::new(0.5, 0.5, 0.5));
        let bbox2 = BoundingBox::new_from_center(Vector::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0);
        for ray in rays.iter() {
            assert!(bbox.intersect_ray(ray, 0.0, 100.0));
            assert!(bbox2.intersect_ray(ray, 0.0, 100.0))
        }
    }
}
