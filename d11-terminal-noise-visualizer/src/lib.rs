use std::io;

use crossterm::{
    queue,
    style::{Color, PrintStyledContent, Stylize},
};

pub fn closest_terminal_gray(value: f32) -> Color {
    let normal = (value + 1.) / 2.;
    let value = (255. * normal) as u8;

    Color::Rgb {
        r: value,
        g: value,
        b: value,
    }
}

pub fn draw_noise(out: &mut impl io::Write, screen: (u16, u16), scale: u8) -> io::Result<()> {
    let (cols, rows) = screen;
    let step = 1. / 2f32.powi(scale.max(1) as i32);

    for y in 0..rows {
        let fy = step * (y as f32);
        for x in 0..cols {
            let fx = step * (x as f32);
            let noise_value = perlin_noise::perlin([fx, fy]);
            let noise_color = closest_terminal_gray(noise_value);
            queue!(out, PrintStyledContent(" ".on(noise_color)))?;
        }
    }

    out.flush()
}
