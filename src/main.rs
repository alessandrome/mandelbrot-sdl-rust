extern crate sdl2;

use std::cmp::{max, min};
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod complex;
use complex::{Complex, C32};


const WIDTH: usize = 1024;
const HEIGHT: usize = 800;
const MAX_ITER: usize = 768;
const EXPLODE_VAL: f32 = 8.0;


fn mandelbrot(c: Complex) -> usize {
    let mut z = Complex { real: 0.0, imaginary: 0.0 };
    for i in 0..MAX_ITER {
        if z.norm_sqrt() > EXPLODE_VAL {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}

fn mandelbrot_color(iter: usize) -> Color {
    if iter == MAX_ITER {
        Color::RGB(0, 0, 0) // Colore nero per i punti interni
    } else {
        let r_raw= min(iter, 255);
        let r = ((r_raw * 5) % 256) as u8; // Colori graduali per i punti esterni
        // let r = ((iter * 16) % 255) as u8;; // Colori graduali per i punti esterni
        let g_raw = min(255, max(256,iter) - 256);
        let g = ((g_raw * 7) % 256) as u8;
        // let g = r;
        let b = (iter * 9 % 256) as u8;
        // let b = 255;
        Color::RGB(r, g, b)
    }
}

fn linear_space(start: f32, end: f32, points: usize) -> Vec<f32> {
    let mut points_vec: Vec<f32> = Vec::with_capacity(points);
    let step = (end - start) / points as f32;
    let mut point= start;
    for p in 0..points {
        points_vec.push(point);
        point += step;
    }
    points_vec
}


pub fn main() -> Result<(), String> {
    let (x_start, x_end) = (-1.5_f32, 0.2_f32);
    let (y_start, y_end) = (-0.5_f32, 0.8_f32);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        let x_points= linear_space(x_start, x_end, WIDTH);
        let y_points= linear_space(y_start, y_end, HEIGHT);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let iters = mandelbrot(Complex::new(x_points[x], y_points[y]));
                let color = mandelbrot_color(iters);
                canvas.set_draw_color(color);
                canvas.draw_point((x as i32, y as i32))?;
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}