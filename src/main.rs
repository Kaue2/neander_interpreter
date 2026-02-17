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

fn nop(i: &mut Interpreter, _address: usize) {
    // println!("Função nop");
    i.next();
    return;
}

fn add(i: &mut Interpreter, address: usize) {
    // println!("Função add");
    let val = i.memory[address];
    i.ac = i.ac.saturating_add(val);
    i.next();
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

    fn get_next_address(&mut self) -> usize {
        self.next(); // pega o endereço
        // pega a linha e converte pro endereço real do array
        let address = usize::from(self.memory[self.pc] * 2 + 4);
        address
    }

    pub fn process(&mut self, rules: HashMap<u8, InstructionFn>) {
        while !self.exit && self.pc < self.memory.len() {
            let opcode_or_address = self.memory[self.pc];
            //println!("Pc1: {}", self.pc);

            if let Some(instruction) = rules.get(&opcode_or_address) {
                if opcode_or_address == 0 || opcode_or_address == 240 {
                    instruction(self, 0);
                } else {
                    let address = self.get_next_address();
                    // println!("pc: {}", self.pc);
                    instruction(self, address);
                }
            } else {
                self.next();
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
    let file = File::open("./exemplo.bin").unwrap();
    let rules = get_rules();

    let mut inter = match Interpreter::new(file) {
        Ok(i) => i,
        Err(ErrorInvalidFormat) => {
            eprintln!("ERROR: formato inválido do buffer recebido");
            process::exit(1);
        },
    };

    println!("{}", inter);
    inter.process(rules);
}

#[cfg(test)]
mod tests {
    use crate::{Interpreter, get_rules};

    #[test]
    fn rules() {
        let mut i = Interpreter::default();
        i.memory = Vec::from([3, 78, 68, 82, 0, 0, 48, 0, 3, 0, 33, 0, 240, 0]);
        
        assert_eq!(i.pc, 4);
        assert_eq!(i.memory[i.pc], 0);

        i.process(get_rules());

        assert_eq!(i.ac, 33);
        assert_eq!(i.pc, 12); 
    }
}