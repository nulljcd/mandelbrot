use image::{self, ExtendedColorType, ImageError, save_buffer};
use num::Complex;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use std::env;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z: Complex<f64> = Complex { re: 0., im: 0. };

    for i in 0..limit {
        if z.norm_sqr() > 4. {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn render(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), ImageError> {
    let mut values: Vec<u8> = vec![0; bounds.0 * bounds.1];

    values
        .par_chunks_mut(bounds.0)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..bounds.0 {
                let point = pixel_to_point(bounds, (x, y), upper_left, lower_right);

                row[x] = match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
            }
        });

    save_buffer(
        "output.png",
        &values,
        bounds.0 as u32,
        bounds.1 as u32,
        ExtendedColorType::L8,
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let bounds: (usize, usize) = (4096, 4096);

    let upper_left: Complex<f64> = Complex {
        re: args[1].parse::<f64>().unwrap(),
        im: args[2].parse::<f64>().unwrap(),
    };

    let lower_right: Complex<f64> = Complex {
        re: args[1].parse::<f64>().unwrap() + args[3].parse::<f64>().unwrap(),
        im: args[2].parse::<f64>().unwrap() - args[3].parse::<f64>().unwrap(),
    };

    render(bounds, upper_left, lower_right).unwrap();
}
