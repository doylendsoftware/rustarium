use std::net::{SocketAddr,IpAddr,Ipv4Addr};
use constants::BUFFER_SIZE;

pub fn build_ipv4_addr(o1: u8, o2: u8, o3: u8, o4: u8, port: u16) -> SocketAddr{
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port)
}

pub fn array_to_string(array: &[u8; BUFFER_SIZE]) -> String {
    let mut result = String::new();
    let mut index = 0;
    
    while (index < BUFFER_SIZE) && (array[index] != 10) {
        result.push(array[index] as char);
        index += 1;
    }
    
    result
}