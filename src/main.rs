extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("rustradio")
        .version("0.1.0")
        .author("Yuval Adam <_@yuv.al>")
        .about("RustRadio SDR framework")
        .get_matches();

    println!("\n::: Starting RustRadio");
}
