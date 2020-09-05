//this code will be used to control the motion of the legs
// this code should take in the inputs from the laptop and convert those to desired motor positions
use crate::movement::legs;
use crate::movement::legs::Legs;

fn interpolate(leg: &mut Legs, input: Vec<f32>){
    //this will break an input into several distinct steps inside of itself
}

fn set_height(leg: &mut Legs){
    // this will control how tall the bot stands
}

fn twist(leg: &mut Legs) {
    // this function will control the twist of the bot
}

fn pitch(leg: &mut Legs) {
    // this function will control the pitch of the bot
}

fn correct(leg: &mut Legs) {
    //this function will correct the bot based on orientation
}

fn calibrate(leg: &mut Legs) {
    //this function will be run on startup to establish initial orientation conditions   
}
