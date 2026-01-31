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
        // let mut buff = vec![0; 4096];
        // let read = stream.read(&mut buff);
        // match read {
        //     Ok(_) => {
        //         println!("{}", String::from_utf8_lossy(&buff))
        //     }
        //     Err(e) => {eprintln!("Could not read from server: {}", e)}
        // }

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        stream
            .write(input.as_bytes())
            .map_err(|err| {
                eprintln!("Could not write to server: {}", err);
                err
            })
            .unwrap();
    }
}
