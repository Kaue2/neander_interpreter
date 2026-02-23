use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::process;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::{execute};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::widgets::{ScrollbarState};
use ratatui::{Terminal};
use ratatui::prelude::{Backend, CrosstermBackend};
use crate::tui::ui::ui;
mod tui;

struct  ErrorInvalidFormat;
type InstructionFn = fn(&mut Interpreter, usize);

#[derive(Debug)]
struct  ProgramCounter(u8);

#[derive(Default)]
struct App {
    pub program_scroll_state: ScrollbarState,
    pub program_scroll: usize,
    pub data_scroll_state: ScrollbarState,
    pub data_scroll: usize,
}

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

    // fn value(&self) -> u8 {
    //     self.0
    // }
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
    // println!("Função halt");
    i.exit = true;
}

fn sta(i: &mut Interpreter, address: usize) {
    // println!("Função sta");
    i.memory[address] = i.ac;
}

fn lda(i: &mut Interpreter, address: usize) {
    // println!("Função lda");
    i.ac = i.memory[address];
}

fn or(i: &mut Interpreter, address: usize) {
    // println!("Função or");
    i.ac = i.memory[address] | i.ac;
}

fn and(i: &mut Interpreter, address: usize) {
    // println!("Função and");
    i.ac = i.memory[address] & i.ac;
}

fn not(i: &mut Interpreter, _address: usize) {
    // println!("Função not");
    i.ac = !i.ac;
}

fn jmp(i: &mut Interpreter, address: usize) {
    // println!("Função jmp");
    i.pc.0 = address as u8;
}

fn jn(i: &mut Interpreter, address: usize) {
    // println!("Função jn");
    if i.negative {
        i.pc.0 = address as u8;
    }
}

fn jz(i: &mut Interpreter, address: usize) {
    // println!("Função jz");
    if i.zero {
        i.pc.0 = address as u8;
    }
}

fn get_rules() -> HashMap<u8, InstructionFn> { 
    let mut m: HashMap<u8, InstructionFn> = HashMap::new();
    m.insert(0, nop);
    m.insert(16, sta);
    m.insert(32, lda);
    m.insert(48, add);
    m.insert(64, or);
    m.insert(80, and);
    m.insert(96, not);
    m.insert(128, jmp);
    m.insert(144, jn);
    m.insert(160, jz);
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

    fn fetch(&mut self) -> u8 {
        let val = self.memory[self.pc.pos()];
        self.pc.increment();
        val
    }   

    fn get_next_address(&mut self) -> usize {
        // pega a linha e converte pro endereço real do array
        // println!("{}", self.pc.pos());
        let address = usize::from(self.memory[self.pc.pos()]) * 2 + 4;
        self.pc.increment(); // pega o endereço
        address
    }

    fn check_flags(&mut self) {
        if self.ac == 0 {
            self.zero = true;
        } else {
            self.zero = false;
        }

        if self.ac >= 128 {
            self.negative = true;
        } else {
            self.negative = false;
        }
    }

    pub fn process(&mut self, rules: HashMap<u8, InstructionFn>) {
        while !self.exit && self.pc.pos() < self.memory.len() {
            let opcode = self.fetch();

            if let Some(instruction) = rules.get(&opcode) {
                if opcode == 0 || opcode == 240 {
                    instruction(self, 0);
                } else {
                    let address = self.get_next_address();
                    instruction(self, address);
                }
            } else {
                self.pc.increment();
            };
            self.check_flags();
        }
    }

    pub fn convert_data(&self) -> Vec<String> {
        let mut pos: usize = 0;
        let mut instruction = true;
        let relevant_data = self.memory.get(4..).unwrap_or(&[]);
        let mut str_data: Vec<String> = Vec::new();
        
        while pos < relevant_data.len() {
            if instruction {
                if let Some(mnemonic) = match relevant_data[pos] {
                    0 => Some("NOP"),
                    16 => Some("STA"),
                    32 => Some("LDA"),
                    48 => Some("ADD"),
                    64 => Some("OR"),
                    80 => Some("AND"),
                    96 => Some("NOT"),
                    128 => Some("JMP"),
                    144 => Some("JN"),
                    160 => Some("JZ"),
                    240 => Some("HLT"),
                    _ => None
                } {
                    str_data.push(mnemonic.to_string());
                    if mnemonic != "NOP" && mnemonic != "HLT" {
                        instruction = false;
                    } else {
                        instruction = true;
                    }
                } else {
                    str_data.push(relevant_data[pos].to_string());
                }
            } else {
                str_data.push(relevant_data[pos].to_string());
                instruction = true;
            }
            pos += 2;
        }
        str_data
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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    interpreter: &mut Interpreter,
    mut app: App
) -> Result<bool, Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, interpreter, &mut app))
            .map_err(|e| e.to_string())?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Down => {
                    app.program_scroll = app.program_scroll.saturating_add(1);
                    app.program_scroll_state = 
                        app.program_scroll_state.position(app.program_scroll);
                }
                KeyCode::Up => {
                    app.program_scroll = app.program_scroll.saturating_sub(1);
                    app.program_scroll_state = 
                        app.program_scroll_state.position(app.program_scroll);
                }
                KeyCode::Char('j') => {
                    app.data_scroll = app.data_scroll.saturating_add(1);
                    app.data_scroll_state = 
                        app.data_scroll_state.position(app.data_scroll);
                }
                KeyCode::Char('k') => {
                    app.data_scroll = app.data_scroll.saturating_sub(1);
                    app.data_scroll_state = 
                        app.data_scroll_state.position(app.data_scroll);
                }
                KeyCode::Char('q') => {
                    return Ok(true);
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let file = File::open("./exemplo.bin").unwrap();
    let rules = get_rules();

    let mut inter = match Interpreter::new(file) {
        Ok(i) => i,
        Err(ErrorInvalidFormat) => {
            eprintln!("ERROR: formato inválido do buffer recebido");
            process::exit(1);
        },
    };

    let app = App::default();

    inter.process(rules);
    let _res = run_app(&mut terminal, &mut inter, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Interpreter, get_rules};
    use std::fs::File;

    #[test]
    fn rules() {
        let mut i = Interpreter::default();
        i.memory = Vec::from([3, 78, 68, 82, 0, 0, 48, 0, 3, 0, 33, 0, 240, 0]);
        
        // assert_eq!(i.pc.value(), 4);
        assert_eq!(i.memory[i.pc.pos()], 0);

        i.process(get_rules());

        assert_eq!(i.ac, 33);
        // assert_eq!(i.pc.value(), 14); 
    }

    #[test]
    fn process() {
        let file = File::open("./exemplo.bin").unwrap();
        let rules = get_rules();
        if let Ok(mut inter) = Interpreter::new(file) {
            inter.process(rules);
            let elements = inter.convert_data();
            println!("Len: {}", elements.len());

            println!("{:?}", elements);
        }
    }
}