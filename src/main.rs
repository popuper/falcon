use std::net::TcpListener;
use crate::handler::handle;
use crate::pool::Pool;

mod pool;
mod handler;
mod response;
mod pager;
mod request;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = Pool::new(10);
    for stream in listener.incoming() {
        pool.run(|| { handle(stream.unwrap()) })
    }
}
