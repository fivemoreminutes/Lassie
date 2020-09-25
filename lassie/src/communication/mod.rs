pub mod spi_comms;
pub mod wifi_comms;
use std::convert::TryInto;
use std::error::Error;
use byteorder::{ByteOrder, LittleEndian};


//TO-DO: dont do single letter variables for non iterators *(l to length)
pub fn data_packaging_f32(data: &Vec<f32>, buffer: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    //this is the sister function to the data parsing code, where data is taken from a float and transitioned to bytes
    let mut buffer1 = [0; 4];
    let start_c = "star".as_bytes();
    let end_c = "done".as_bytes();
    let mut i = 0;
    let l = data.len();

    buffer.append(&mut start_c.to_vec());
    if l == 0 {
        //To-Do: Actually write an Error for this
        Err("There was an Error")? //this is a jank way of returning a dyn error
    } 
    else {
        //essentially pushing data to a buffer after converting it to byte data
        loop {
            LittleEndian::write_f32_into(&data[i..=i], &mut buffer1[..]);
            buffer.append(&mut buffer1.to_vec());

            i += 1;
            if i >= l {
                break;
            }
        }
        buffer.append(&mut end_c.to_vec());

        Ok(()) // returning ok if successful
    }
}

pub fn data_packaging_i32(data: &Vec<i32>, buffer: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    //this is the sister function to the data parsing code, where data is taken from a float and transitioned to bytes
    let mut buffer1 = [0; 4];
    let start_c = "star".as_bytes();
    let end_c = "done".as_bytes();
    let mut i = 0;
    let l = data.len();

    buffer.append(&mut start_c.to_vec());
    if l == 0 {
        Err("There was an Error")? //this is a jank way of returning a dyn error
    } 
    else {
        //essentially pushing data to a buffer after converting it to byte data
        loop {
            LittleEndian::write_i32_into(&data[i..=i], &mut buffer1[..]);
            buffer.append(&mut buffer1.to_vec());

            i += 1;
            if i == l {
                break;
            }
        }
        buffer.append(&mut end_c.to_vec());

        Ok(()) // returning ok if successful
    }
}

pub fn data_parsing_f32(data: &mut Vec<f32>, buffer: &Vec<u8>,) -> Result<(), Box<dyn Error>> {
    let start: &[u8];

    //These are my key words for defining the start and end of a data package for both wifi and spi comms
    let start_c = "star".as_bytes(); //start phrase
    let end_c = "done".as_bytes(); //ending phrase
    let mut temp: std::vec::Vec<f32> = Vec::new(); //for storing data before verifying end
    let mut i = 4; //iterators
    let mut j = 7;

    //To-Do: Add error case for if exceeding length of buffer, replace j with i+3

    if &buffer.len() > &0 {
        start = &buffer[0..=3];
        if start == start_c {
            'inner: loop {
                //grab a slice
                let pos = &buffer[i..=j];
                //if slice is equal to the end phrase, write temp to the output data
                if pos == end_c {
                    data.append(&mut temp);
                    break 'inner;
                }
                //if slice is equal to first phrase again disregard data as there was likely an error
                
                //TO-Do: look into adding a log using this condition
                else if pos == start_c {
                    break 'inner;
                }
                //if length of temp data was over 100 there is a problem and system panics
                else if temp.len() > 100 {
                    println!("There was an error");
                    panic!();
                }
                //data is pushed to temp if there is nothing else needed
                else {
                    temp.push(LittleEndian::read_f32(&pos[..]));
                }
                i += 4; //iterators
                j += 4;
            }
        }
    }
    Ok(()) //return ok if an error doesnt occur
}

pub fn data_parsing_i32(data: &mut Vec<i32>, buffer: &Vec<u8>,) -> Result<(), Box<dyn Error>> {
    let start: &[u8];
    let length = buffer.to_vec().len();
    //These are my key words for defining the start and end of a data package for both wifi and spi comms
    let start_c = "star".as_bytes(); //start phrase
    let end_c = "done".as_bytes(); //ending phrase
    let mut temp: std::vec::Vec<i32> = Vec::new(); //for storing data before verifying end
    let mut i = 5; //iterators
    let mut j = i+3;

    if &buffer.len() > &0 {
            'inner: loop {
                //grab a slice
                let pos = &buffer[i..=j];
                //if length of temp data was over 100 there is a problem and system panics
                if temp.len() > 100 {
                    println!("There was an error");
                    panic!();
                }
                //data is pushed to temp if there is nothing else needed
                else {
                    data.push(i32::from_le_bytes(pos.try_into().unwrap()));
                }
                i += 4; //iterators
                j += 4;
                if j > length-4{
                    break;
                }
            }
        
    }
    Ok(()) //return ok if an error doesnt occur
}