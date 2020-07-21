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

    init::startup();

    loop {
        // main loop that will be used for control
        communication::test();
        motor_calc::test();
        let address = "test";
        let mut data = Vec::new();
        communication::recieve_data(address, &mut data);

        // call communication code - recieve
        // operation mode set, not sure what this will look like yet
        // call movement code
        // call send sensor data
        // call communication code - send

        break; // temporary break to allow the code to compile
    }

    //testing github commiting
}
