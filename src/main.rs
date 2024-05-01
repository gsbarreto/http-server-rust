use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer).to_string();
    let route = request.split_whitespace().collect::<Vec<&str>>()[1];
    
    let splitted_route = route.split('/').collect::<Vec<&str>>();
    let first_param = splitted_route[1];
    match first_param {
        "" => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
        "user-agent" => {
            let user_agent_line = request.split("\r\n").filter(|&x| x.contains("User-Agent")).collect::<Vec<&str>>()[0];
            let user_agent = user_agent_line.split(": ").collect::<Vec<&str>>()[1];                   
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",user_agent.len(), user_agent).as_bytes()).unwrap();
        },
        "echo" => {
            if splitted_route.len() == 3 {
                let echo = splitted_route[2];
                stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", echo.len(), echo).as_bytes()).unwrap();
            } else {
                stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap();
            }
        }
        _ => stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap()
    }
}
fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_request(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
