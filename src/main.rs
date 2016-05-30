extern crate sdl2;

mod renderer;


use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color::RGB;


use renderer::*;

const WINDOW_TITLE: &'static str = "Voxel Experiments";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(WINDOW_TITLE, WIDTH, HEIGHT)
        .fullscreen_desktop()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_logical_size(WIDTH, HEIGHT).ok();
    renderer.set_draw_color(RGB(10, 30, 90));

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB888, WIDTH, HEIGHT)
        .unwrap();

    let texture_update_function = render_pixels(WIDTH as usize, HEIGHT as usize);





    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        texture.with_lock(None, texture_update_function.as_ref()).unwrap();

        renderer.clear();
        renderer.copy(&texture, None, Some(Rect::new(0, 0, WIDTH, HEIGHT)));
        renderer.present();
    }
}
