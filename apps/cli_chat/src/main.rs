use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let ip = std::env::args().nth(1).expect("No pattern given");
    let port = std::env::args().nth(2).expect("no path give");

    let mut stream = TcpStream::connect(format!("{ip}:{port}"))?;

    stream.write(&[1])?;
    stream.write(&mut [0; 128])?;
    Ok(())
}
