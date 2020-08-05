use byteorder::{ByteOrder, LittleEndian};
use std::io::prelude::*;
use std::str::from_utf8;
use std::net::TcpStream;
use std::net::TcpListener;

 pub struct Comms<'a> {
    pub address: &'a str,
    pub stream: Option<TcpStream>,
    pub rdata: Vec<f32>,
    pub sdata: Vec<f32>,
    pub connection: bool,
    pub buffer: [u8;4],
}
//Comms is my communication variable that will also be used for spi when I start work on that
impl Comms<'_> {

pub fn listen<'a>(&mut self) {
    println!("{}",self.address);
    let listener = TcpListener::bind(&self.address).unwrap();
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
    let mut end = &[0;4];
    let mut start = &[0;4];

    let start_c = "star".as_bytes();
    let end_c = "done".as_bytes();

    let mut buffer = [0;4]; //creating a buffer to read data into t
    let mut temp = Vec::new();
    //the following checks if I am connected to the laptop and then writes data if possible
    if self.connection == false { 
        println!("Not Connected!!!"); 
    }
    else{

        self.stream.as_mut().unwrap().read(&mut buffer[..]); //reading from the port to reference of buffer to a vector to capture all data 
        
        //println!("{}",buffer[1]);
        
        start = &buffer;//from_utf8(&buffer[..]).unwrap();
        //println!("test");
        if start == start_c {
            'inner: loop {

                
                self.stream.as_mut().unwrap().read(&mut buffer[..]);

                let mut end = &buffer;// from_utf8(&buffer[..]).unwrap();
                if end == end_c{
                    println!("Broke Here 1");
                    self.rdata = temp;
                    break 'inner
                }
                else if end == start_c{
                    println!("Broke Here 2");
                    break 'inner
                }
                else if temp.len() > 100{
                    println!("There was an error");
                    panic!();
                }
                else{
                    temp.push(LittleEndian::read_f32(&buffer[..]));
                }
            }
        }

        self.sdata = [0.01;5].to_vec();
        let l = self.sdata.len();
        let mut i = 0;
        self.buffer = [0;4];
        self.stream.as_mut().unwrap().write(start_c);
        loop {
            self.buffer = [0;4]; 

            LittleEndian::write_f32_into(&self.sdata[i..=i], &mut buffer[..]);
            self.stream.as_mut().unwrap().write(&buffer[..]);
                i += 1;
                if i == l {
                    break
                }
        }
        self.stream.as_mut().unwrap().write(end_c);
    }
}
}
/******************************************************************************************
 * takes the data and converts it to f32 using the byte order library
 * ***************************************************************************************/
/*
fn to_u8(&mut self) {
    let size = 4 * self.sdata.len(); //number of bytes needed to write
    let mut bytes = vec![0; size]; //essentially the buffer
    LittleEndian::write_f32_into(&self.sdata, &mut bytes); //writes data to the bytes

    let mut done = false; 
    let len = bytes.len(); 
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
*/
/******************************************************************************************
 * writes to the f32 vector using the byte order library in a while loop
 * ***************************************************************************************/
/*
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


fn to_c(&mut self) {
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
*/