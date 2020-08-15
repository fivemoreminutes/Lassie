use byteorder::{ByteOrder, LittleEndian}; //used for f32 conversion
use rppal::gpio::Gpio; //used to manually select spi devices in spi communication
use rppal::spi::{Bus, Mode, SlaveSelect, Spi}; //used to do spi communication
use std::error::Error; //defines the Error class used for the spi communication
use std::io::prelude::*; //honestly cant remember what this is used for but its important
use std::net::TcpListener; //used to bind a listener to a port
use std::net::TcpStream; //used for tcp communication
use std::{thread, time}; //used to give a minor pause after sending data over spi to allow it time to process the data

/*******************************************
 * Comms:
 * The communication class for the lassie software. The contents and methods for sending and recieving all data
 * are contained inside this class. These include the following methods:
 *
 * listen: waits and listens for a new tcp stream by listening on a set address and socket on the local network
 * wifi_comms:sends and recieves data and returns it to the self.sdata and self.rdata respectively
 * spi_init:initiates the spi communication and the pins for communication
 * spi_comms: actually sending data over spi, will eventuntually take in an int for device select
 * data_packaging: takes a f32 vector and converts it to a byte array with a starting and ending phrase to parse data
 *
 * Special mention:
 * build_comms: the constructor for the comms class as a ~~function~~ that can be called to create a comms object without
 * having to make all the data fields in the object public
 *
 * ***************************************/

pub struct Comms<'a> {
    //These two are data for wifi coms
    pub rdata: Vec<f32>,
    pub sdata: Vec<f32>,
    //These two are data for spi coms
    pub tx: Vec<f32>,
    pub rx: Vec<f32>,
    // this is the spi connection made by spi_init, and an indicator as to connection status
    spi: Option<Spi>,
    spi_connection: bool,
    //these variables are used by the wifi comms section
    address: &'a str,
    stream: Option<TcpStream>,
    connection: bool,

    //these are the device pins on the pi for spi communication
    dev1: u8,
}

