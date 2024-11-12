use std::{net::TcpStream, process::exit, sync::Arc, thread, time::Duration};

use crate::parse_args::lookup_service_from_etc_services;


pub struct TcpScan{
    pub host: String,
    pub port: [u64;2],
    pub openned: bool,
} 

impl  TcpScan {
    pub fn new(host:String,port:[u64;2],openned: bool)->Self{
        Self { host, port, openned }
    }

    pub fn scan(&self){
        let mut i= self.port[0];
        let z= self.port[1];
        while  i <=z{

            let service= lookup_service_from_etc_services(i as u16, "tcp").unwrap_or(Some("Service no fund".to_string()));
            let is_open=self.is_port_open(i);
            if self.openned && is_open{
                println!("host {}:{} is open {}",self.host,i,is_open);
                let _serv=&service.is_some();
                if  *_serv {
                    println!(" |_ service {}",service.unwrap())
                }

            }else if !self.openned {
                println!("host {}:{} is open {} service name {:?}",self.host,i,is_open,service.unwrap_or("not fund".to_string()));
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
            let handler=thread::spawn(move || {
                let tcp_scan = TcpScan {
                    host: host.to_string(),
                    port: [start_port, end_port],
                    openned,
                };
                tcp_scan.scan();
            });
            handles.push(handler);
            i += lap;
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
    fn is_port_open(&self,port:u64) -> bool {
        let timeout = Duration::from_secs(1);
        let Ok(parsed)= &format!("{}:{}", self.host, port).parse() else{
            println!("Invalid host or Port");
            exit(0)
        };
        TcpStream::connect_timeout(parsed, timeout).is_ok()
    }
}