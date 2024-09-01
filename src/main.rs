extern crate bmp;
use bmp::Image;
use num_complex::Complex64 as Complex;

fn pixel_to_complex(pixel_x: u32, pixel_y: u32, width: u32, height: u32) -> Complex {
    const X_MIN: f64 = -2.5;
    const DX: f64 = 3.5;
    let dx_per_pixel = DX / width as f64;
    let y = -(pixel_y as f64 - height as f64 / 2.0) * dx_per_pixel;
    let x = pixel_x as f64 * dx_per_pixel + X_MIN;

    Complex::new(x, y)
}

fn steps_until_divergence(initial_value: Complex, shift: Complex) -> Option<u32> {
    let mut z = initial_value;
    for step_count in 0..1000 {
        if z.norm_sqr() > 4.0 {
            return Some(step_count);
        }
        z = z * z + shift;
    }
    None
}

fn divergence_count_to_color(diverged_at: Option<u32>) -> bmp::Pixel {
    match diverged_at {
        None => bmp::consts::BLACK,
        Some(x) if x > 40 => bmp::consts::DARK_RED,
        Some(x) if x > 20 => bmp::consts::RED,
        Some(x) if x > 10 => bmp::consts::GRAY,
        Some(_) => bmp::consts::WHITE,
    }
}

fn main() {
    let initial_value = Complex::new(0.0, 0.0);

    let width = 1920_u32;
    let height = 1080_u32;

    let mut img = Image::new(width.into(), height.into());
    for (pixel_x, pixel_y) in img.coordinates() {
        let shift = pixel_to_complex(pixel_x, pixel_y, width, height);
        let diverged_at = steps_until_divergence(initial_value, shift);
        let color = divergence_count_to_color(diverged_at);
        img.set_pixel(pixel_x, pixel_y, color);
    }
    let _ = img.save("mandelbrot.bmp");
}
