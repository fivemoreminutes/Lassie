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
    let mut end: &[u8] = &[0;4];
    let mut start: &[u8] = &[0;4];

    let mut start_c = "star".as_bytes();
    let mut end_c = "done".as_bytes();

    let mut buffer = [0;512]; //creating a buffer to read data into t
    let mut temp = Vec::new();
    //the following checks if I am connected to the laptop and then writes data if possible
    if self.connection == false { 
        println!("Not Connected!!!"); 
    }
    else{

        self.stream.as_mut().unwrap().read(&mut buffer[..]); //reading from the port to reference of buffer to a vector to capture all data 
        
        //println!("{}",buffer[1]);
        start = &buffer[0..=3];//from_utf8(&buffer[..]).unwrap();
        //println!("test");
                        
            let mut i = 0;
            let mut j = 3;
        if start == start_c{
            'inner: loop {
                
                //self.stream.as_mut().unwrap().read(&mut buffer[..]);
                
                let mut pos = &buffer[i..=j];
                //let mut end = &buffer;// from_utf8(&buffer[..]).unwrap();
                if pos == end_c{
                    println!("Broke Here 1");
                    self.rdata = temp;
                    break 'inner
                }
                else if pos == start_c{
                    println!("Broke Here 2");
                    break 'inner
                }
                else if temp.len() > 100{
                    println!("There was an error");
                    panic!();
                }
                else{
                    temp.push(LittleEndian::read_f32(&pos[..]));
                }
                i += 4;
                j += 4;
            }
        }

        self.sdata = [0.01;5].to_vec();
        let l = self.sdata.len();
        let mut i = 0;
        let mut buffer = Vec::new(); 
        buffer.append(&mut start_c.to_vec());
        loop {
            let mut buffer1 = [0;4];
            LittleEndian::write_f32_into(&self.sdata[i..=i], &mut buffer1[..]);
            buffer.append(&mut buffer1.to_vec());
            //self.stream.as_mut().unwrap().write(&buffer[..]);
            i += 1;
            if i == l {
                break
            }
        }
        self.stream.as_mut().unwrap().write(&buffer[..]);
    }
}
}

