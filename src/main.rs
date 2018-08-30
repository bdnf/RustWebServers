use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

fn handle_connection(mut stream: TcpStream) {
 let mut buffer_for_request_data = [0; 512];
 stream.read(&mut buffer_for_request_data).unwrap();
 
 let mut file = File::open("./serve.html").unwrap();
 let mut contents = String::new();
 file.read_to_string(&mut contents).unwrap();

 //String::from_utf8_lossy function takes a &[u8] and produces a String from it.
 //println!("Request: {}", String::from_utf8_lossy(&buffer_for_request_data[..]));

 //example response
 //let response = "HTTP/1.1 200 OK\r\n\r\n";
 let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

 stream.write(response.as_bytes()).unwrap();
 stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
     let stream = stream.unwrap();

     handle_connection(stream);
    }
}
