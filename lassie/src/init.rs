use crate::communication::Comms;
use crate::communication;

// code that runs before the mainloop in the main code
pub fn startup<'a>() -> Comms<'a> {
  /*    
    let mut com = Comms {
        address: "192.168.1.3:2000",
        stream: None,
        rdata: Vec::new(),
        sdata: Vec::new(),
        connection: false,
        buffer: [0;4],
    };
    */
    let mut com = communication::build_comms("192.168.1.3:2000");
    com.listen();
    com.spi_init();
    com
}
