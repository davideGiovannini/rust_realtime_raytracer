use cgmath::{Rotation3, Rotation, InnerSpace, Rad};
use cgmath::Quaternion;
use data_structures::geom::{Vector, Ray};


#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub position: Vector,
    sensitivity: f64,
    width: usize,
    height: usize,
    pitch: f64,
    yaw: f64,
    quaternion: Quaternion<f64>,
}



impl Camera {
    pub fn new(position: Vector, width: usize, height: usize, sensitivity: f64) -> Camera {
        Camera {
            position: position,
            width: width,
            height: height,
            pitch: 30.0,
            yaw: 120.0,
            sensitivity: sensitivity,
            quaternion: Quaternion::one(),
        }
    }

    pub fn update_with_mouse(&mut self, xoffs: i32, yoffs: i32) {
        self.yaw = (self.yaw + (xoffs as f64) * self.sensitivity) % 360.0;
        self.pitch += (yoffs as f64) * self.sensitivity;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.quaternion = Quaternion::from_euler(Rad::new(self.yaw.to_radians()),
                                                 Rad::new(0.0),
                                                 Rad::new(self.pitch.to_radians()));
    }

    pub fn right(&self) -> Vector {
        self.quaternion.rotate_vector(Vector::new(1.0, 0.0, 0.0))
    }

    pub fn right_for(&self, value: f64) -> Vector {
        self.quaternion.rotate_vector(Vector::new(value, 0.0, 0.0))
    }
    pub fn direction(&self) -> Vector {
        self.quaternion.rotate_vector(Vector::new(0.0, 0.0, 1.0))
    }

    pub fn direction_for(&self, value: f64) -> Vector {
        self.quaternion.rotate_vector(Vector::new(0.0, 0.0, value))
    }


    pub fn ray_for(&self, x: f64, y: f64) -> Ray {
        let w = self.width as f64;
        let h = self.height as f64;
        let cx = x * w - (w / 2.0);
        let cy = y * h - (h / 2.0);

        Ray::new(self.position,
                 self.quaternion
                     .rotate_vector(Vector::new(cx, cy, 1.0))
                     .normalize())
    }
}
