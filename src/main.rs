extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rusty-tor")
        .version("0.1.0")
        .author("Oleh Franchuk <0x00.gear@gmail.com>")
        .about("tiny Tor client implementation(in pure Rust)")
        .arg(Arg::with_name("host")
            .required(true)
            .long("host")
            .help("the onion service to reach")
            .takes_value(true)
            .value_name("HOST"))
        .arg(Arg::with_name("no_banner")
                .required(false)
                .long("no_banner")
                .short("n")
                .help("prevent the RustyTor banner from being displayed")
                .takes_value(false))
        .arg(Arg::with_name("verbose")
                .required(false)
                .short("v")
                .long("verbose")
                .help("enable verbose output")
                .takes_value(false)
             )
        .get_matches();

}
