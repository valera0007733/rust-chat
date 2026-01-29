use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
mod server;
mod client;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let app_type = args.get(0);
    match app_type {
        Some(_) => server::create_server(),
        None => server::send_msg_to_client(),
    }
}