impl Comms<'_> {
    pub fn listen<'a>(&mut self) {
        //prints its address
        println!("{}", self.address);
        //binds address to a listener port
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("\nWaiting for new connection");
        //if listener finds new connection writes stream to stream object and sets connection status

        match listener.accept() {
            Ok((socket, addr)) => {
                self.stream = Some(socket);
                self.connection = true;
                println!("\nConnected to new client: {}", addr)
            }
            Err(e) => println!("\nCould not connect to client: {}", e),
        }
    }

    pub fn wifi_comms(&mut self) -> Result<(), Box<dyn Error>> {
        //These are my key words for defining the start and end of a data package for both wifi and spi comms
        //creating a buffer to read data into
        let mut buffer = [0; 512];

        let mut temp = Vec::new();
        //the following checks if I am connected to the laptop and then writes data if possible
        if self.connection == false {
            println!("Not Connected!!!");
            Err("Not Connected")?
        } else {
            //if there is a stream I read from the tcp buffer

            match self.stream.as_mut() {
                None => (),
                Some(t) => {
                    t.read(&mut buffer[..])?;
                }
            }
            //send the data (currently as bytes) to be parsed into a float vector
            self.data_parsing(&mut temp, &buffer.to_vec())?;
            //setting that the comm data value
            self.rdata = temp;

            self.sdata = [0.01; 5].to_vec(); //temp right now as I have no sensor data
            let mut buffer: std::vec::Vec<u8> = Vec::new(); //re-initializing the buffer

            self.data_packaging(&self.sdata, &mut buffer)?; //packaging the data to be sent
                                                            //if there is a active stream write the data to the tcp device
            match self.stream.as_mut() {
                None => (),
                Some(t) => {
                    t.write(&buffer[..])?;
                }
            };
            Ok(())
        }
    }

    fn data_parsing(
        &mut self,
        data: &mut Vec<f32>,
        buffer: &Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let start: &[u8];

        //These are my key words for defining the start and end of a data package for both wifi and spi comms
        let start_c = "star".as_bytes(); //start phrase
        let end_c = "done".as_bytes(); //ending phrase
        let mut temp: std::vec::Vec<f32> = Vec::new(); //for storing data before verifying end
        let mut i = 4; //iterators
        let mut j = 7;

        if &buffer.len() > &0 {
            start = &buffer[0..=3];
            if start == start_c {
                'inner: loop {
                    //grab a slice
                    let pos = &buffer[i..=j];
                    //if slice is equal to the end phrase, write temp to the output data
                    if pos == end_c {
                        data.append(&mut temp);
                        break 'inner;
                    }
                    //if slice is equal to first phrase again disregard data as there was likely an error
                    else if pos == start_c {
                        break 'inner;
                    }
                    //if length of temp data was over 100 there is a problem and system panics
                    else if temp.len() > 100 {
                        println!("There was an error");
                        panic!();
                    }
                    //data is pushed to temp if there is nothing else needed
                    else {
                        temp.push(LittleEndian::read_f32(&pos[..]));
                    }
                    i += 4; //iterators
                    j += 4;
                }
            }
        }
        Ok(()) //return ok if an error doesnt occur
    }

    pub fn spi_init(&mut self) -> Result<(), Box<dyn Error>> {
        //sets up a new spi connection on bus Spi0 and a slave select, though the CS pin is essentially ignored
        match Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0) {
            Ok(spi) => {
                self.spi = Some(spi);
                self.spi_connection = true;
            }
            Err(e) => println!("Could not connect to Spi Bus because: {:?}", e),
        }
        //set device one pin to high to manually stop any communication
        Gpio::new()?.get(self.dev1)?.into_output().set_high();
        Ok(()) //return ok on success
    }

    //cs is chip select, but I am manually setting the pin to high/low to circumvent the SS pin shortage
    pub fn spi_comms(&mut self) -> Result<(), Box<dyn Error>> {
        //let WIP: u8 = 1; //done writing when WIP = 0
        let mut buffer: std::vec::Vec<u8> = Vec::new();
        let pause = time::Duration::from_millis(20); //wait time for arduino to process data
                                                     //if there is a connection, then:
        if self.spi_connection {
            //package the sending data to the buffer

            ////////////////////////////////////////////////////////////////////////////////
            /// TEST CODE FOR TESTING PURPOSES NEEDS TO CHANGE IN THE FUTURE ///////////////
            ////////////////////////////////////////////////////////////////////////////////
            self.data_packaging(&self.rdata, &mut buffer)?;
            let mut pin = Gpio::new()?.get(self.dev1)?.into_output();

            //set CS pin to low to start transfer
            pin.set_low();
            //write to device if it is connected
            match self.spi.as_mut() {
                None => (),
                Some(t) => {
                    t.write(&mut buffer[..])?;
                }
            }
            //end comm by setting pin back to high
            pin.set_high();
            //pause for a second to allow the arduino to process
            thread::sleep(pause);

            //I will need to adjust this whole section I think using spi::segments
            let mut buffer = [0u8; 20];

            pin.set_low();
            self.spi.as_mut().unwrap().read(&mut buffer[..])?;
            pin.set_high();

            let l = buffer.len();
            if &buffer[0..=3] == "star".as_bytes() && &buffer[l - 3..=l] == "done".as_bytes() {
                self.rx = Vec::new();
                let mut i = 4;
                let mut j = 7;
                let mut pos = &buffer[i..j];
                loop {
                    if j == l {
                        break;
                    } else if i > 100 {
                        println!("There was an error reading from spi, rx is messed up now");
                        break;
                    }
                    self.rx.push(LittleEndian::read_f32(&pos[..]));
                    i += 4;
                    j += 4;
                    pos = &buffer[i..j];
                }
            }
        } else {
            println!("Not Connected to Spi bus!!")
        }
        Ok(())
    }

    fn data_packaging(&self, data: &Vec<f32>, buffer: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        //this is the sister function to the data parsing code, where data is taken from a float and transitioned to bytes
        let mut buffer1 = [0; 4];
        let start_c = "star".as_bytes();
        let end_c = "done".as_bytes();
        let mut i = 0;
        let l = data.len();

        buffer.append(&mut start_c.to_vec());
        if l == 0 {
            Err("There was an Error")? //this is a jank way of returning a dyn error
        } else {
            //essentially pushing data to a buffer after converting it to byte data
            loop {
                LittleEndian::write_f32_into(&data[i..=i], &mut buffer1[..]);
                buffer.append(&mut buffer1.to_vec());

                i += 1;
                if i == l {
                    break;
                }
            }
            buffer.append(&mut end_c.to_vec());

            Ok(()) // returning ok if successful
        }
    }
} // end of the Comms class

/*
* build_comms: the constructor for the comms class
*/
pub fn build_comms<'a>(addr: &'a str) -> Comms<'a> {
    //sets all the defualt values for the comms class
    let com = Comms {
        rdata: Vec::new(),
        sdata: Vec::new(),

        tx: Vec::new(),
        rx: Vec::new(),

        spi: None,
        spi_connection: true,
        address: addr,
        stream: None,
        connection: false,

        dev1: 22 as u8,
    };

    com
}
