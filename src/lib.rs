extern crate local_ip;

use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub const BUFFER_SIZE: usize = 4096;

pub fn build_ipv4_addr(o1: u8, o2: u8, o3: u8, o4: u8, port: u16) -> SocketAddr{
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port)
}

fn array_to_string(array: &[u8; BUFFER_SIZE]) -> String {
    let mut result = String::new();
    let mut index = 0;
    
    while (index < BUFFER_SIZE) && (array[index] != 10) {
        result.push(array[index] as char);
        index += 1;
    }
    
    result
}

pub fn create_socket(port: u16) -> UdpSocket {
    UdpSocket::bind((local_ip::get().unwrap(),port)).unwrap()
}

pub fn get_message(socket: &UdpSocket) -> Option<(SocketAddr,String)> {
    let mut buffer = [0; BUFFER_SIZE];
    
    match socket.recv_from(&mut buffer) {
        Ok((_,src)) => Some((src,array_to_string(&buffer))),
        Err(_) => None,
    }
}

pub fn sendln(socket: &mut UdpSocket, address: &SocketAddr, mut message: String) {
    message.push('\n');
    socket.send_to(message.as_bytes(), address).expect("Couldn't send data");
}

