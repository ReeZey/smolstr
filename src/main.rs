use std::{fs::{self, File}, io::{BufWriter, Write, Read}, ops::Index, time::SystemTime, path::Path};
use byteorder::{ReadBytesExt, BigEndian};

fn main() {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir(output_dir).unwrap();
    }
    

    let start = SystemTime::now();
    let chars: &str = " abcdefghijklmnopqrstuvwxyz.!?12";

    let mut text: String = String::new();
    File::open("bible2022.txt").unwrap().read_to_string(&mut text).unwrap();
    
    let encoded_text = compress(chars, &text);

    //println!("{:?}", encoded_text);
    let checksum = crc32fast::hash(&encoded_text);
    println!("currently writing {}", checksum);
    println!("{}/{}.smol", output_dir.to_string_lossy(), checksum);
    fs::write(output_dir.join(format!("{}.smol",checksum)), encoded_text).unwrap();
    fs::write(output_dir.join(format!("{}.txt",checksum)), text.as_bytes()).unwrap();

    //println!("yo");
    //let text: String = 
    decompress(&output_dir.join(format!("{}.smol",checksum)).to_string_lossy(), chars);
    //println!("{}", text);

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
            for _ in 0..3 {
                push_char_to_stack(chars, ' ', &mut value, &mut counter, &mut writer);
            }
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
    let mut space_counter = 0;

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
            
            if char == " " && !is_number {
                space_counter += 1;
                
                continue;
            } else {
                if space_counter >= 3 {
                    while space_counter >= 3 {
                        array.push("\n".to_owned());
                        space_counter -= 3;
                    }
                    space_counter = 0;
                }

                if space_counter > 0 {
                    array.push(" ".to_owned());
                    space_counter = 0;
                }
            }

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
                    array.push(char.to_ascii_uppercase().to_string());
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