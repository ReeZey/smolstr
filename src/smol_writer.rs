use std::{char, io::{BufWriter, Write, Error, ErrorKind}};

use crate::smol_utils::{char_to_index, index_to_char};

#[derive(Debug)]
pub struct SmolWriter {
    pub number_string: String,
    charset: String,
    counter: u8,
    value: u128,
    bufwriter: BufWriter<Vec<u8>>
}

#[allow(dead_code)]
impl SmolWriter {
    fn new(charset: String) -> SmolWriter {
        SmolWriter { 
            number_string: Default::default(), 
            charset,
            counter: Default::default(), 
            value: Default::default(), 
            bufwriter: BufWriter::new(vec![]) 
        }
    }

    fn write_header(&mut self) {
        self.bufwriter.write("smol".as_bytes()).unwrap();
        self.bufwriter.write(self.charset.as_bytes()).unwrap();
    }

    pub fn compress(&mut self, text: &String) -> Vec<u8> {
        self.write_header();

        for mut char in text.chars() {
            if char == '\r' {
                continue;
            }
    
            if char == '\n' {
                self.write('2').unwrap();
                self.write(' ').unwrap();
                continue;
            }
            
            if char.is_numeric() {
                self.number_string.push(char);
                continue;
            } else {
                self.write_number();
            }
    
            if char.is_uppercase() {
                self.write('2').unwrap();
                char = char.to_ascii_lowercase();
            }
    
            self.write(char).unwrap();
        }
    
        self.flush();
    
        return self.buffer();
    }

    pub fn update_charset(&mut self, charset: String) -> Result<(), Error> {
        if charset.len() != 32 {
            return Err(Error::new(
                ErrorKind::InvalidData, 
                format!("Charset is wrong, requires all 32 letters to be filled, you filled {}", charset.len())
            ))
        }
        self.charset = charset;
        Ok(())
    }

    pub fn write_number(&mut self) {
        if self.number_string.len() > 0 {
            self.write('1').unwrap();
            for c in self.number_string.clone().chars() {
                let number = index_to_char(&self.charset, c.to_string().parse::<usize>().unwrap()).unwrap_or(' ');
                self.write(number).unwrap();
            }
            self.write('1').unwrap();
    
            self.number_string = String::new();
        }
    }

    pub fn write(&mut self, c: char) -> Result<(), Error> {
        let index = char_to_index(&self.charset, c).unwrap_or(0);

        self.value |= (index as u128 & 31) << self.counter.clone() * 5;

        self.counter += 1;
        if self.counter > 24 {
            let value = self.value;
            self.bufwriter.write(value.to_be_bytes().as_slice()).unwrap();
            self.counter = 0;
            self.value = 0;
        }

        Ok(())
    }
    
    pub fn flush(&mut self) {
        self.write_number();

        if self.counter > 0 {
            let value = self.value;
            self.bufwriter.write(value.to_be_bytes().as_slice()).unwrap();
        }
        
        self.bufwriter.flush().unwrap();
    }

    pub fn buffer(&mut self) -> Vec<u8> {
        return self.bufwriter.get_ref().to_vec();
    }
}

impl Default for SmolWriter {
    fn default() -> Self {
        Self { 
            number_string: Default::default(), 
            charset: " abcdefghijklmnopqrstuvwxyz.!?12".to_string(),
            counter: Default::default(), 
            value: Default::default(), 
            bufwriter: BufWriter::new(vec![]) 
        }
    }
}