use std::io::{Cursor, Read};

use byteorder::{ReadBytesExt, BigEndian};

use crate::smol_utils;

pub struct SmolReader {
    reader: Cursor<Vec<u8>>,
    is_number: bool,
    next_capitalized: bool,
    string_builder: String,
    charset: String
}

impl SmolReader {
    pub fn read_header(&mut self) {
        self.reader.read(&mut [0; 4]).unwrap();
        
        let mut charset_buffer = vec![0; 32];
        self.reader.read_exact(&mut charset_buffer).unwrap();
        self.charset = String::from_utf8_lossy(&charset_buffer).to_string();
    }

    pub fn handle_special(&mut self, char: char) -> bool {
        if char == '1' {
            self.is_number = !self.is_number;
            return true;
        }
        if char == '2' {
            self.next_capitalized = true;
            return true;
        }
        return false;
    }

    pub fn read_number(&mut self, char: char) {
        let index = smol_utils::char_to_index(&self.charset, char).unwrap().to_string();
        self.string_builder += &index;
    }

    pub fn decompress(&mut self, data: Vec<u8>) -> String {
        self.reader = Cursor::new(data.clone());

        self.read_header();

        while data.len() > self.reader.position() as usize {
            let next = self.reader.read_u128::<BigEndian>().unwrap();
    
            //println!("{:?}", next);
    
            for index in 0..25 {
                let letter = ((next >> (index * 5)) & 31) as usize;
    
                let char = smol_utils::index_to_char(&self.charset, letter).unwrap_or(' ');

                if self.handle_special(char) {
                    continue;
                }

                if self.is_number {
                    self.read_number(char);
                } else {
                    if self.next_capitalized {
                        if char == ' ' {
                            self.string_builder.push('\n');
                        } else {
                            self.string_builder.push(char.to_ascii_uppercase());
                        }
    
                        self.next_capitalized = false;
                    }else {
                        self.string_builder.push(char);
                    }
                }
            }
        }

        return self.string_builder.clone();
    }
}

impl Default for SmolReader {
    fn default() -> Self {
        Self { 
            reader: Default::default(), 
            is_number: Default::default(), 
            next_capitalized: Default::default(), 
            string_builder: Default::default(),
            charset: Default::default(),
        }
    }
}