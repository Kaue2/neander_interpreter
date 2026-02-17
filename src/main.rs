use core::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use std::process;

struct  ErrorInvalidFormat;
type InstructionFn = fn(&mut Interpreter, usize);

#[derive(Debug)]
struct  ProgramCounter(u8);

impl ProgramCounter {
    fn increment(&mut self) {
        self.0 = self.0.saturating_add(2);
    }

    fn pos(&self) -> usize{
        self.0 as usize
    }

    // converte o endereço virtual para o endereço físico
    fn address(&self) -> u8 {
        (self.0.saturating_sub(4)) / 2 
    }

    fn value(&self) -> u8 {
        self.0
    }
 }

#[derive(Debug)]
struct Interpreter {
    ac: u8,
    pc: ProgramCounter,
    memory: Vec<u8>,
    zero: bool,
    negative: bool,
    exit: bool, 
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            ac: 0,
            pc: ProgramCounter(4),
            memory: Vec::new(),
            zero: true,
            negative: false,
            exit: false,
        }
    }
}

// funções

fn nop(_i: &mut Interpreter, _address: usize) {
    // println!("Função nop");
    return;
}

fn add(i: &mut Interpreter, address: usize) {
    // println!("Função add");
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
            pc:         ProgramCounter(4),
            memory:     buffer,
            zero:       true,
            negative:   false,
            exit: false,
        };

        return Ok(i);
    }

    fn get_next_address(&mut self) -> usize {
        self.pc.increment(); // pega o endereço
        // pega a linha e converte pro endereço real do array
        let address = usize::from(self.memory[self.pc.pos()] * 2 + 4);
        address
    }

    pub fn process(&mut self, rules: HashMap<u8, InstructionFn>) {
        while !self.exit && self.pc.pos() < self.memory.len() {
            let opcode_or_address = self.memory[self.pc.pos()];
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
                self.pc.increment();
            };
            self.pc.increment();
        }
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Interpreter state:")?;
        writeln!(f, "AC: {}", self.ac)?;
        writeln!(f, "PC: {}", self.pc.address())?;
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
    use std::fs::File;

    #[test]
    fn rules() {
        let mut i = Interpreter::default();
        i.memory = Vec::from([3, 78, 68, 82, 0, 0, 48, 0, 3, 0, 33, 0, 240, 0]);
        
        assert_eq!(i.pc.value(), 4);
        assert_eq!(i.memory[i.pc.pos()], 0);

        i.process(get_rules());

        assert_eq!(i.ac, 33);
        assert_eq!(i.pc.value(), 14); 
    }

    #[test]
    fn process() {
        let file = File::open("./exemplo.bin").unwrap();
        let rules = get_rules();
        if let Ok(mut inter) = Interpreter::new(file) {
            inter.process(rules);

            assert_eq!(inter.ac, 33);
            assert_eq!(inter.pc.address(), 5);
        }
    }
}