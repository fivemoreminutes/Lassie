use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::communication::spi_comms::Spi_Comms;
use crate::communication::spi_comms;
use crate::movement;

pub struct Legs {
    P_CONST: Vec<f32>,
    I_CONST: Vec<f32>,
    D_CONST: Vec<f32>,

    t: Vec<f32>,
    prev_instant: Instant,

    L1: f32,
    L2: f32,
    L3: f32,

    pub input: Vec<f32>,
    output: Vec<i32>,//PWM outputs to the arduino
    pos: Vec<f32>, //angle in degrees

    diff_1: Vec<f32>,
    diff_2: Vec<f32>,
    diff_3: Vec<f32>,

    dev_pin: u8,
    coms: Option<Spi_Comms>,
}

impl Legs {

    pub fn movement(&mut self){

        self.update_time();
        self.update_pos();
        self.PID_loop();

    }

    fn PID_loop(&mut self){
        //will use the self.P_CONST etc. to control the PID control, each "loop" will be a whole leg
        let l = self.t.len();
        let mut diff = Vec::new();
        for index in 0..3{
            match index{
                0 => diff = self.diff_1.to_vec(),
                1 => diff = self.diff_2.to_vec(),
                2 => diff = self.diff_3.to_vec(),
                _ => ()
            };

            let mut diff_i = 0.0;
            for x in 0..l as usize {
                diff_i = diff_i + diff[x]*self.t[x];
            }
           
            let diff_d = (diff[l-1]-diff[l-2])/self.t[l-1];

            let P = (self.P_CONST[index])*diff[l-1];
            let I = (self.I_CONST[index])*diff_i;
            let D = (self.D_CONST[index] )*diff_d;

            self.output[index] = (P+I+D).round() as i32;
        }
    }

    fn update_pos(&mut self){

        match self.coms.as_mut() {
            Some(T) => {match movement::to_degrees(T.rx[..].to_vec()){
                            Ok(t) => self.pos = t,
                            Err(e) => println!("There was an error: {:?}", e),
                           };
                        },
            None => (),
    };
        
        self.diff_1.push(self.input[0]-self.pos[0]);
        self.diff_2.push(self.input[1]-self.pos[1]);
        self.diff_3.push(self.input[2]-self.pos[2]);

        let l = self.diff_1.len();
        if l > 20{
            self.diff_1.remove(0);
            self.diff_2.remove(0);
            self.diff_3.remove(0);
        }
    }

    fn update_time(&mut self){
        let t_cur = Instant::now();
        self.t.push(t_cur.duration_since(self.prev_instant).as_secs_f32());
        let l = self.t.len();
        if l > 20 {
            self.t.remove(0);
        }

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
                    Ok(()) => { T.tx = vec![0;3];
                                T.spi_comms();
                                self.update_pos();
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


pub fn test(){
    println!("test");
}