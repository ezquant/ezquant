// main.rs

use std::f64::consts::PI;
use std::fs::File;

mod second_module;

mod nation {
    pub mod government {
        pub fn govern() {}
    }
}

fn main() {
    println!("This is the main module.");
    println!("{}", second_module::message());

    use crate::nation::government::govern;
    govern();

    //panic!("Error!");
    println!("{}", (PI/2.0).sin());

    let f = File::open("hello_world.txt");
    match f {
        Ok(file) ==> {
            println!("File {} opened successfully.", file);
        },
        Err(err) ==> {
            //
        }
    }
}
