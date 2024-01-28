/// Plot a newton fractal in transgender flag colours.

use num_complex::{Complex64, ComplexFloat};


/// Dimensions of the image.
const XDIM : usize = 256;
const YDIM : usize = 256;

/// Affine transform from pixels to complex numbers.
const SCALE : f64 = 2.0 / YDIM as f64;
const XOFFSET : f64 = SCALE * (- (XDIM as f64)) / 2.0;
const YOFFSET : f64 = SCALE * (- (YDIM as f64)) / 2.0;

/// https://en.wikipedia.org/wiki/Transgender_flag#/media/File:Transgender_Pride_flag.svg
const TRANSGENDER_FLAG_COLOURS : [[u8; 4]; 3] = [
    [0x5b, 0xce, 0xfa, 0xff],
    [0xff, 0xff, 0xff, 0xff],
    [0xf5, 0xa9, 0xb8, 0xff],
];

const PLOT_MODE : usize = 1;
const FILENAME : &str = "results/plot_time.png";

fn main() {
    let mut pixels = vec![0_u8; XDIM * YDIM * 4];

    // Solve this function, starting at various values of z.
    let f = |z| z * z * z - 1.0;

    // This is the derivative df/dz.
    let fdash = |z| 3.0 * z * z;

    for y in 0..YDIM {
        for x in 0..XDIM {
            let mut z = Complex64::new(
                x as f64 * SCALE + XOFFSET,
                y as f64 * SCALE + YOFFSET
            );

            // Loop until we are very close to one of the solutions
            let mut t = 0;
            for _ in 0..100 {
                // https://en.wikipedia.org/wiki/Newton%27s_method
                // newton raphson: z[next] = z - f(z)/f'(z)
                // eprintln!("{t} {z}");
                let err = f(z) / fdash(z);
                if err.abs() < 1e-10 {
                    break;
                }
                z = z - err;
                t += 1;
            }
            let colour = match PLOT_MODE {
                0 => {
                    match (z.re(), z.im()) {
                        (re, _) if re > 0.0 => TRANSGENDER_FLAG_COLOURS[0],
                        (re, im) if re < 0.0 && im < 0.0 => TRANSGENDER_FLAG_COLOURS[1],
                        (_, _) => TRANSGENDER_FLAG_COLOURS[2],
                    }
                }
                _ => {
                    TRANSGENDER_FLAG_COLOURS[t % 3]
                }
            };
            let pos = y * YDIM * 4 + x * 4;
            pixels[pos..pos+4].copy_from_slice(colour.as_slice());
        }
    }

    let mut buf = Vec::new();
    let mut encoder = png::Encoder::new(&mut buf, XDIM as u32, YDIM as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header().unwrap()
        .write_image_data(&pixels).unwrap();
    std::fs::write(FILENAME, &buf).unwrap();
}
