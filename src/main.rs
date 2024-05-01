use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0;1024];
                stream.read(&mut buffer).unwrap();
                let route = std::str::from_utf8(&buffer[5..6]).unwrap();
                if route == " " {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                } else {
                    stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
