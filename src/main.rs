use std::{fs::{self, File}, io::{BufWriter, Write, Read, BufReader, Cursor, BufRead}, path::Path, time::{self, Duration}, thread, ops::Index};
use byteorder::{ReadBytesExt, BigEndian, LittleEndian};
use roman::Roman;

mod roman;

fn main() {
    let chars: &str = " abcdefghijklmnopqrstuvwxyz.!?";

    let mut text: String = String::new();
    text = "du lyckades decrypta ris hemliga meddelande".to_owned();
    //File::open("bible2022.txt").unwrap().read_to_string(&mut text).unwrap();
    
    let encoded_text = compress(chars, &text);

    fs::write("dump.bin", encoded_text).unwrap();
    fs::write("dump.txt", text.as_bytes()).unwrap();

    //println!("yo");
    let text: String = decompress("dump.bin", chars);
    println!("{}", text);
}

fn char_to_index(chars: &str, searced_str: char) -> Option<usize> {
    for (index, char) in chars.chars().enumerate() {
        if searced_str == char {
            return Some(index);
        }
    }
    return None;
}

fn compress(chars: &str, text: &String) -> Vec<u8> {
    let mut writer = BufWriter::new(vec![]);

    let mut value: u128 = 0;
    let mut counter = 0;
    //let mut current_number = vec![];
    for t in text.chars() {
        /*
        if t.is_numeric() {
            current_number.push(t.to_string());
            continue;
        } else if current_number.len() > 0 {
            let number = current_number.join("").parse::<u32>().unwrap();
        
            let roman = Roman::from(number).to_string();
            
            for r in roman.chars() {
                if let Some(abc) = push_char_to_stack(chars, r, value, &mut counter) {
                    writer.write(abc.to_be_bytes().as_slice()).unwrap();
                }
            }
            current_number.clear();
        }
        */
        
        if let Some(abc) = push_char_to_stack(chars, t, &mut value, &mut counter) {
            writer.write(abc.to_be_bytes().as_slice()).unwrap();
        }
    }

    /*
    if current_number.len() > 0 {
        let number = current_number.join("").parse::<u32>().unwrap();
        
        let roman = Roman::from(number).to_string();
        
        for r in roman.chars() {
            if let Some(abc) = push_char_to_stack(chars, r, value, &mut counter) {
                writer.write(abc.to_be_bytes().as_slice()).unwrap();
            }
        }
    }*/

    if counter > 0 {
        writer.write(value.to_be_bytes().as_slice()).unwrap();
    }
    
    return writer.buffer().to_vec();
}

fn push_char_to_stack(chars: &str, char: char, value: &mut u128, counter: &mut u8) -> Option<u128> {
    let index = char_to_index(chars, char);
    if index.is_none() {
        return None;
    }
    let index = index.unwrap();

    *value = *value << 5;
    *value |= (index & 31) as u128;

    *counter += 1;
    if *counter > 24 {
        let return_value = value.clone();
        *counter = 0;
        *value = 0;
        return Some(return_value);
    }

    return None;
}

fn decompress(path: &str, chars: &str) -> String {
    let mut buffer = File::open(path).unwrap();
    
    let mut string = String::new();
    loop {
        //println!("yo");
        let next = buffer.read_u128::<BigEndian>();
        if next.is_err() {
            break;
        }
        let next = next.unwrap();

        let length = format!("{:b}", next).len() / 5;

        for index in 0..length+1 {
            let letter = (next >> (length - index) * 5) & 31;
            //println!("{}", letter);
            string += index_to_char(chars, letter as usize);
        }

        

        /*
        for index in 0..26 {
            let letter = (next >> (index*5)) & 31;
            string += index_to_char(chars, letter as usize);
        }
        */
    }
    
    return string;
}

fn index_to_char(chars: &str, index: usize) -> &str {
    return chars.index(index..index+1);
}