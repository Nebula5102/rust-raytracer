use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static __WIDTH: i32 = 256;
static __HEIGHT: i32 = 256;

fn hello_world() {
    // Create path
    let path = Path::new("first_img.ppm");

    // Display human readable path for printing
    let display = path.display();

    // Open file in write-only mode, returns io::Result<File>
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",display,why),
        Ok(file) => file,
    };
    // Write file parameters 
    let header: String = format!("P3\n{__WIDTH} {__HEIGHT}\n255\n");
    
    let _ = file.write(header.as_bytes());
    for j in 0..__HEIGHT {
        for i in 0..__WIDTH {
            let ii = i as f64;
            let jj = j as f64;
            let r: f64 =  ii / ((__WIDTH as f64) -1.);
            let g: f64 =  jj / ((__HEIGHT as f64) -1.);
            let b = 0.0;

            let ir: i32 = (r * 255.999) as i32;
            let ig: i32 = (g * 255.999) as i32;
            let ib: i32 = (b * 255.999) as i32;
            
            let pixel_string: String = format!("{ir} {ig} {ib}\n");

            let _ = file.write(pixel_string.as_bytes());
        }
    }
}

fn render() {
    hello_world()
}

fn main() {
    render()
}
