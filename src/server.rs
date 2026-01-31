use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn create_server(address: &str) {
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let _ = writeln!(stream, "Hello fromm server!").map_err(|err| {
        eprintln!("Could not write to client: {}", err);
    });

    loop {
        let mut buffer = [0; 1024];
        let status = stream.read(&mut buffer).map_err(|e| eprintln!("{:?}", e));
        match status {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                println!("Received: {:?}", String::from_utf8_lossy(&buffer[..n]));
            }
            Err(e) => {
                eprintln!("Read error: {:?}", e);
                break;
            }
        }
    }
}
