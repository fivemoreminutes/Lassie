use crate::communication::Comms;

// code that runs before the mainloop in the main code
pub fn startup<'a>() -> Comms<'a> {
      
    let mut com = Comms {
        address: "192.168.1.3:2000",
        stream: None,
        rdata: Vec::new(),
        sdata: Vec::new(),
        connection: false,
        buffer: [0;4],
    };
    com.listen();
    com
}
