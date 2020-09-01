use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::communication::spi_comms::Spi_Comms;
use crate::communication::spi_comms;

pub struct Legs {
    P_CONST: Vec<f32>,
    I_CONST: Vec<f32>,
    D_CONST: Vec<f32>,

    t: Instant,
    dt: Duration,

    L1: f32,
    L2: f32,
    L3: f32,

    pub input: Vec<f32>,
    output: Vec<i32>,//PWM outputs to the arduino
    pos: Vec<f32>, //angle in degrees
    pos_integral: Vec<f32>,
    held_diff: Vec<f32>,

    dev_pin: u8,
    coms: Option<Spi_Comms>,
}

impl Legs {

    fn movement(&mut self){
        let current_time = Instant::now();
        self.dt = current_time.duration_since(self.t);
        self.t = current_time;
        self.update_pos();


        /*
        let diff = self.input[index]-self.pos[index];
        let diff_i = self.pos_integral[index] + diff*t;
        let diff_d = (diff-self.held_diff[index])/t;
        */

        self.PID_loop();

    }

    fn PID_loop(&mut self){
        //will use the self.P_CONST etc. to control the PID control, each "loop" will be a whole leg

        let t = 0.01; //idk how to do this yet I will need to talk to ryan
        for index in 0..3{
            let diff = self.input[index]-self.pos[index];
            let diff_i = self.pos_integral[index] + diff*t;
            let diff_d = (diff-self.held_diff[index])/t;

            let P = (self.P_CONST[index])*diff;
            let I = (self.I_CONST[index])*diff_i;
            let D = (self.D_CONST[index] )*diff_d;

            self.output[index] = (P+I+D).round() as i32;
            self.pos_integral[index] = diff_i;
            self.held_diff[index] = diff;
        }
    }

    fn update_pos(&mut self){
        match self.coms.as_mut() {
            Some(T) => {self.pos = T.rx[..].to_vec();},
            None => (),
    };
    }

    fn f32_PWM_output(&self, mut num: f32) -> i32{
        ((num+2.0)*255.0/2.0).round() as i32
    }

    pub fn return_pos(&mut self) -> Vec<f32>{
        self.pos[..].to_vec()
    }

    pub fn __init__(&mut self){
        self.coms = Some(spi_comms::build_comms(self.dev_pin));
        
        match self.coms.as_mut(){
            Some(T) => {
                match T.spi_init(){
                    Ok(()) => { T.tx = vec![0.0;3];
                                T.spi_comms();
                                self.pos = T.rx[..].to_vec();
                                },
                    Err(e) => println!("There was an Error: {:?}",e)
                        };
                    },
            None => println!("SPI communication could not be initiated")
        };

    }
}

pub fn constructor(leg_num: usize) -> Legs{
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

    let pins: [u8;4] =[23, 24, 25, 26];

    let index: usize = leg_num*3;

    let leg = Legs{
        P_CONST: p_const[index..=index+2].to_vec(),
        I_CONST: i_const[index..=index+2].to_vec(),
        D_CONST: d_const[index..=index+2].to_vec(),

        L1: 10.0, //length in meters 
        L2: 10.0,
        L3: 10.0,

        input: Vec::new(),
        output: Vec::new(),

        pos: vec![0.0;3],
        pos_integral: vec![0.0;3],
        held_diff: vec![0.0;3],

        dev_pin: pins[leg_num],
        coms: None,

        t: Instant::now(),
        dt: Duration::from_millis(0),
    };
    leg
}


pub fn test(){
    println!("test");
}