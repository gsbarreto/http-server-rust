use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0;1024];
                stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer).to_string();
                let route = request.split_whitespace().collect::<Vec<&str>>()[1];
                let splitted_route = route.split('/').collect::<Vec<&str>>();
                let first_param = splitted_route[1];
                if first_param == "" {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    continue;
                }
                if splitted_route.len() == 3 && first_param == "echo" {
                    let second_param = splitted_route[2];
                    stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",second_param.len(), second_param).as_bytes()).unwrap();
                    continue;
                }   
                stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
