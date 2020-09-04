use byteorder::{ByteOrder, LittleEndian}; //used for f32 conversion
use rppal::gpio::Gpio; //used to manually select spi devices in spi communication
use rppal::spi::{Bus, Mode, SlaveSelect, Spi}; //used to do spi communication
use rppal::spi::Segment;
use std::error::Error; //defines the Error class used for the spi communication
use std::io::prelude::*; //honestly cant remember what this is used for but its important
use std::net::TcpListener; //used to bind a listener to a port
use std::net::TcpStream; //used for tcp communication
use std::{thread, time}; //used to give a minor pause after sending data over spi to allow it time to process the data
use crate::communication;

const SPI_BUFFER_LENGTH: usize= 20;
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

pub struct Spi_Comms {
    //These two are data for wifi coms
    pub tx: Vec<i32>, //TO-DO: I realized this should likely be integer on both sides
    pub rx: Vec<i32>,
    // this is the spi connection made by spi_init, and an indicator as to connection status
    spi: Option<Spi>,
    spi_connection: bool,
    //these are the device pins on the pi for spi communication
    dev: u8,
}

impl Spi_Comms {

    pub fn spi_comms(&mut self) -> Result<(), Box<dyn Error>> {
        //let WIP: u8 = 1; //done writing when WIP = 0
        let mut buffer: std::vec::Vec<u8> = Vec::new();
        let pause = time::Duration::from_millis(20); //wait time for arduino to process data
                                                     //if there is a connection, then:
        if self.spi_connection {
            //package the sending data to the buffer
            //self.data_packaging(&self.rx, &mut buffer)?;
            communication::data_packaging_i32(&self.tx,&mut buffer)?;
            let mut pin = Gpio::new()?.get(self.dev)?.into_output();
            let mut buffer_r = [0u8; SPI_BUFFER_LENGTH];
            //set CS pin to low to start transfer
    
            //write to device if it is connected
            match self.spi.as_mut() {
                None => (),
                Some(t) => {
                    pin.set_low();
                    t.transfer(&mut buffer_r, &buffer)?;
                    pin.set_high();
                }
            }
            
            let mut temp: Vec<i32> = Vec::new();
            //end comm by setting pin back to high
            communication::data_parsing_i32(&mut temp, &buffer_r.to_vec())?;
            self.rx = temp;
            //pause for a second to allow the arduino to process
            thread::sleep(pause);
        } else {
            println!("Not Connected to Spi bus!!")
        }
        Ok(())
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
        Gpio::new()?.get(self.dev)?.into_output().set_high();
        Ok(()) //return ok on success
    }

} // end of the Comms class

/*
* build_comms: the constructor for the comms class
*/
pub fn build_comms(pin: u8) -> Spi_Comms {
    //sets all the defualt values for the comms class
    let coms = Spi_Comms {
        tx: Vec::new(),
        rx: Vec::new(),

        spi: None,
        spi_connection: false,

        dev: pin,
    };
    coms
}
