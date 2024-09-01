extern crate bmp;
use bmp::Image;
use num_complex::Complex64 as Complex;

fn pixel_to_complex(pixel_x: u32, pixel_y: u32, width: u32, height: u32) -> Complex {
    const COMPLEX_BOTTOM_LEFT: Complex = Complex::new(-2.0, -2.0);
    const COMPLEX_TOP_RIGHT: Complex = Complex::new(2.0, 2.0);
    let delta = COMPLEX_TOP_RIGHT - COMPLEX_BOTTOM_LEFT;
    let dx_per_pixel = delta.re / width as f64;
    let dy_per_pixel = delta.im / height as f64;
    Complex::new(pixel_x as f64 * dx_per_pixel, pixel_y as f64 * dy_per_pixel) + COMPLEX_BOTTOM_LEFT
}

fn steps_until_divergence(initial_value: Complex, shift: Complex) -> Option<u8> {
    let mut z = initial_value;
    for step_count in 0..100 {
        if z.norm_sqr() > 4.0 {
            return Some(step_count);
        }
        z = z * z + shift;
    }
    None
}

fn main() {
    let initial_value = Complex::new(0.0, 0.0);

    let width = 1920_u32;
    let height = 1080_u32;

    let mut img = Image::new(width.into(), height.into());
    for (pixel_x, pixel_y) in img.coordinates() {
        let shift = pixel_to_complex(pixel_x, pixel_y, width, height);
        let diverged_at = steps_until_divergence(initial_value, shift);
        let color = match diverged_at {
            Some(_) => bmp::consts::WHITE,
            None => bmp::consts::BLACK,
        };
        img.set_pixel(pixel_x, pixel_y, color);
    }
    let _ = img.save("mandelbrot.bmp");
}
