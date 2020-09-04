pub mod legs;
use std::error::Error;

fn to_degrees(data: Vec<i32>) -> Result<Vec<f32>, Box<dyn Error>>{
    let pwm_lims = [0, 255];
    let angles = [0, 360]; //this will need to be adjusted in the future
    let l = data.len();
    let mut i = 0;
    let mut degrees: Vec<f32> = Vec::new();
    loop {
        degrees.push(((data[i]/pwm_lims[1]) as f32)*(angles[1] as f32));
        if i == l || i > l{
            break
        }
    }
Ok(degrees)
}