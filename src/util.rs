use std::net::{SocketAddr,IpAddr,Ipv4Addr};
use constants::BUFFER_SIZE;

/**
Create a `SocketAddr` from four octets and a port number.
*/
pub fn build_ipv4_addr(o1: u8, o2: u8, o3: u8, o4: u8, port: u16) -> SocketAddr{

    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(o1, o2, o3, o4)), port)
}


/**
Convert `&[u8]` into a `String`.

You should never need to call this function.
*/
pub fn array2string2(array: &[u8; BUFFER_SIZE], length: usize) -> String {

    let mut result = String::new();
    for counter in 0..length {
        result.push(array[counter] as char);
    }
    result
}