use std::env;

use clap::{command, Arg};
use parse_args::parse_flag;

pub mod error;
pub mod scan_tcp;
mod scan_udp;
pub mod parse_args;


fn main() {
    let matches = command!()
        // .about("Usage: tinyscanner [OPTIONS] [HOST] [PORT]")
        .override_usage("Usage: tinyscanner [OPTIONS] [HOST] [PORT]")
        .arg(
            Arg::new("port")
                .short('p')
                .required(true)
                .help("Range of ports to scan")
        )
        .arg(
            Arg::new("udp")
                .short('u')
                .required(true)
                .help("UDP scan")
                .conflicts_with("tcp")
        )
        .arg(
            Arg::new("tcp")
                .short('t')
                .required(true)
                .help("TCP scan")
                .conflicts_with("udp")
        )
        .arg(
            Arg::new("openned")
                .short('o')
                .long("openned")
                .help("print open port only")
                .num_args(0..=1)
                .require_equals(true)
                .default_missing_value("true")
        )
        .get_matches();

        parse_flag(matches);
    
    }
    
