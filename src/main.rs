use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use HttpServer::ThreadPool; 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }

    println!("Server shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "<h1>Hello, world</h1>")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "<h1>Not Found</h1>")
    };

    let response = format!("{status_line}{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
