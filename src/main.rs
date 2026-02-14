use core::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use std::process;

struct  ErrorInvalidFormat;
type InstructionFn = fn(&mut Interpreter, usize);

#[derive(Debug)]
struct Interpreter {
    ac: u8,
    pc: usize,
    memory: Vec<u8>,
    zero: bool,
    negative: bool,
    exit: bool, 
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            ac: 0,
            pc: 4,
            memory: Vec::new(),
            zero: true,
            negative: false,
            exit: false,
        }
    }
}

// funções

fn nop(_i: &mut Interpreter, _address: usize) {
    println!("Função nop");
    return;
}

fn add(i: &mut Interpreter, address: usize) {
    println!("Função add");
    let val = i.memory[address];
    i.ac = i.ac.saturating_add(val);
}

fn halt(i: &mut Interpreter, _address: usize) {
    println!("Função halt");
    i.exit = true;
}

fn get_rules() -> HashMap<u8, InstructionFn> { 
    let mut m: HashMap<u8, InstructionFn> = HashMap::new();
    m.insert(0, nop);
    m.insert(16, nop);
    m.insert(32, nop);
    m.insert(48, add);
    m.insert(64, nop);
    m.insert(80, nop);
    m.insert(96, nop);
    m.insert(128, nop);
    m.insert(144, nop);
    m.insert(160, nop);
    m.insert(240, halt);


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
            pc:         4,
            memory:     buffer,
            zero:       true,
            negative:   false,
            exit: false,
        };

        return Ok(i);
    }

    // vai pra próxima linha de instrução
    fn next(&mut self) {
        self.pc += 2;
    }

    pub fn process(&mut self, rules: HashMap<u8, InstructionFn>) {
        while !self.exit && self.pc < self.memory.len() {
            let opcode_or_address = self.memory[self.pc];
            println!("code: {}", opcode_or_address);
            if let Some(instruction) = rules.get(&opcode_or_address) {
                if opcode_or_address == 0 || opcode_or_address == 240 {
                    instruction(self, 0);
                    self.next();
                } else {
                    self.next();
                    println!("pc: {}", self.pc);
                    let address = usize::from(self.memory[self.pc] * 2 + 4); // pega a linha e converte pro endereço real do array
                    instruction(self, address);
                }
            };
            
        }
    }

    pub fn calculate_pc(&self) -> u8 {
        return ((self.pc.saturating_sub(4)) / 2) as u8;
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Interpreter state:")?;
        writeln!(f, "AC: {}", self.ac)?;
        writeln!(f, "PC: {}", self.calculate_pc())?;
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
    /*
    let file = File::open("./exemplo.bin").unwrap();
    let rules = get_rules();
    let mut inter: Interpreter = match Interpreter::new(file) {
        Ok(i) => i,
        Err(ErrorInvalidFormat) => {
            println!("ERROR: invalid file format.");
            process::exit(1);
        }
    };
     */
    let rules = get_rules();
    let mut inter: Interpreter = Default::default();
    inter.memory = Vec::from([3, 78, 68, 82, 0, 0, 48, 0, 3, 0, 33, 0, 240, 0]);
    inter.pc = 4;
    
    println!("{}", inter);
    inter.process(rules);
}

#[cfg(test)]
mod tests {
    use std::default;

    use crate::{Interpreter, get_rules};


    #[test]
    fn rules() {
        let mut i = Interpreter::default();
        i.memory = Vec::from([3, 78, 68, 82, 0, 0, 48, 0, 3, 0, 33, 0, 240, 0]);
        i.pc = 4;
        i.process(get_rules());

        assert_eq!(i.ac, 33);
    }
}