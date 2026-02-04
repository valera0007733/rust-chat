mod server;
mod client;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let address = args.get(0).expect("No address provided");
    let app_type = args.get(1);
    match app_type {
        Some(_) => {
            server::create_server(address)
        },
        None => {
            client::connect_to_server(address)
        }
    }
}
