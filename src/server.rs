use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

pub fn create_server(address: &str) {
    let listener = TcpListener::bind(address).unwrap();

    let (tx, rx) = channel();
    handle_client_messages(rx);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let thread_tx: Sender<String> = tx.clone();
                thread::spawn(move || handle_client(stream, thread_tx))
            }
            Err(e) => {
                eprintln!("Could not accept connection from client: {:?}", e);
                continue;
            }
        };
    }
}

fn handle_client(mut stream: TcpStream, thread_tx: Sender<String>) {
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
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {:?}", &received);
                let tx_result = thread_tx.send(received.to_string());
                match tx_result {
                    Err(e) => {
                        eprintln!("Could not send message to main thread: {:?}", e);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Read error: {:?}", e);
                break;
            }
        }
    }
}

fn handle_client_messages(rx: Receiver<String>) {
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(message) => println!("Received from client: {}", message),
                Err(e) => {
                    eprintln!("Receiver error: {:?}", e);
                }
            }
        }
    });
}
