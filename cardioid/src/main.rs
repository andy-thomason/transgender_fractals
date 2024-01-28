/// Plot a newton fractal in transgender flag colours.

/// Dimensions of the image.
const XDIM : usize = 512;
const YDIM : usize = 512;

/// Affine transform from pixels to complex numbers.
const SCALE : f64 = 3.0 / YDIM as f64;
const XOFFSET : f64 = SCALE * (- ((XDIM/2) as f64));
const YOFFSET : f64 = SCALE * (- ((YDIM/2+YDIM/16) as f64));

/// https://en.wikipedia.org/wiki/Transgender_flag#/media/File:Transgender_Pride_flag.svg
const TRANSGENDER_FLAG_COLOURS : [[u8; 4]; 3] = [
    [0x5b, 0xce, 0xfa, 0xff],
    [0xff, 0xff, 0xff, 0xff],
    [0xf5, 0xa9, 0xb8, 0xff],
];

const FILENAME : &str = "results/cardioid.png";

fn main() {
    let mut pixels = vec![0_u8; XDIM * YDIM * 4];

    // see. https://mathworld.wolfram.com/HeartCurve.html
    let f = |x : f64, y : f64| (x*x + y*y - 1.0).powi(3) - 1.5*x*x*y*y*y;

    for iy in 0..YDIM {
        for ix in 0..XDIM {
            let x = ix as f64 * SCALE + XOFFSET;
            let y = iy as f64 * SCALE + YOFFSET;
            let val = f(x, y);
            let ival = if val < 0.0 {
                (-20.0*val).floor() as usize + 2
            } else {
                1
            };
            // if iy == 128 { eprint!("{val:3} "); }
            let colour = TRANSGENDER_FLAG_COLOURS[ival % 3];

            let pos = (YDIM-1-iy) * YDIM * 4 + ix * 4;
            pixels[pos..pos+4].copy_from_slice(colour.as_slice());
        }
        // eprintln!();
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
