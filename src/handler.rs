use crate::response::{Response, ResponseBody};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    let request = String::from_utf8_lossy(&buf).to_string();
    println!("{request}");

    //response
    let body = fs::read_to_string("./pages/example.html").unwrap();
    let len = body.len();
    let response_body = ResponseBody::building("text/html; charset=UTF-8".to_string(), body, len);
    let response = Response::default_as_200(response_body).format_to_ready();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {}
}
