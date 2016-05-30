


pub fn render_pixels(width: usize, height: usize) -> Box<Fn(&mut [u8], usize) -> ()> {
    Box::new(move |buffer: &mut [u8], pitch: usize| {
        let channels = pitch / width;

        for y in 0..height {
            for x in 0..width {
                let offset = y * pitch + x * channels;
                buffer[offset + 1] = 128;
                buffer[offset] = 255;
            }
        }
    })
}
