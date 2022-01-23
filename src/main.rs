use std::fs::File;

use tiff::encoder::{colortype, TiffEncoder};

fn mandelbrot(r: f32, i: f32) -> u32 {
    let mut r0: f32 = 0.0;
    let mut i0: f32 = 0.0;
    let mut c: u32 = 0;
    while (r0 * r0 + i0 * i0) < 4.0 {
        if c == 2048 {
            break;
        }
        c += 1;
        let t: f32 = r * r - i * i + r0;
        i0 = 2.0 * r0 * i0 + i0;
        r0 = t;
    }
    return c;
}

fn main() {
    // scan along i = 1.0 from -2 to +2, steps of 0.01
    let mut ms: [[u32; 400]; 400] = [[0; 400]; 400];
    for _i in 0..400 {
        let i: f32 = -2.0 + 0.01 * _i as f32;
        for _r in 0..400 {
            let r: f32 = -2.0 + 0.01 * _r as f32;
            ms[_i][_r] = mandelbrot(r, i)
        }
    }
    // Convert this to a 1D vector
    let data: Vec<u32> = ms.iter().flatten().cloned().collect();

    let mut file = File::create(r"test.tiff").unwrap();
    let mut tiff = TiffEncoder::new(&mut file).unwrap();
    tiff.write_image::<colortype::Gray32>(400, 400, &data)
        .unwrap();

    print_block(0, 0, 30, 30, 400, 400, &ms);
}

/// Print out a block of data as an ascii table
fn print_block(
    x: usize,
    y: usize,
    view_width: usize,
    view_height: usize,
    image_w: usize,
    image_h: usize,
    data: &[[u32; 400]],
) {
    let left: usize = std::cmp::max(x, 0);
    let right: usize = std::cmp::min(x + view_width, image_w);
    let top: usize = std::cmp::max(y, 0);
    let bottom: usize = std::cmp::min(y + view_height, image_h);

    print!("     ");
    for x in left..right {
        print!("{x:4} ")
    }
    print!("\n   ┌─{}", "─".repeat(view_width as usize * 5));
    println!();
    for x in left..right {
        print!("{x:2} │ ");
        for y in top..bottom {
            print!("{:4} ", data[y][x]);
        }
        println!();
    }
}
