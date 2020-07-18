use std::io::prelude::*;
use std::net::TcpStream;
/* 
let mut Address: String = "127.0.0.1".to_owned();
let port: &str = ":88888";

Address.push_str(port); */
/*
pub fn send_data( Address: & String, data: &mut  [f32) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(Address)?;
    let mut pos = 0;

    while pos <data.len(){
        let bytes_written = stream.write(&data[pos..])?;
        pos += bytes_written;
    }

    Ok(())
}
*/

pub fn recieve_data(address: & String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    Ok(())
}

pub fn test() {
    println!("test");
}

