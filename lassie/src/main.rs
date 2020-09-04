mod communication;
mod init;
mod movement;
use crate::movement::legs;
use crate::movement::legs::Legs;
extern crate chrono;

fn main() {
    // this code will implimented on the raspberry pi and communicate with the the host computer

    const VERSION: &'static str = env!("CARGO_PKG_VERSION"); // Grab version number meta data
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS"); // Grab my name from meta data

    println!("\n\nLassie, The Best Dog"); // start by printing the version number to the terminal
    println!("\nVersion Number {}", VERSION);
    println!("\nMade by {}", AUTHORS);
    println!("\n{:?}\n\n", chrono::offset::Local::now());



    let mut com = init::startup(); //calling initial connection to the laptop

    let mut leg: Vec<Legs> = Vec::new();
  
    for x in 0..4 as usize {
      leg[x] = legs::constructor(x);
      match leg[x].init_spi(){
          Ok(()) => (),
          Err(e) => {println!("There was an error: {:?}", e);
          panic!();},
      };
    }


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
            leg[x].movement();
        }

        /*
        match com.spi_comms() {
            Ok(()) => (),
            Err(e) => {
                println!("\nThere was an error: {:?}", e);
                match com.spi_init() {
                    Ok(()) => (),
                    Err(e) => println!("There was an Error: {:?}", e),
                };
            }
        } //talking to arduinos
*/
        let l = com.rdata.len(); //outputting the data
        for x in 0..l {
            print!(" {} ", com.rdata[x]);
        }
        print!("\n")
    }
}
