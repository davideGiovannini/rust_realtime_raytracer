extern crate sdl2;
extern crate sdl2_sys;
extern crate cgmath;
extern crate libc;

mod renderer;
mod data_structures;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color::RGB;

use std::time::Instant;

use renderer::*;
use data_structures::geom::*;

const WINDOW_TITLE: &'static str = "Voxel Experiments";


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(WINDOW_TITLE, WIDTH as u32, HEIGHT as u32)
        .fullscreen_desktop()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_logical_size(WIDTH as u32, HEIGHT as u32).ok();
    renderer.set_draw_color(RGB(10, 30, 90));
    renderer.clear();


    println!("Initialized window at {},{}", WIDTH, HEIGHT);


    let mut raycaster_renderer = RaycasterRenderer::new(&renderer);


    // DATA

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
    let bbox = BoundingBox::new_from2points(Vector::new(0.0, 0.0, 0.0), Vector::new(1.0, 1.0, 1.0));

    // _DATA


    let mut event_pump = sdl_context.event_pump().unwrap();


    let mut last_frame = Instant::now();
    let mut frames: usize = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        raycaster_renderer.render_frame(&bbox, &ray_matrix);

        frames += 1;
        if last_frame.elapsed().as_secs() > 0 {
            println!("FPS {}", frames);
            last_frame = Instant::now();
            frames = 0;
        }
    }
}
