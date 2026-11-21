use std::collections::HashMap;
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

    let mut store: HashMap<String, String> = HashMap::new();

    let listener = TcpListener::bind(std::format!("localhost:{}", port))?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buffer = [0u8; 1024];
        let bytes_read = stream.read(&mut buffer)?;

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let tokens: Vec<&str> = request.split_whitespace().collect();

        if tokens.len() < 2 {
            stream.write_all(format!("Unknown request: {}", request).as_bytes())?;
            return Ok(());
        }

        match tokens.get(0).copied() {
            Some("GET") => {
                let Some(key) = tokens.get(1).copied() else {
                    stream.write_all(b"ERROR Missing key\n")?;
                    continue;
                };
                match store.get(key) {
                    Some(v) => stream.write_all(format!("OK {v}\n").as_bytes())?,
                    None => stream.write_all(b"NOT_FOUND\n")?,
                }
            }
            Some("SET") => {
                let (Some(key), Some(value)) = (tokens.get(1).copied(), tokens.get(2).copied())
                else {
                    stream.write_all(b"ERROR Expected key and value\n")?;
                    continue;
                };
                store.insert(key.to_string(), value.to_string());
                stream.write_all(b"OK\n")?;
            }
            Some(command) => {
                stream.write_all(format!("ERROR Unknown command: {command}\n").as_bytes())?;
            }
            None => {
                stream.write_all(b"ERROR Empty request\n")?;
            }
        }
    }

    Ok(())
}
