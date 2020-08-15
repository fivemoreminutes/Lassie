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
        let mut rdata: std::vec::Vec<f32> = Vec::new(); //init new vectors for data
        let mut sdata = Vec::new();
        //stage data to be sent by placing it in the coms object
        com.sdata = sdata;

        match com.wifi_comms() {
            Ok(()) => (),
            Err(e) => {
                println!("\nThere was an error: {:?}", e);
                com.listen();
            }
        } //sending the data

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

        let l = com.rdata.len(); //outputting the data
        for x in 0..l {
            print!(" {} ", com.rdata[x]);
        }
        print!("\n")
    }
}
