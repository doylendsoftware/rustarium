use util::array2string2;
use local_ip;
use std::net::{UdpSocket, SocketAddr};
use constants::BUFFER_SIZE;

pub struct Socket {
    socket: UdpSocket,
}

impl Socket {
    pub fn new(port: u16) -> Self {
        Self {
            socket: UdpSocket::bind((local_ip::get().unwrap(),port)).unwrap()
        }
    }
    
    pub fn get(&self) -> Option<(SocketAddr,String)> {
        let mut buffer = [0; BUFFER_SIZE];
        
        match self.socket.recv_from(&mut buffer) {
            Ok((amount,source)) => Some((source,array2string2(&buffer,amount))),
            Err(_) => None,
        }
    }
    
    pub fn send(&mut self, address: &SocketAddr, message: String) {
        self.socket.send_to(message.as_bytes(),address).expect("Couldn't send data");
    }
}