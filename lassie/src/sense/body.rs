use crate::communication::spi_comms::Spi_Comms;
use crate::communication::spi_comms;

const PIN_NUM: u8 = 27; //pin connected to arduino for misc sensors

//more will be added to this as time goes on, likely something to determine loading on each leg and some other neat jazz
pub struct Body {
    orientation: Vec<f32>,
    coms: Spi_Comms,
}

impl Body {
    pub fn find_orientation(&mut self){
        //need to find a IMU First and then write arduino code
    }

    pub fn init_spi(&mut self) -> Result<() , Box<dyn Error>>{
        self.coms = Some(spi_comms::build_comms(self.dev_pin)); //attempting to create a coms object
        match self.coms.as_mut(){
            Some(T) => { //if successful in making the device then make the connection
                T.spi_init()?; //attempt to connect to the device over spi, error out if there is a problem
                T.tx = vec![0;3]; //creating an empty vector to get data back
                T.spi_comms()?; //communicating to update postions in the comms.rx location
                self.update_pos();//updating the positions in the legs 
                        },
            None => println!("SPI communication could not be initiated")
        };
        Ok(())
    }

}

pub fn constructor() -> Body{
    bod = Body{
        orientation: Vec::new(),
        coms = spi_comms::build_comms(DEV_PIN),
    }
}