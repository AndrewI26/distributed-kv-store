use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let mut port: i16 = 8000;

    let args: Vec<String> = std::env::args().collect();

    if args.get(1).is_some_and(|arg| arg == "address") {
        if let Some(port_arg) = args.get(2) {
            port = port_arg.parse().expect("Invalid port number");
        }
    }

    let listener = TcpListener::bind(std::format!("localhost:{}", port))?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        stream.write_all(b"Message received!")?;
        let mut buffer = [0u8; 1024];
        let bytes_read = stream.read(&mut buffer)?;

        println!(
            "Received: {}",
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
    }

    Ok(())
}
