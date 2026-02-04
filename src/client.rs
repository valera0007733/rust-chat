use std::io::{Write, stdin};
use std::net::TcpStream;

pub fn connect_to_server(address: &str) {
    let mut stream = TcpStream::connect(address)
        .map_err(|err| {
            eprintln!("Could not connect to server: {}", err);
            err
        })
        .unwrap();

    loop {
        let mut input = String::new();
        let read_result = stdin().read_line(&mut input);
        match read_result {
            Err(err) => {
                eprintln!("Could not read from stdin: {}", err);
                continue;
            }
            _ => {}
        }

        let write_result = stream.write(input.as_bytes());
        match write_result {
            Err(err) => {
                eprintln!("Could not write to server: {}", err);
                continue;
            }
            _ => {}
        }
    }
}
