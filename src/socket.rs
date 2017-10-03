use util::array2string2;
use local_ip;
use std::net::{UdpSocket, SocketAddr,ToSocketAddrs};
use constants::BUFFER_SIZE;

pub struct Socket {
    /*!
    Wrapper around UdpSocket.
    */
    socket: UdpSocket,
}

impl Socket {
    pub fn new(port: u16) -> Self {
        /*!
        Create a new Socket.
        
        This function binds to the computer's local IP address. To
        bind to a custom address, use `Socket::new_any_ip()`
        */
        Self {
            socket: UdpSocket::bind((local_ip::get().unwrap(),port)).unwrap()
        }
    }
    
    pub fn new_any_ip<A: ToSocketAddrs>(addr: A) -> Self {
        /*!
        Create a new Socket bound to any IP address.
        */
        Self {
            socket: UdpSocket::bind(addr).unwrap()
        }
    }
    
    pub fn get(&self) -> Option<(SocketAddr,String)> {
        /*!
        Get a message, if one is waiting.
        
        Returns the sender's `SocketAddr`, and a `String` containing
        the message text itself.
        
        Returns `None` if there are no waiting messages.
        */
        let mut buffer = [0; BUFFER_SIZE];
        
        match self.socket.recv_from(&mut buffer) {
            Ok((amount,source)) => Some((source,array2string2(&buffer,amount))),
            Err(_) => None,
        }
    }
    
    pub fn send(&mut self, address: &SocketAddr, message: String) {
        /*!
        Send a message.
        */
        self.socket.send_to(message.as_bytes(),address).expect("Couldn't send data");
    }
}