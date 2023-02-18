mod vec3;
pub use vec3::{Vec3, Point, Color};

fn main() {

    let r: Vec3 = Point {
        x: 3.0, y: 4.0, z: 0.0
    };
    eprintln!("LFG LENGTH: {}", r.length());
    // image
    static IMAGE_WIDTH: i32 = 256;
    static IMAGE_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        eprintln!("Scanlines remaining: {}", j);
        
        let mut i = 0;
        while i < IMAGE_WIDTH {
            let r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let ir = 255 as f32 * r;
            let ig = 255 as f32 * g;
            let ib = (255 as f32 * b) as i32;

            println!("{} {} {}", ir, ig, ib);

            i += 1;
        }
        j -= 1;
    }
}
