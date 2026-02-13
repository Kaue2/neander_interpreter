use core::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use std::process;

struct  ErrorInvalidFormat;
type InstructionFn = fn(&mut Interpreter, u8);

struct Interpreter {
    pub ac: u8,
    pub pc: usize,
    pub memory: Vec<u8>,
    pub zero: bool,
    pub negative: bool,
}

// funções

fn nop(_i: &mut Interpreter, _val: u8) {
    return;
}

fn add(i: &mut Interpreter, val: u8) {
    i.ac.saturating_add(val);
}

fn get_rules() -> HashMap<u8, InstructionFn> {
    let mut m: HashMap<u8, InstructionFn> = HashMap::new();
    m.insert(0, nop);
    m.insert(16, todo!());
    m.insert(32, todo!());
    m.insert(48, add);
    m.insert(64, todo!());
    m.insert(80, todo!());
    m.insert(96, todo!());
    m.insert(128, todo!());
    m.insert(144, todo!());
    m.insert(160, todo!());
    m.insert(240, todo!());


    return m;
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

    pub fn process(&mut self, rules: HashMap<u8, InstructionFn>) {
        let mut instruction = true;
        for pair in self.memory.chunks(2) .skip(2) { // pulando 2 primeiros pares
            match rules.get(&pair[0]) {
                Some(_) => {},
                None => {}
            }
            println!("{} {}", pair[0], pair[1])
        }
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



fn main() {
    let file = File::open("./exemplo.bin").unwrap();
    let rules = get_rules();
    let mut inter: Interpreter = match Interpreter::new(file) {
        Ok(i) => i,
        Err(ErrorInvalidFormat) => {
            println!("ERROR: invalid file format.");
            process::exit(1);
        }
    };
    
    println!("{}", inter);
    inter.process(rules);
}
