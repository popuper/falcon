use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::response::Response;

pub fn handle(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    let cow = String::from_utf8_lossy(&buf).to_string();
    println!("{cow}");
    let response = fs::read_to_string("hello.html").unwrap();
    let response = Response::default_as_200(response).format_to_ready();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}