use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

const MIN_BOUND_X: f64 = -2.25;
const MIN_BOUND_Y: f64 = -1.625;
const MAX_BOUND_X: f64 = 1.0;
const MAX_BOUND_Y: f64 = 1.625;

const MAX_ITERATIONS: usize = 1024;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

fn rgb_to_int(r: u32, g: u32, b: u32) -> u32 {
    ((r * 256) + g) * 256 + b
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut precomputed_strengths: Vec<f64> = vec![0.0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "paws at u",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(48);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let norm_x = x as f64 / WIDTH as f64;
            let norm_y = y as f64 / HEIGHT as f64;
            
            let transformed_x = lerp(MIN_BOUND_X, MAX_BOUND_X, norm_x);
            let transformed_y = lerp(MIN_BOUND_Y, MAX_BOUND_Y, norm_y);

            let mut check_x: f64 = 0.0;
            let mut check_y: f64 = 0.0;

            let mut iter_count = 0;

            while check_x*check_x + check_y*check_y <= 4.0 && iter_count < MAX_ITERATIONS {
                let new_x = check_x*check_x - check_y*check_y + transformed_x;
                check_y = 2.0*check_x*check_y + transformed_y;
                check_x = new_x;

                iter_count += 1;
            }

            let strength = ((iter_count + 1) as f64) / (MAX_ITERATIONS as f64);
            let modified_strength = strength.powf(0.222).clamp(0.0, 1.0);

            precomputed_strengths[y*WIDTH+x] = modified_strength;
        }
    }

    let mut time: f64 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        time += 0.1;

        if window.is_key_down(Key::Space) {
            time = 0.0;
        }

        for i in 0..buffer.len() {
            buffer[i] = {
                let t = precomputed_strengths[i];

                if t >= 1.0 {
                    16777215
                } else {
                    let t = (t * (time.sin() + 2.0) * 0.5).clamp(0.0, 1.0);

                    rgb_to_int(
                        lerp(0.0, 255.0, t).round() as u32, 
                        lerp(0.0, 144.0, t).round() as u32, 
                        lerp(139.0, 0.0, t).round() as u32
                    )
                }
            };
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
