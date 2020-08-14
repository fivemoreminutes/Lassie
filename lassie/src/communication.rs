use byteorder::{ByteOrder, LittleEndian};
use std::io::prelude::*;
use std::str::from_utf8;
use std::error::Error;
use std::net::TcpStream;
use std::net::TcpListener;
use rppal::gpio::Gpio;
use rppal::spi::{Bus,Mode, SlaveSelect, Spi};
use std::{thread, time};

 pub struct Comms<'a> {
    //These two are for wifi coms
    pub rdata: Vec<f32>, 
    pub sdata: Vec<f32>,
    //These two are for spi coms
    pub tx: Vec<f32>,
    pub rx: Vec<f32>,
   
    spi: Option<Spi>,
    spi_connection: bool,

    address: &'a str,
    stream: Option<TcpStream>,
    connection: bool,
    buffer: [u8;4],

    dev1: u8
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

    let start_c = "star".as_bytes();
    let end_c = "done".as_bytes();

    let mut buffer = [0;512]; //creating a buffer to read data into t
    let mut temp = Vec::new();
    //the following checks if I am connected to the laptop and then writes data if possible
    if self.connection == false { 
        println!("Not Connected!!!"); 
    }
    else{
         //reading from the port to reference of buffer to a vector to capture all data 
        match self.stream.as_mut().unwrap().read(&mut buffer[..]){
            Ok(_x) => (),
            Err(e) => {println!("There was an error: {}", e);
                        self.listen()}
        }

        //println!("{}",buffer[1]);
        if &buffer.len() > &0 {
            start = &buffer[0..=3];//from_utf8(&buffer[..]).unwrap();
            //println!("test");
                            
                let mut i = 4;
                let mut j = 7;
            if start == start_c{
                'inner: loop {
                    
                    //self.stream.as_mut().unwrap().read(&mut buffer[..]);
                    
                    let pos = &buffer[i..=j];
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
            let mut buffer:std::vec::Vec<u8> = Vec::new();
            /* Made new function take place of this whole segment
            let l = self.sdata.len();
            let mut i = 0;
            let mut buffer = Vec::new(); 
            buffer.append(&mut start_c.to_vec());
            let mut buffer1 = [0;4];
            loop {
                LittleEndian::write_f32_into(&self.sdata[i..=i], &mut buffer1[..]);
                buffer.append(&mut buffer1.to_vec());
                //self.stream.as_mut().unwrap().write(&buffer[..]);
                i += 1;
                if i == l {
                    break
                }
            }
            buffer.append(&mut end_c.to_vec());
            println!("{} {} {} {}", buffer[0], buffer[1],buffer[2],buffer[3]); 
*/      
            self.data_packaging(&self.sdata, &mut buffer);

            match self.stream.as_mut().unwrap().write(&buffer[..]){
                Ok(_x) => (),
                Err(e) => {println!("There was an error: {}", e);
                            self.listen()}
            }

        }
        else{
            self.listen()
        }
    }
}

pub fn spi_init(&mut self){
    match Spi::new(Bus::Spi0, SlaveSelect::Ss0, 500_000, Mode::Mode0){
        Ok(spi) => {
                    self.spi = Some(spi);
                    self.spi_connection = true;    },
        Err(e) => println!("Could not connect to Spi Bus because: {:?}", e)
    } 
    Gpio::new().unwrap().get(self.dev1).unwrap().into_output().set_high();
}

//cs is chip select, but I am manually setting the pin to high/low to circumvent the SS pin shortage
pub fn spi_comms(&mut self) -> Result<(),Box< dyn Error >> {
    //let WIP: u8 = 1; //done writing when WIP = 0
    let mut buffer: std::vec::Vec<u8> = Vec::new();
    let ten_millis = time::Duration::from_millis(5);

    //test code start
    //self.tx = self.rdata;
    //test code end
    if self.spi_connection{
        self.data_packaging(&self.rdata,&mut buffer);

        let mut pin = Gpio::new().unwrap().get(self.dev1).unwrap().into_output();
        //let mut pin1 = pin.into_output();
        let mut i = 0;
        let mut j = 3;
        let l = buffer.len();
        loop{

        pin.set_low();
        self.spi.as_mut().unwrap().write(&mut buffer[i..=i]);
        pin.set_high();
        thread::sleep(ten_millis);
        if i >= l-1{
            break
        }
        //println!("{}",buffer[i]);
        i+=1;
        //j+=4;
    }
    /*
        let mut buffer = [0u8; 20];

        pin.set_low();
        self.spi.as_mut().unwrap().read(&mut buffer[..]);
        pin.set_high();

        let l = buffer.len();
        if &buffer[0..=3] == "star".as_bytes() && &buffer[l-3..=l] == "done".as_bytes(){
            self.rx = Vec::new();
            let mut i = 4;
            let mut j = 7;
            let mut pos = &buffer[i..j];
            loop {
                if j == l{
                    break
                }
                else if i >100{
                    println!("There was an error reading from spi, rx is messed up now");
                    break
                }
                self.rx.push(LittleEndian::read_f32(&pos[..]));
                i += 4;
                j += 4;
            }
        }
*/
    }
    else{
        println!("Not Connected to Spi bus!!")
    }
    Ok(())
}


fn data_packaging(&self, data: &Vec<f32>, buffer: &mut Vec<u8>) {

    let mut buffer1 = [0;4];
    let start_c = "star".as_bytes();
    let end_c = "done".as_bytes();
    let mut i = 0;
    let l = data.len();

    buffer.append(&mut start_c.to_vec());
    loop {
        LittleEndian::write_f32_into(&data[i..=i], &mut buffer1[..]);
        buffer.append(&mut buffer1.to_vec());
        //self.stream.as_mut().unwrap().write(&buffer[..]);
        i += 1;
        if i == l {
            break
        }
    }
    buffer.append(&mut end_c.to_vec());
}

}

pub fn build_comms<'a>(addr: &'a str) -> Comms<'a>{
    
    let mut com = Comms {
        rdata: Vec::new(),
        sdata: Vec::new(),

        tx: Vec::new(),
        rx: Vec::new(),

        spi: None,
        spi_connection: true,
        
        address: addr,
        stream: None,
        connection: false,
        buffer: [0;4],

        dev1: 22 as u8,

    };

    com
}

