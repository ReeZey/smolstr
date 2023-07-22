use std::{fs::{self, File}, io::{Read, stdin, stdout, Write}, time::{SystemTime, Duration}, env, collections::VecDeque, path::PathBuf, thread};

mod smol_utils;
mod smol_writer;
mod smol_reader;
use smol_writer::SmolWriter;
use smol_reader::SmolReader;

fn main() {
    let mut arguments: VecDeque<String> = env::args().collect();
    arguments.pop_front();

    println!("\n--- smolstr utility ---\n");

    let mut pause_before_exit = false;

    let mut encode: bool = false;
    let mut decode: bool = false;
    let mut filename: PathBuf;
    let file_path: PathBuf;

    if arguments.len() == 0 {
        println!("without arguments, we enter raw mode");
        println!("---- arguments ---");
        println!("encode: -x");
        println!("decode: -u");
        println!("filename: -o");
        println!("----");
        println!("running without -x or -u will attempt to auto detect");
        println!("usage: <filename> <args>");
        print!("input: ");
        stdout().flush().unwrap();

        let mut string = String::new();
        stdin().read_line(&mut string).unwrap();

        string = string.replace("\n", "");
        string = string.replace("\r", "");

        for argument in string.split(" ") {
            arguments.push_back(argument.to_string())
        }

        pause_before_exit = true;
    }
    file_path = PathBuf::from(arguments.pop_front().unwrap());
    filename = PathBuf::from(file_path.file_name().unwrap().to_string_lossy().to_string());

    while arguments.len() > 0 {
        let argument: &str = &arguments.pop_front().unwrap();
        match argument {
            "-x" => encode = true,
            "-u" => decode = true,
            "-o" => {
                filename = match arguments.pop_front() {
                    Some(filename) => PathBuf::from(filename),
                    None => {
                        println!("Error. Improper usage of -o, example: -o <filename>");
                        return;
                    }
                };
            }
            _ => {}
        }
    }
    
    let mut input_data = vec![];
    File::open(file_path.clone()).unwrap().read_to_end(&mut input_data).unwrap();

    let auto_detect = !encode && !decode;
    if auto_detect {
        let magic: &[u8; 4] = &input_data[..4].try_into().unwrap();
        if magic == b"smol" {
            decode = true;
        } else {
            encode = true;
        }
    }

    let total_time = SystemTime::now();

    if encode {
        print!("starting encode: ");
        stdout().flush().unwrap();
        let start_write = SystemTime::now();

        let mut smol_writer = SmolWriter::default();
        let mut input_text = match String::from_utf8(input_data.clone()) {
            Ok(input_text) => input_text,
            Err(err) => {
                println!("Error. invalid charaters found in file, could not create smol file, error: {}", err);
                return;
            }
        };
        let encoded_text = smol_writer.compress(&mut input_text);

        let mut output_file = filename.clone();
        while output_file.exists() {
            output_file = PathBuf::from(format!("{}.smol", output_file.to_string_lossy()));
        }

        fs::write(&output_file, &encoded_text).unwrap();
        println!("file written to {:?}, took {:?}", output_file, start_write.elapsed().unwrap());
        input_data = encoded_text;
    }

    if decode {
        print!("starting decode: ");
        stdout().flush().unwrap();
        let start_read = SystemTime::now();

        let mut smol_reader = SmolReader::default();
        let un_smol = match smol_reader.decompress(input_data) {
            Ok(un_smol) => un_smol,
            Err(err) => {
                println!("Error. Either this is not a smol file or the file is corrupt, error: {}", err);
                return;
            }
        };

        let mut output_file = filename;
        while output_file.exists() {
            output_file = PathBuf::from(format!("{}.unsmol", output_file.to_string_lossy()));
        }

        fs::write(&output_file, un_smol).unwrap();

        println!("file written to {:?}, took {:?}", output_file, start_read.elapsed().unwrap());
    }

    println!("\neverything done. took {:?}\n", total_time.elapsed().unwrap());

    if pause_before_exit {
        println!("send anything to exit");
        stdin().read(&mut [0u8]).unwrap();
    } else {
        for index in 0..10 {
            print!("\rautoexit in {}s ", 10 - index);
            stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
}