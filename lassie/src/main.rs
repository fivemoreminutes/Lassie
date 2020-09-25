mod communication;
mod init;
mod movement;
use crate::movement::legs;
use crate::movement::legs::Legs;
extern crate chrono;
use crate::communication::wifi_comms;
use crate::communication::wifi_comms::Wifi_Comms;

const VERSION: &'static str = env!("CARGO_PKG_VERSION"); // Grab version number meta data
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS"); // Grab my name from meta data


fn main() {
    // this code will implimented on the raspberry pi and communicate with the the host computer

    println!("\n\nLassie, The Best Dog"); // start by printing the version number to the terminal
    println!("\nVersion Number {}", VERSION);
    println!("\nMade by {}", AUTHORS);
    println!("\n{:?}\n\n", chrono::offset::Local::now());
    
    let mut leg: Vec<Legs> = Vec::new();
    let mut com = wifi_comms::build_comms("192.168.1.3:2000");

    init::startup(&mut leg, &mut com); //calling 
    // right now legs are defined in the main class but I expect this to change over time 

    loop {
        let rdata: std::vec::Vec<f32> = Vec::new(); //init new vectors for data
        let sdata = Vec::new();
        //stage data to be sent by placing it in the coms object
        com.sdata = sdata;

        match com.wifi_comms() {
            Ok(()) => (),
            Err(e) => {
                println!("\nThere was an error: {:?}", e);
                com.listen();
            }
        } //sending the data

        for x in 0..4 as usize{
            leg[x].test_spi_Coms();
        }
    }
}
