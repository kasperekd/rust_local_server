use std::{
    fs,
    io::{self, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let mut port = 8001;

    loop {
        let address = format!("127.0.0.1:{}", port);
        match TcpListener::bind(&address) {
            Ok(listener) => {
                println!("Server listening on: {}", address);
                for stream in listener.incoming() {
                    let stream = match stream {
                        Ok(stream) => stream,
                        Err(e) => {
                            eprintln!("Failed to accept incoming connection: {}", e);
                            continue;
                        }
                    };
                    handle_connection(stream);
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::AddrInUse {
                    println!("Port {} is already in use. Trying the next port.", port);
                    port += 1;
                    continue;
                } else {
                    eprintln!("Failed to bind to address {}: {}", address, e);
                    break;
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    if let Some(Ok(request_line)) = buf_reader.lines().next() {
        println!("Incoming connection from: {}\t{}", stream.peer_addr().unwrap(), request_line);

        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } 
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        if let Ok(contents) = fs::read_to_string(filename) {
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        } 
        else {
            println!("Error reading file");
        }
    } 
    else {
        println!("Error reading request line");
    }
}