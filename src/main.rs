use crate::handler::handle;
use crate::pool::Pool;
use std::net::TcpListener;

mod handler;
mod pager;
mod pool;
mod request;
mod response;
mod responser;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = Pool::new(10);
    for stream in listener.incoming() {
        pool.run(|| handle(stream.unwrap()))
    }
}
