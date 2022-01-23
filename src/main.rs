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
}
