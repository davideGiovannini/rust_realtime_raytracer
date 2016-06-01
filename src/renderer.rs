use data_structures::geom::*;


use sdl2_sys::render;
use sdl2::render::Texture;
use std::ptr;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;


pub fn render_pixels() -> Box<Fn(&mut [u8], usize) -> ()> {

    let mut ray_matrix: Vec<Vec<Ray>> = Vec::with_capacity(HEIGHT);
    for y in 0..HEIGHT {
        ray_matrix.push(Vec::with_capacity(WIDTH));
        for x in 0..WIDTH {
            let cx = (x as f64 / WIDTH as f64) * 8.0;
            let cy = (y as f64 / HEIGHT as f64) * 6.0;
            ray_matrix[y].push(Ray::new(Vector::new(cx + 1.0, 2.0 + cy, -2.0),
                                        Vector::new(-0.3, -0.3, 0.3)));
        }
    }
    let ray_matrix = ray_matrix;

    Box::new(move |buffer: &mut [u8], pitch: usize| {
        let channels = pitch / WIDTH;

        let bbox = BoundingBox::new(Vector::new(0.0, 0.0, 0.0), Vector::new(1.0, 1.0, 1.0));

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let offset = y * pitch + x * channels;

                if bbox.intersect_ray(&ray_matrix[y][x], 0.0, 100.0) {
                    buffer[offset + 2] = 0;
                    buffer[offset + 1] = 128;
                    buffer[offset] = 255;
                } else {
                    buffer[offset + 2] = 255;
                    buffer[offset + 1] = 0;
                    buffer[offset] = 155;
                }
            }
        }
    })
}


pub fn unsafe_render(texture: &Texture, bbox: &BoundingBox, ray_matrix: &Vec<Vec<Ray>>) {

    unsafe {
        let sdl_texture = texture.raw();

        let mut pixels = ptr::null_mut();
        let mut pitch = 0;

        render::SDL_LockTexture(sdl_texture, ptr::null(), &mut pixels, &mut pitch);

        let mut pixel_color: u32;

        for row in 0..HEIGHT {
            let mut dst = pixels.offset(row as isize * pitch as isize);

            for col in 0..WIDTH {

                if bbox.intersect_ray(&ray_matrix[row][col], 0.0, 100.0) {
                    pixel_color = 0xFF000000 | (255 << 16) | (0 << 8) | 125 << 0;
                } else {
                    pixel_color = 0xFF000000 | (0 << 16) | (228 << 8) | 155 << 0;
                }

                ptr::write(dst as *mut u32, pixel_color);
                dst = dst.offset(4);
            }
        }
        render::SDL_UnlockTexture(sdl_texture);
    }
}
