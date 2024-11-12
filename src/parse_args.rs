use std::{fs::File, io::{self, BufRead}, path::Path};

use crate::{scan_tcp::TcpScan, scan_udp::UdpScan};

pub fn parse_flag(matches: clap::ArgMatches) {
    let ids = matches.ids();
    let mut ports=Vec::new();
    
    if let Some(port_str) = matches.get_one::<String>("port") {
        ports = parse_port(port_str.clone());
    }
    if ports.len()!=2 {
        println!("Invalid port");
        return
    }


    for id in ids {
        let open=matches.contains_id("openned");
        match id.as_str() {
            "tcp" => {
                if let Some(arg_tcp) = matches.get_one::<String>("tcp") {
                    TcpScan::new(arg_tcp.to_string(),[ports[0],ports[1]] ,open ).to_multi_thread();
                    return;
                }
                println!("Invalid host")
            }
            "udp" => {
                if let Some(arg_udp)=matches.get_one::<String>("udp") {
                    UdpScan::new(arg_udp.to_string(),[ports[0],ports[1]] ,open ).to_multi_thread();
                    return;
                }
                println!("Invalid host")
            }
            _ => {}
        }
    }
}


pub fn parse_port(port: String) -> Vec<u64> {
    if port == "-" {
       
        return vec![1, 65535];
    }

    if port.contains('-') {
       
        let parts: Vec<&str> = port.split('-').collect();
        if parts.len() == 2 {
            let start = parts[0].parse::<u64>();
            let end = parts[1].parse::<u64>();
            match (start, end) {
                (Ok(start), Ok(end)) if start <= end && start <= 65535 && end <= 65535 =>  vec![start, end],
                _ => vec![],
            }
        } else {
            vec![]
        }
    } else {
       
        match port.parse::<u64>() {
            Ok(value) => vec![value, value],
            Err(_) => vec![],               
        }
    }
}



// Function to look up a service name in the /etc/services file
pub fn lookup_service_from_etc_services(port: u16, protocol: &str) -> io::Result<Option<String>> {
    let path = Path::new("/etc/services");
    let file = File::open(path)?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue; // Skip comments and empty lines
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let service_port_proto: Vec<&str> = parts[1].split('/').collect();
            if service_port_proto.len() == 2 {
                let service_port = service_port_proto[0].parse::<u16>().ok();
                let service_protocol = service_port_proto[1];

                if service_port == Some(port) && service_protocol == protocol {
                    return Ok(Some(parts[0].to_string())); // Return the service name
                }
            }
        }
    }

    Ok(None) // No match found
}