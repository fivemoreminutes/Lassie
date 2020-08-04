use byteorder::{ByteOrder, LittleEndian};
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

/******************************************************************************************
 * send_data <- takes in the pi address and port, as well as data in a ver<f32> format
 * writes data to the port at the listed address
 * send_data -> outputs success/ failure of write
 * ***************************************************************************************/

 pub struct Comms<'a> {
    pub address: &'a str,
    pub stream: Option<TcpStream>,
    pub rdata: Vec<f32>,
    pub sdata: Vec<f32>,
    pub connection: bool,
    pub buffer: Vec<u8>,
}

impl Comms<'_> {

pub fn listen<'a>(&mut self) {
    let listener = TcpListener::bind(self.address).unwrap();
    println!("\nWaiting for new connection");
    match listener.accept() {
        Ok((socket,addr)) => {
            self.stream = Some(socket);
            self.connection = true;
            println!("\nConnected to new client: {}", addr)
        }
        Err(e) => println!("\nCould not connect to client: {}", e)
    }
}

pub fn wifi_comms(&mut self) {
    
    //let mut stream = TcpStream::connect(address)?; //connecting to port
    self.buffer = Vec::new(); //creating a buffer to read data into t
   
    if self.connection == false { 
        println!("Not Connected!!!") }
    else{
        self.stream.as_mut().unwrap().read(&mut self.buffer); //reading from the port to reference of buffer to a vector to capture all data 
        self.to_f32_vec(); 
        self.buffer = Vec::new(); 

        self.to_u8(); 
        self.stream.as_mut().unwrap().write(&self.buffer);
    }
}

/******************************************************************************************
 * to_u8 <- takes in a empty buffer vector and a data vector
 * takes the data and converts it to f32 using the byte order library
 * to_u8 -> nothing formally output, because it writes to the location of the buffer
 * ***************************************************************************************/

fn to_u8(&mut self) {
    let size = 4 * self.sdata.len(); //number of bytes needed to write
    let mut bytes = vec![0; size]; //essentially the buffer
    LittleEndian::write_f32_into(&self.sdata, &mut bytes); //writes data to the bytes

    let mut done = false; // might rework this later to reduce redefinitions, but writes
    let len = bytes.len(); // each part to its respective location in buffer array
    let mut i = 0;
    while !done {
        if len > i {
            self.buffer.push(bytes[i]);
            i += 1;
        } else {
            done = true;
        }
    }
}

/******************************************************************************************
 * to_f32_vec <- takes in a filled buffer and an empty f32 vector
 * writes to the f32 vector using the byte order library in a while loop
 * to_f32_vec -> nothing formally returned, but writes to location of f32 vector
 * ***************************************************************************************/

fn to_f32_vec(&mut self) {
    let len = self.buffer.len(); //finds the length of the buffer data
    let mut done = false; //qualifier for while loop
    let mut i = 0; //lower index of buffer slice location
    let mut j = 3; //upper index of buffer slice location
    while !done {
        self.rdata.push(LittleEndian::read_f32(&self.buffer[i..=j])); //writes to the data location
        i += 4;
        j += 4; //iterates
        if i == len {
            done = true; //ends when it reaches the end of the buffer vector
        }
    }
}
}