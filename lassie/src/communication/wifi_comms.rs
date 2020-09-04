use byteorder::{ByteOrder, LittleEndian}; //used for f32 conversion
use rppal::gpio::Gpio; //used to manually select spi devices in spi communication
use rppal::spi::{Bus, Mode, SlaveSelect, Spi}; //used to do spi communication
use std::error::Error; //defines the Error class used for the spi communication
use std::io::prelude::*; //honestly cant remember what this is used for but its important
use std::net::TcpListener; //used to bind a listener to a port
use std::net::TcpStream; //used for tcp communication
use std::{thread, time}; //used to give a minor pause after sending data over spi to allow it time to process the data
use crate::communication;
/*******************************************
 * Comms:
 * The communication class for the lassie software. The contents and methods for sending and recieving all data
 * via TCP are contained inside this class. These include the following methods:
 *
 * listen: waits and listens for a new tcp stream by listening on a set address and socket on the local network
 * wifi_comms:sends and recieves data and returns it to the self.sdata and self.rdata respectively
 *
 * Special mention:
 * build_comms: the constructor for the comms class as a ~~function~~ that can be called to create a comms object without
 * having to make all the data fields in the object public
 *
 * ***************************************/

pub struct Wifi_Comms<'a> {
    //These two are data for wifi coms
    pub rdata: Vec<f32>,
    pub sdata: Vec<f32>,

    //these variables are used by the wifi comms section
    address: &'a str,
    stream: Option<TcpStream>,
    connection: bool,

}

impl Wifi_Comms<'_> {
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
            communication::data_parsing_f32(&mut temp, &buffer.to_vec())?;

            //setting that the comm data value
            self.rdata = temp;

            self.sdata = [0.01; 5].to_vec(); //temp right now as I have no sensor data
            let mut buffer: std::vec::Vec<u8> = Vec::new(); //re-initializing the buffer

            //self.data_packaging(&self.sdata, &mut buffer)?; //packaging the data to be sent
            communication::data_packaging_f32(&self.sdata,&mut buffer);
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

} // end of the Comms class

/*
* build_comms: the constructor for the comms class
*/
pub fn build_comms<'a>(addr: &'a str) -> Wifi_Comms<'a> {
    //sets all the defualt values for the comms class
    let com = Wifi_Comms {
        rdata: Vec::new(),
        sdata: Vec::new(),

        address: addr,
        stream: None,
        connection: false,
    };
    com
}
