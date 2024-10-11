extern crate sdl2;

use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod complex;
use complex::{Complex, C32};


const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MAX_ITER: usize = 768;
const EXPLODE_VAL: f32 = 4.0;
const NUM_THREADS: usize = 4;


// Calculate if it in given value in n. iterations explodes
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

// Just give a SDL Color based on number iteration for a specific pixel
fn mandelbrot_color(iter: usize) -> Color {
    if iter == MAX_ITER {
        Color::RGB(0, 0, 0) // Colore nero per i punti interni
    } else {
        // let r_raw= min(iter, 255);
        // let r = ((r_raw * 5) % 256) as u8; // Colori graduali per i punti esterni
        // // let r = ((iter * 16) % 255) as u8;; // Colori graduali per i punti esterni
        // let g_raw = min(255, max(256,iter) - 256);
        // let g = ((g_raw * 7) % 256) as u8;
        // // let g = r;
        // let b = (iter * 9 % 256) as u8;
        // // let b = 255;
        // Color::RGB(r, g, b)
        let val = ((iter * 16) % 255) as u8; // Colori graduali per i punti esterni
        Color::RGB(val, val, 255)
    }
}


#[derive(Copy, Clone)]
struct MandelbrotSDLPoint {
    pub window_coord: (i32, i32),
    pub complex_value: Complex,
    pub sdl_color: Color,
}


// Generate n. values equidistant from each other
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
    let mut to_draw = true;
    let (x_start, x_end) = (-2.0_f32, 0.5_f32);
    let (y_start, y_end) = (-1.00_f32, 1.00_f32);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Mandelbrot", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    // Start Time
    let start = Instant::now();
    let mut cycles: i64 = 0;

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

        // Calculate points' color
        if to_draw {
            canvas.clear();
            let mut threads = vec![];
            let mut mandelbrot_points = Arc::new(Mutex::new(Vec::with_capacity(WIDTH * HEIGHT)));
            let y_step = (y_end - y_start) / HEIGHT as f32;
            let lines_per_thread = HEIGHT / NUM_THREADS;
            let x_points = linear_space(x_start, x_end, WIDTH);
            let shared_x_points = Arc::new(x_points);

            for i in 0..NUM_THREADS {
                let y = y_start + (i as f32 * y_step * lines_per_thread as f32);
                let shared_x_point_copy = Arc::clone(&shared_x_points);
                // let mut mandelbrot_points_copy = Arc::clone(&mandelbrot_points);
                let mut mandelbrot_points_copy = mandelbrot_points.clone();
                threads.push(thread::spawn(move || {
                    let y_points = linear_space(y, y + y_step * lines_per_thread as f32, lines_per_thread);
                    for x in 0..WIDTH {
                        for y in 0..lines_per_thread {
                            let iters = mandelbrot(Complex::new(shared_x_point_copy[x], y_points[y]));
                            let color = mandelbrot_color(iters);
                            let mut data_locked = mandelbrot_points_copy.lock().unwrap();
                            data_locked.push(MandelbrotSDLPoint {
                                window_coord: (x as i32, ((i * lines_per_thread) + y) as i32),
                                complex_value: Complex::new(shared_x_point_copy[x], y_points[y]),
                                sdl_color: color,
                            });
                        }
                    }
                }));
            }

            for t in threads {
                t.join().unwrap();
            }
            for mp in &*mandelbrot_points.lock().unwrap() {
                canvas.set_draw_color(mp.sdl_color);
                canvas.draw_point(mp.window_coord)?;
            }
            canvas.present();
            to_draw = false;
        }
        cycles += 1;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    let elapsed = start.elapsed();
    println!("Elpased time: {:?}", elapsed);
    println!("Cycles: {}", cycles);
    println!("Cycles/s: {:.4}", cycles as f64 / elapsed.as_secs_f64());
    Ok(())
}