
use crate::communication::wifi_comms;
use crate::communication::wifi_comms::Wifi_Comms;
use crate::movement::legs;
use crate::movement::legs::Legs;

// code that runs before the mainloop in the main code
pub fn startup<'a>(leg: &mut Vec<Legs>, com:&mut Wifi_Comms) {
  //com = wifi_comms::build_comms("192.168.1.3:2000"); //builds a comm object for all communication
  com.listen(); //start a tcp connection

  for x in 0..4 as usize {
    leg[x] = legs::constructor(x);
    match leg[x].init_spi(){
        Ok(()) => (),
        Err(e) => {println!("There was an error: {:?}", e);
        panic!();},
    };
  }
}