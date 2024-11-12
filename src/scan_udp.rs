use std::{
    net::UdpSocket, sync::Arc, thread, time::Duration
};

use crate::parse_args::lookup_service_from_etc_services;

pub struct UdpScan {
    pub host: String,
    pub port: [u64; 2],
    pub openned: bool,
}

impl UdpScan {
    pub fn new(host: String, port: [u64; 2], openned: bool) -> Self {
        Self {
            host,
            port,
            openned,
        }
    }
    pub fn scan(&self,) {
        let mut i = self.port[0];
        let z = self.port[1];

        while i <= z {
            let service = lookup_service_from_etc_services(i as u16, "udp")
                .unwrap_or(Some("Service no fund".to_string()));
            let is_open = self.is_udp_port_open(i as u16);
            if self.openned && is_open {
                println!("host {}:{} is open {}", self.host, i, is_open);
                let _serv = &service.is_some();
                if *_serv {
                    println!(" |_ service {}", service.unwrap())
                }
            } else if !self.openned {
                println!(
                    "host {}:{} is open {} service name {:?}",
                    self.host,
                    i,
                    is_open,
                    service.unwrap_or("not fund".to_string())
                );
            }
            i += 1;
        }
    }

    pub fn to_multi_thread(&self) {
        let mut i = self.port[0];
        let z = self.port[1];
        let cpus = num_cpus::get();
        let diff = z - i;
        let lap = diff / (cpus as u64 - 1);
        let mut handles = vec![];
        
        let host = Arc::new(self.host.clone());
        let openned = self.openned;
    
        while i <= z {
            let host = Arc::clone(&host);
            let start_port = i;
            let end_port = if i + lap >= z { z } else { i + lap };
    
            let handler =thread::spawn(move || {
                let udp_scan = UdpScan {
                    host: host.to_string(),
                    port: [start_port, end_port],
                    openned,
                };
                udp_scan.scan();
            });
            handles.push(handler);

    
            i += if lap>0 {lap}else{1};
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn is_udp_port_open(&self, port: u16) -> bool {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to local address");
        socket
            .set_read_timeout(Some(Duration::from_secs(1)))
            .expect("Failed to set read timeout");

        let message = b"Ping";

        if socket
            .send_to(message, format!("{}:{}", self.host, port))
            .is_err()
        {
            return false;
        }

        let mut buffer = [0; 512];

        match socket.recv_from(&mut buffer) {
            Ok((_, _)) => true,
            Err(_) => false,
        }
    }
}
