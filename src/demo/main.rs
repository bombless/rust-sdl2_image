#![crate_type = "bin"]
#![feature(old_path)]

extern crate sdl2;
extern crate sdl2_image;

use std::env;

mod video;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: ./demo image.[png|jpg]")
    } else {
        video::main(&Path::new(&args[1]));
    }
}
