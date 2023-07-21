use std::{fs::{self, File}, io::{BufWriter, Write, Read}, ops::Index, time::SystemTime, env};
use byteorder::{ReadBytesExt, BigEndian};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("no file, drag file or use cli");
    }

    let start = SystemTime::now();
    let chars: &str = " abcdefghijklmnopqrstuvwxyz.!?12";

    let filename = args[1].clone();

    //let mut text: String = String::new();
    let mut buffer = vec![];
    File::open(&filename).unwrap().read_to_end(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&mut buffer).to_string();

    println!("currently converting {} to smol", filename);
    let encoded_text = compress(chars, &text);
    fs::write(format!("{}.smol",filename), encoded_text).unwrap();
    
    let un_smol = decompress(&format!("{}.smol",filename), chars);
    fs::write(format!("{}.unsmol",filename), un_smol).unwrap();

    println!("done, took {:?}", start.elapsed().unwrap());
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
    let mut current_number = vec![];
    for mut char in text.chars() {
        if char == '\r' {
            continue;
        }

        if char == '\n' {
            push_char_to_stack(chars, '2', &mut value, &mut counter, &mut writer);
            push_char_to_stack(chars, ' ', &mut value, &mut counter, &mut writer);
            continue;
        }

        if char.is_numeric() {
            current_number.push(char.to_string());
            continue;
        } else if current_number.len() > 0 {
            push_char_to_stack(chars, '1', &mut value, &mut counter, &mut writer);
            for index in &current_number {
                let abc: Vec<char> = index_to_char(chars, index.parse::<usize>().unwrap()).chars().collect();
                push_char_to_stack(chars, abc[0], &mut value, &mut counter, &mut writer);
            }
            push_char_to_stack(chars, '1', &mut value, &mut counter, &mut writer);

            current_number.clear();
        }

        if char.is_uppercase() {
            push_char_to_stack(chars, '2', &mut value, &mut counter, &mut writer);
            char = char.to_ascii_lowercase();
        }

        push_char_to_stack(chars, char, &mut value, &mut counter, &mut writer);
    }

    if current_number.len() > 0 {
        push_char_to_stack(chars, '1', &mut value, &mut counter, &mut writer);
        for index in &current_number {
            let abc: Vec<char> = index_to_char(chars, index.parse::<usize>().unwrap()).chars().collect();
            push_char_to_stack(chars, abc[0], &mut value, &mut counter, &mut writer);
        }
        push_char_to_stack(chars, '1', &mut value, &mut counter, &mut writer);
    }

    if counter > 0 {
        writer.write(value.to_be_bytes().as_slice()).unwrap();
    }
    
    writer.flush().unwrap();

    return writer.get_ref().to_vec();
    //return writer.buffer().to_vec();
}

fn push_char_to_stack(chars: &str, char: char, value: &mut u128, counter: &mut u8, writer: &mut BufWriter<Vec<u8>>) {
    let index = char_to_index(chars, char).unwrap_or(0);

    *value |= (index as u128 & 31) << *counter * 5;

    *counter += 1;
    if *counter > 24 {
        writer.write(value.to_be_bytes().as_slice()).unwrap();
        *counter = 0;
        *value = 0;
    }
}

fn decompress(path: &str, chars: &str) -> String {
    let mut buffer = File::open(path).unwrap();
    
    let mut is_number = false;
    let mut next_capitalized = false;

    let mut array: Vec<String> = vec![];

    loop {
        let next = buffer.read_u128::<BigEndian>();
        if next.is_err() {
            break;
        }
        let next = next.unwrap();

        //println!("{:?}", next);

        for index in 0..25 {
            let letter = (next >> (index * 5)) & 31;

            let char = index_to_char(chars, letter as usize);

            if char == "1" {
                is_number = !is_number;
                continue;
            }
            if char == "2" {
                next_capitalized = true;
                continue;
            }

            if is_number {
                let abc: Vec<char> = char.chars().collect();
                let index = char_to_index(chars, abc[0]).unwrap();
                array.push(index.to_string())
            } else {
                if next_capitalized {
                    if char == " " {
                        array.push("\n".to_owned());
                    } else {
                        array.push(char.to_ascii_uppercase().to_string());
                    }

                    next_capitalized = false;
                }else {
                    array.push(char.to_string());
                }
            }
        }
    }

    return array.join("");
}

fn index_to_char(chars: &str, index: usize) -> &str {
    return chars.index(index..index+1);
}