use std::{fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}, thread, env};

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer).to_string();
    let route = request.split_whitespace().collect::<Vec<&str>>()[1];
    
    let splitted_route = route.split('/').collect::<Vec<&str>>();
    let first_param = splitted_route[1];

    let args: Vec<String> = env::args().collect();
    let index_directory_arg = args.iter().position(|x| x.contains("--directory"));
    let mut directory_path = "/";
    match index_directory_arg {
        Some(index_directory_arg) => directory_path = &args[index_directory_arg+1],
        None => {}
    }

    match first_param {
        "" => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(),
        "files" => {
            let filename = splitted_route[2];
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
