use std::io::prelude::*;
mod server;
mod client;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let app_type = args.get(0);
    match app_type {
        Some(_) => server::create_server(),
        None => client::connect_to_server(),
    }
}
