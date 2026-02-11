use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use std::process;

struct  ErrorInvalidFormat;

struct Interpreter {
    pub ac: i8,
    pub memory: Vec<u8>,
    pub zero: bool,
    pub negative: bool,
}

impl Interpreter {
    pub fn new(mut file: File) -> Result<Self, ErrorInvalidFormat> {
        let required_format: [u8; 4] = [3, 78, 68, 82]; // 03 N D R format 
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        if !buffer.starts_with(&required_format) {
            return Err(ErrorInvalidFormat);
        }

        let i = Interpreter { 
            ac:         0,
            memory:     buffer,
            zero:       true,
            negative:   false 
        };

        return Ok(i);
    }
}

fn main() {
    
    let file = File::open("./exemplo.bin").unwrap();
    let inter: Interpreter = match Interpreter::new(file) {
        Ok(i) => i,
        Err(ErrorInvalidFormat) => {
            println!("ERROR: invalid file format.");
            process::exit(1);
        }
    };
    
    for (i, byte_or_error) in inter.memory.bytes().enumerate() {
        let byte = byte_or_error.unwrap();
        print!(" {} ", byte);

        if (i + 1) % 16 == 0 {
            println!();
        }
    }
}
