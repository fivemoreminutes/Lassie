use crate::communication;
use  std::net::TcpStream;

// pub fn test() {
//     println!("test");
// }

pub fn startup() -> TcpStream {
    // Initial Tasks - takes place in the init code
    // check communication with arduinos
    // run zeroing protocol to establish linkage locations
    // start sending any sensor data to the main computer
    // wait for commands from main computer

    communication::listen()
}
