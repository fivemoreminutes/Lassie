mod communication;
mod init;
mod motor_calc;
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

    loop {

        let mut rdata = Vec::new(); //init new vectors for data
        let mut sdata = Vec::new();
        //stage data to be sent by placing it in the coms object
        com.sdata = sdata;
        com.wifi_comms(); //sending the data

        let l = com.rdata.len(); //outputting the data 
        for x in 0..l {
            println!("{}",com.rdata[x]);
        }
        println!("\n");
  
    }

}
