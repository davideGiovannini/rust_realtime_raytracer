

use sdl2_sys::render;
use sdl2::render::{Renderer, Texture};
use sdl2::pixels::PixelFormatEnum;
use std::ptr;
use data_structures::{Octree, Camera};



pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;



pub struct RaycasterRenderer<'a> {
    renderer: &'a Renderer<'a>,
    framebuffer: Texture,
}


impl<'a> RaycasterRenderer<'a> {
    pub fn new(renderer: &'a Renderer) -> RaycasterRenderer<'a> {
        let texture = renderer.create_texture_streaming(PixelFormatEnum::ARGB8888, WIDTH as u32, HEIGHT as u32)
            .unwrap();
        RaycasterRenderer {
            renderer: renderer,
            framebuffer: texture,
        }
    }

    pub fn render_frame(&mut self, octree: &Octree, camera: &Camera) {

        unsafe {
            let sdl_texture = self.framebuffer.raw();
            let sdl_renderer = self.renderer.raw();

            let mut pixels = ptr::null_mut();
            let mut pitch = 0;

            render::SDL_LockTexture(sdl_texture, ptr::null(), &mut pixels, &mut pitch);

            let mut pixel_color: u32;

            for row in 0..HEIGHT {
                let mut dst = pixels.offset(((HEIGHT - 1) - row) as isize * pitch as isize);

                for col in 0..WIDTH {

                    if let Some(data) = octree.raycast(&camera.ray_for(col as f64 / WIDTH as f64,
                                                                       row as f64 /
                                                                       HEIGHT as f64),
                                                       0.0,
                                                       100.0) {
                        pixel_color = data.color;
                    } else {
                        pixel_color = 0xFF101010;//| (0 << 16) | (228 << 8) | 155 << 0;
                    }

                    ptr::write(dst as *mut u32, pixel_color);
                    dst = dst.offset(4);
                }
            }
            render::SDL_UnlockTexture(sdl_texture);

            render::SDL_RenderClear(sdl_renderer);
            // Copy frame from buffer
            // //Render frame
            render::SDL_RenderCopy(sdl_renderer, sdl_texture, ptr::null(), ptr::null());
            // gStreamingTexture.render( ( SCREEN_WIDTH - gStreamingTexture.getWidth() ) / 2, ( SCREEN_HEIGHT - gStreamingTexture.getHeight() ) / 2 );
            // //Update screen
            render::SDL_RenderPresent(sdl_renderer);
        }
    }
}
