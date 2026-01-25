use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let app_type = args.get(0);
    match app_type {
        Some(_) => { create_server() },
        None => { create_client() },
    }
}

fn create_client() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").map_err(|err| {
        println!("Could not connect to server: {}", err);
        err
    }).unwrap();

    stream.write("Hello!".as_bytes()).map_err(|err| {
        println!("Could not write to server: {}", err);
        err
    }).unwrap();
}

fn create_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(ref e) => {
                println!("{:?}", e);
                continue;
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
}
