use core::fmt;
use std::collections::HashMap;
use std::fmt::write;
use std::fs::File;
use std::io::{Read};
use std::process;

struct  ErrorInvalidFormat;

struct Interpreter {
    pub ac: i8,
    pub pc: usize,
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
            pc:         0,
            memory:     buffer,
            zero:       true,
            negative:   false 
        };

        return Ok(i);
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Interpreter state:")?;
        writeln!(f, "AC: {}", self.ac)?;
        writeln!(f, "PC: {}", self.pc)?;
        writeln!(f, "Memory:")?;

        for (i, byte) in self.memory.iter().enumerate() {
            write!(f, " {:02X} ", byte)?;

            if (i + 1) % 16 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn get_rules() -> HashMap<u8, String> {
    let mut m = HashMap::new();
    m.insert(0, "NOP".to_string());
    m.insert(16, "STA".to_string());
    m.insert(32, "LDA".to_string());
    m.insert(48, "ADD".to_string());
    m.insert(64, "OR".to_string());
    m.insert(80, "AND".to_string());
    m.insert(96, "NOT".to_string());
    m.insert(128, "JMP".to_string());
    m.insert(144, "JN".to_string());
    m.insert(160, "NOT".to_string());
    m.insert(240, "NOT".to_string());


    return m;
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
    
    println!("{}", inter);
}
