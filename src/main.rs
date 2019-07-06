use num::Complex;

fn main() {
    let mut fb = mini_gl_fb::gotta_go_fast("Hello world!", 800.0, 600.0);
    let mut buffer = vec![[0, 0, 0, 255]; 800 * 600];
    
    for x in 0..800 {
        for y in 0..600 {
            let c = Complex::new(x as f64 / 300.0 - 2.0, (y as i16 - 300) as f64 / 250.0);
            let mut z = Complex::new(0.0, 0.0);
            for i in 0..100 {
                if z.norm() > 2.0 {
                    buffer[x+y*800] = [(i as f32 * 10.0) as u8, 0, 0, 255];
                    break;
                }
                z = z*z + c;
            }
        }
    }

    fb.update_buffer(&buffer);
    fb.persist();
}
