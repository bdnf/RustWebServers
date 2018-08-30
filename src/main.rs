use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn handle_connection(mut stream: TcpStream) {
 let mut buffer_for_request_data = [0; 512];

 stream.read(&mut buffer_for_request_data).unwrap();

 //String::from_utf8_lossy function takes a &[u8] and produces a String from it.
 println!("Request: {}", String::from_utf8_lossy(&buffer_for_request_data[..]));
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
     let stream = stream.unwrap();

     handle_connection(stream);
    }
}
