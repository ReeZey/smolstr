use std::{fs::{self, File}, io::{Read, stdin, stdout, Write}, time::{SystemTime, Duration}, env, thread};

mod smol_utils;
mod smol_writer;
mod smol_reader;
use smol_writer::SmolWriter;
use smol_reader::SmolReader;

fn main() {
    let total_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();

    println!("--- smolstr utility ---");
    println!("this will create an .smol and .unsmol file in working directory\n");
    thread::sleep(Duration::from_secs(1));
    
    let filename = match args.len() > 1 {
        true => args[1].clone(),
        false => {
            println!("its possible to use this utility with cli (smolstr.exe <file>)");
            print!("or enter dynamic or fullpath here: ");
            stdout().flush().unwrap();

            let mut string = String::new();
            stdin().read_line(&mut string).unwrap();

            string = string.replace("\n", "");
            string = string.replace("\r", "");

            string
        }
    };

    println!("starting to convert {:?}", filename);

    //let mut text: String = String::new();
    let mut buffer = vec![];
    File::open(&filename).unwrap().read_to_end(&mut buffer).unwrap();
    let text = String::from_utf8_lossy(&mut buffer).to_string();
    //let text = String::from("yoaaaaefaeg");

    let start_write = SystemTime::now();
    let mut smol_writer = SmolWriter::default();
    let encoded_text = smol_writer.compress(&text);
    println!("write took {:?}", start_write.elapsed().unwrap());

    fs::write(format!("{}.smol",filename), &encoded_text).unwrap();

    let start_read = SystemTime::now();
    let mut smol_reader = SmolReader::default();
    let un_smol = smol_reader.decompress(encoded_text);
    fs::write(format!("{}.unsmol",filename), un_smol).unwrap();
    println!("read took {:?}", start_read.elapsed().unwrap());

    println!("done, took {:?}", total_time.elapsed().unwrap());
}