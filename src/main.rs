extern crate sdl2;
extern crate sdl2_sys;
extern crate cgmath;
extern crate libc;
extern crate rand;

mod renderer;
mod data_structures;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color::RGB;

use std::time::Instant;

use renderer::*;
use data_structures::*;
use data_structures::geom::*;

const WINDOW_TITLE: &'static str = "Voxel Experiments";



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(WINDOW_TITLE, WIDTH as u32, HEIGHT as u32)
        .fullscreen_desktop()
        .build()
        .unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();

    renderer.set_logical_size(WIDTH as u32, HEIGHT as u32).ok();
    renderer.set_draw_color(RGB(10, 30, 30));
    renderer.clear();

    sdl_context.mouse().set_relative_mouse_mode(true);

    println!("Initialized window at {},{}", WIDTH, HEIGHT);

    let mut camera = Camera::new(Vector::new(4.0, 0.0, 1.0), 1, 1, 0.5);
    let mut raycaster_renderer = RaycasterRenderer::new(&renderer);


    // DATA

    let mut octree = Octree::new(zero3(), 100.0);

    octree.add_voxel(VoxelData::new(Vector::new(2.5, 0.5, 5.5), 1.0, 1.0, 1.0, 0xFF_FF_00_00));
    octree.add_voxel(VoxelData::new(Vector::new(3.5, 4.5, 11.0), 1.0, 3.0, 2.0, 0xFF_00_FF_00));
    octree.add_voxel(VoxelData::new(Vector::new(5.5, 2.0, 7.0), 2.0, 2.0, 2.0, 0xFF_00_00_FF));

    octree.add_voxel(VoxelData::new(Vector::new(4.0, 0.0, -3.0), 2.0, 2.0, 2.0, 0xFF_00_FF_FF));
    octree.add_voxel(VoxelData::new(Vector::new(4.5, 4.0, 1.0), 2.0, 2.0, 2.0, 0xFF_FF_00_FF));
    octree.add_voxel(VoxelData::new(Vector::new(4.5, -4.0, 1.0), 2.0, 2.0, 2.0, 0xFF_FF_FF_00));
    // _DATA


    let mut event_pump = sdl_context.event_pump().unwrap();


    let mut last_frame = Instant::now();
    let mut frames: usize = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    camera.position -= camera.right_for(0.1)
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    camera.position -= camera.direction_for(0.1)
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    camera.position += camera.direction_for(0.1)
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    camera.position += camera.right_for(0.1)
                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    camera.position += Vector::new(0.0, -0.1, 0.0)
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    camera.position += Vector::new(0.0, 0.1, 0.0)
                }
                Event::MouseMotion { xrel, yrel, .. } => camera.update_with_mouse(xrel, yrel),
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        raycaster_renderer.render_frame(&octree, &camera);

        frames += 1;
        if last_frame.elapsed().as_secs() > 0 {
            println!("FPS {}", frames);
            last_frame = Instant::now();
            frames = 0;
        }
    }
}
