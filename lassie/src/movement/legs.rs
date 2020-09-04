use std::time::{Duration, Instant};
use std::thread::sleep;
use std::error::Error;
use crate::communication::spi_comms::Spi_Comms;
use crate::communication::spi_comms;
use crate::movement;


const BUFFER_LENGTH: usize = 20; //buffer length for the integral controller

pub struct Legs {
    demo_mode: bool,

    P_CONST: Vec<f32>, //proportional constants
    I_CONST: Vec<f32>, //integral constants
    D_CONST: Vec<f32>, // derivative constants

    t: Vec<f32>, //time in seconds 
    prev_instant: Instant, //previous instant for doing duration calculations

    L1: f32, //linkage length 1 in m
    L2: f32, // linkage length 2 in m
    L3: f32, //linkage length 3 in m

    pub input: Vec<f32>, //desired positions for the motors in degrees
    output: Vec<i32>,//PWM outputs to the arduino
    pos: Vec<f32>, //angle in degrees

    diff_1: Vec<f32>, //difference between pos and desired position over time for each motor
    diff_2: Vec<f32>,
    diff_3: Vec<f32>,

    dev_pin: u8, //device pin on the rasp. pi
    coms: Option<Spi_Comms>, //spi comms object for communication with the arduino
}

impl Legs {

    pub fn movement(&mut self){

        self.update_time(); // update the time 
        self.update_pos(); // update the postions of each motor 
        self.PID_loop(); //create the outputs to be sent to the arduino

    }

    fn PID_loop(&mut self){
        //will use the self.P_CONST etc. to control the PID control, each "loop" will be a whole leg
        let mut PWM_max = 0.0; //this is needed here to be able to compile 
        // the following conditional is for limiting the max speed able to be output to the motors, that way testing can be 
        // done safely
        if self.demo_mode == false{ 
            PWM_max = 255.0;
        }
        else{
            PWM_max = 100.0;
        }


        let l = self.t.len(); //length of time for integral calculations. They will mostly be about 20 
        let mut diff = Vec::new(); //could not define this inside the match so it is intialized here
        for index in 0..3{ //this selects the correct difference variable to use 
            match index{
                0 => diff = self.diff_1.to_vec(), 
                1 => diff = self.diff_2.to_vec(),
                2 => diff = self.diff_3.to_vec(),
                _ => ()
            };

            let mut diff_i = 0.0; //initializes the difference for the integral controller 
            for x in 0..l as usize {
                diff_i = diff_i + diff[x]*self.t[x]; //finding the total integral controller difference
            }
           
            let diff_d = (diff[l-1]-diff[l-2])/self.t[l-1]; //finding the differential difference
            //setting the PID values by multiplying the respective gains by the differences 
            let P = (self.P_CONST[index])*diff[l-1]; 
            let I = (self.I_CONST[index])*diff_i;
            let D = (self.D_CONST[index] )*diff_d;

            self.output[index] = ((P+I+D).round()*PWM_max) as i32; //writing the output to the motors to the output variable
        }
    }
    //updates the current position of the motors in the leg
    fn update_pos(&mut self){
        //if there is a coms variable, then make the connection 
        match self.coms.as_mut() {
            Some(T) => {
                        match movement::to_degrees(T.rx.to_vec()) { //convert the PWM values to degrees
                            Ok(t) => self.pos = t, //if ok, then t is the angle in degrees
                            Err(e) => println!("There was an error: {:?}", e), //print error if error'd
                           };
                        },
            None => (),
    };
        
        self.diff_1.push(self.input[0]-self.pos[0]); ////setting the new differences for each motor
        self.diff_2.push(self.input[1]-self.pos[1]);
        self.diff_3.push(self.input[2]-self.pos[2]);

        let l = self.diff_1.len(); //keeping the difference buffer to be only the BUFFER_LENGTH
        if l > BUFFER_LENGTH{
            self.diff_1.remove(0);
            self.diff_2.remove(0);
            self.diff_3.remove(0);
        }
    }
    //updates the current time step for the integral controller
    fn update_time(&mut self){
        let t_cur = Instant::now(); //grabbing this instant 
        self.t.push(t_cur.duration_since(self.prev_instant).as_secs_f32()); //comparing it to the previous instant to get time since then in seconds
        let l = self.t.len(); //getting the length of the time buffer
        if l > BUFFER_LENGTH {
            self.t.remove(0); //remove the first index if it has surpassed the buffer length
        }
    }

    //made a function to return the current position of the motors for external use
    pub fn return_pos(&mut self) -> Vec<f32>{
        self.pos.to_vec()
    }

    //initiating the spi communication by calling methods in the SPI class
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
}//end of legs class

//constructs the legs class 
pub fn constructor(leg_num: usize) -> Legs{
    //row == leg motors, column == leg numbers
    let p_const = [1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0]; //proportional constants for each motor of each leg

    let i_const = [1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0]; //integral constant for each motor of each leg 
   
    let d_const = [1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0,
                   1.0, 1.0, 1.0]; //derivative constant for each motor of each leg

    let pins: [u8;4] =[23, 24, 25, 26]; //device pins TO-DO: update these to be accurate to actual device connections 

    let index: usize = leg_num*3; //indexing for the legs class

    let leg = Legs{
        demo_mode: false,

        P_CONST: p_const[index..=index+2].to_vec(),
        I_CONST: i_const[index..=index+2].to_vec(),
        D_CONST: d_const[index..=index+2].to_vec(),

        L1: 10.0, //length in meters 
        L2: 10.0,
        L3: 10.0,

        input: Vec::new(),
        output: Vec::new(),

        pos: vec![0.0;3],

        dev_pin: pins[leg_num],
        coms: None,

        t: Vec::new(),
        prev_instant: Instant::now(),

        diff_1: Vec::new(),
        diff_2: Vec::new(),
        diff_3: Vec::new(),

    };
    leg
}
