use byteorder::{ByteOrder, LittleEndian};
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;



/******************************************************************************************
 * send_data <- takes in the pi address and port, as well as data in a ver<f32> format
 * writes data to the port at the listed address
 * send_data -> outputs success/ failure of write
 * ***************************************************************************************/

pub fn send_data(address: &str, data: Vec<f32>) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?; //connecting to port
    let mut buf = Vec::new(); //creating new buffer for bytewise data
    to_u8(&mut buf, &data); // converting f32 to bytes and writing to buffer
    stream.write_all(&buf)?; //writing the buffer to socket
    Ok(()) //outputting a success to main
}



/******************************************************************************************
 * recieve_data <- takes in the pi address and port, as well as an empty data vector 
 * reads from the port at the address and writes to the empty vector that can be accessed in main
 * recieve_data -> outputs success/ failure of read
 * ***************************************************************************************/

pub fn recieve_data(address: &str, data: &mut Vec<f32>) ->  std::io::Result<()> {
    
    let mut stream = TcpStream::connect(address)?; //connecting to port
    let mut buffer = Vec::new(); //creating a buffer to read data into t
    stream.read_to_end(&mut buffer)?; //reading from the port to reference of buffer to a vector to capture all data
    
    ////////////// This is for Testing Only ///////////////////////////////////////////////////
    //data.push(2.5); data.push(3.7); data.push(4.6);
    //to_u8(&mut buffer, data);
        
    ///////////////////////////////////////////////////////////////////////////////////////////
    to_f32_vec(&buffer, data); //converting the buffer to a f32 array
    Ok(()) //outputting success/error to main
}



/******************************************************************************************
 * to_u8 <- takes in a empty buffer vector and a data vector
 * takes the data and converts it to f32 using the byte order library
 * to_u8 -> nothing formally output, because it writes to the location of the buffer
 * ***************************************************************************************/

fn to_u8( buffer: &mut Vec<u8>, data: &Vec<f32>) {
    let size = 4*data.len(); //number of bytes needed to write
    let mut bytes = vec![0;size]; //essentially the buffer
    LittleEndian::write_f32_into(&data, &mut bytes ); //writes data to the bytes 

    let mut done = false;       // might rework this later to reduce redefinitions, but writes
    let len = bytes.len();      // each part to its respective location in buffer array
    let mut i = 0;
    while !done{
        if len > i{
            buffer.push(bytes[i]);
            i += 1;
        } else {done = true;}
    }
}


/******************************************************************************************
 * to_f32_vec <- takes in a filled buffer and an empty f32 vector
 * writes to the f32 vector using the byte order library in a wile loop
 * to_f32_vec -> nothing formally returned, but writes to location of f32 vector
 * ***************************************************************************************/

fn to_f32_vec(buffer: &Vec<u8>, data: &mut Vec<f32>){
    let len = buffer.len(); //finds the length of the buffer data
    let mut done = false; //qualifier for while loop
    let mut i = 0; //lower index of buffer slice location
    let mut j = 3; //upper index of buffer slice location
    while !done {
        data.push(LittleEndian::read_f32(&buffer[i..=j])); //writes to the data location
        i += 4; j +=4; //iterates
        if i == len {
            done = true; //ends when it reaches the end of the buffer vector
        }
    }
}

//this is a simple test function to ensure the module is loaded
pub fn test() {
    println!("test");
}
