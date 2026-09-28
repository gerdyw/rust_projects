use std::io::{self, Write};

use crate::color::color;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;
const COLOUR_DEPTH: usize = 255;

pub fn render_output() {
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("{COLOUR_DEPTH}");

    for i in 0..IMAGE_HEIGHT {
        print_progress(i);
        for j in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.0;

            let pixel = color(r, g, b);
            println!("{pixel}");
        }
    }
}

fn print_progress(i: usize) {
    let remaining = IMAGE_HEIGHT - i;
    eprint!("\rScanlines remaining: {:4}", remaining);
    io::stderr().flush().unwrap();
}