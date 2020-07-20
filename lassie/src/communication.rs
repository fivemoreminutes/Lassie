use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{ByteOrder, LittleEndian};
/* 
let mut Address: String = "127.0.0.1".to_owned();
let port: &str = ":88888";

Address.push_str(port); */

pub fn send_data( address: & String, data: [f32; 4]  ) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    let buf = to_u8(data);
    stream.write_all(buf)?;

    Ok(())
}

pub fn recieve_data(address: & String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    let mut data = to_f32_array(&buffer);
    Ok(())
}


fn to_u8<'a>(data: [f32;4]) -> &'a [u8] {
    let mut buffer = data.to_vec();
    let p = buffer.as_mut_ptr();
    let len = buffer.len();
    //let cap = buffer.capacity();

    unsafe{
        std::slice::from_raw_parts(p as *mut _, len)
}
}

fn to_f32_array(buffer: &Vec<u8>) -> [f32;4]{
    let len = buffer.len();
    let mut done = false;
    let mut array: [f32; 4] = [0.0; 4];
    let mut i = 0;
    let mut j = 4;

    while !done {

        array[i] = LittleEndian::read_f32(&buffer[i..=4]);
        i = i+4;
        j = j+4;
        if i == len {
            done = true;
        }

}
array
}

pub fn test() {
    println!("test");
}


