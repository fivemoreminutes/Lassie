
use crate::communication::wifi_comms;
use crate::communication::wifi_comms::Wifi_Comms;


// code that runs before the mainloop in the main code
pub fn startup<'a>() -> Wifi_Comms<'a> {
  let mut com = wifi_comms::build_comms("192.168.1.3:2000"); //builds a comm object for all communication
  com.listen(); //start a tcp connection



  /*
  match com.spi_init() {
    Ok(()) => (),
    Err(e) => println!("There was an Error: {:?}", e),
  };*/ 
  // start an spi conneciton
  com //return the comm variable
}
