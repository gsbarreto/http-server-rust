use std::{collections::HashMap, env, fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

use itertools::Itertools;

fn handle_request(mut stream: TcpStream) {
    let args: Vec<String> = env::args().collect();
    let index_directory_arg = args.iter().position(|x| x.contains("--directory"));
    let mut directory_path = "/";
    match index_directory_arg {
        Some(index_directory_arg) => directory_path = &args[index_directory_arg+1],
        None => {}
    }

    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let http_request = String::from_utf8_lossy(&buffer).to_string();
    let request_lines = http_request.split("\r\n").collect::<Vec<&str>>();
    let main_header = request_lines[0].split_whitespace().collect::<Vec<&str>>();
    let method = main_header[0];
    let endpoint: &str = main_header[1];
    let splitted_endpoint = endpoint.split('/').collect::<Vec<&str>>();
    let first_param = splitted_endpoint[1];
    
    let headers = request_lines[1..].iter().map(|line| {
        let splitted_headers = line.split(": ").collect::<Vec<&str>>();
        if splitted_headers.len() != 2 {
            return ("body".to_string(), splitted_headers[0].to_string().split("\0").join(""));
        }
        Some((splitted_headers[0].to_string(), splitted_headers[1].to_string())).unwrap()
    }).collect::<HashMap<_,_>>();  

    let body = if method == "POST" {
        Some(headers.get("body").unwrap())
    } else {
        None
    };

    match method {
        "GET" => {
            match first_param {
                "" => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
                "files" => {
                    let filename = splitted_endpoint[2];
                    if filename == ""  {
                        stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap();
                        return;
                    }
                    
                    let file = File::open(format!("{directory_path}/{filename}"));
                    match file {
                        Ok(mut file) => {
                            let mut contents = String::new();
                            let _ = file.read_to_string(&mut contents);
                            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",contents.len(), contents).as_bytes()).unwrap();

                        },
                        Err(_) => {
                            stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap();
                        }
                    }                    
                },
                "user-agent" => {
                    let user_agent = headers.get("User-Agent").unwrap();             
                    stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",user_agent.len(), user_agent).as_bytes()).unwrap();
                },
                "echo" => {
                    if splitted_endpoint.len() == 3 {
                        let echo = splitted_endpoint[2];
                        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", echo.len(), echo).as_bytes()).unwrap();
                    } else {
                        stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap();
                    }
                }
                _ => stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap()
            }
        },
        "POST" => {
            match first_param {
                "files" => {
                    let filename = splitted_endpoint[2];
                    if filename == ""  {
                        stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap();
                        return;
                    }
                    
                    let mut file = File::create(format!("{directory_path}/{filename}")).unwrap();
                    file.write_all(body.unwrap().as_bytes()).unwrap();
                    file.flush().unwrap();
                    stream.write_all(b"HTTP/1.1 201 OK\r\n\r\n").unwrap();               
                }
                _ => stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap()
            }
        },
        _ => {
            println!("Unknown request to {}", endpoint);
            stream.write_all(b"HTTP/1.1 404 OK\r\n\r\n").unwrap()
        }
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
