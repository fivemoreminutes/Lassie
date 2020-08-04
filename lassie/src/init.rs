use crate::communication::Comms;

// pub fn test() {
//     println!("test");
// }

pub fn startup<'a>() -> Comms<'a> {
    // Initial Tasks - takes place in the init code
    // check communication with arduinos
    // run zeroing protocol to establish linkage locations
    // start sending any sensor data to the main computer
    // wait for commands from main computer
      
    let mut com = Comms {
        address: "192.168.1.3:1000",
        stream: None,
        rdata: Vec::new(),
        sdata: Vec::new(),
        connection: false,
        buffer: Vec::new(),
    };
    com.listen();
    com
}
