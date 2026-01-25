use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { handle_client(stream) },
            Err(ref e) => {println!("{:?}", e); continue},
        }
    }
}

fn handle_client(stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
}
