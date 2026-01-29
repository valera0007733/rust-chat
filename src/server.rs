
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


pub fn send_msg_to_client() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .map_err(|err| {
            eprintln!("Could not connect to server: {}", err);
            err
        })
        .unwrap();

    stream
        .write("Hello!".as_bytes())
        .map_err(|err| {
            eprintln!("Could not write to server: {}", err);
            err
        })
        .unwrap();

    let mut buff = vec![0; 4096];
    println!("{:?}", stream.read(&mut buff));
    println!("{}", String::from_utf8_lossy(&buff));
}

pub fn create_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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
        eprintln!("Could not write to server: {}", err);
    });
}
