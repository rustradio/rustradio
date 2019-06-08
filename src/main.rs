extern crate clap;

mod sources;

use clap::{App};

fn main() {
    let _matches = App::new("rustradio")
        .version("0.1.0")
        .author("Yuval Adam <_@yuv.al>")
        .about("RustRadio SDR framework")
        .get_matches();

    println!("\n::: Starting RustRadio");

    let mut s = sources::rtlsdr::RtlSdrSink::new(123_456_789);

    s.reader.read_async(4, 32768, |bytes| {
        println!("i[0] = {}", bytes[0]);
        println!("q[0] = {}", bytes[1]);
    }).unwrap();
}
