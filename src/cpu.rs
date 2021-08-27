use super::display::Display;
use super::opcode::Opcode;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// use std::time::Duration;

// const TIMING: Duration = Duration::from_millis(10);
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct State {
    pub memory: [u8; 4096],        // memory: ram access of 4 kB
    pub display_buf: Vec<Vec<u8>>, // display: 64 x 32 pixel monochrome display
    pub display: Display,
    pc: u16,         // program counter: points at current instr in memory
    i: u16,          // index register: points at wherever in memory
    stack: Vec<u16>, // stack: call/return from subroutines/functions
    vars: [u8; 16],  // register: general purpose variable registers
}

impl State {
    pub fn new(context: &sdl2::Sdl, scale: u32) -> Self {
        // storing the fonts in the first 10
        let mut memory = [0; 4096];
        for i in 0..80 {
            memory[i] = FONT[i];
        }
        let mut display = Display::new(&context, 10);
        Self {
            memory,
            display_buf: vec![vec![0; 64]; 32],
            display,
            pc: 0x200,
            i: 0x200,
            stack: Vec::new(),
            vars: [0; 16],
        }
    }

    pub fn load_rom(&mut self, filename: &str) {
        // create a path to the desired file
        let path = Path::new(filename);

        // open the path as an iterator of its bytes
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file.bytes(),
        };

        // paste the rom onto our memory array
        let mut cursor = self.pc;
        for byte in file {
            self.memory[cursor as usize] = byte.unwrap();
            cursor += 1;
        }
    }

    pub fn fetch(&mut self) -> Opcode {
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[self.pc as usize + 1];
        self.pc += 2;
        Opcode::from(byte1, byte2)
    }

    fn skip(&mut self, cond: bool) {
        if cond {
            self.pc += 2;
        }
    }

    fn sub(&mut self, left_sub_right: bool, code: Opcode) {
        let left = match left_sub_right {
            true => code.x,
            false => code.y,
        };
        let right = match left_sub_right {
            true => code.y,
            false => code.x,
        };
        self.vars[0xF] = match left > right {
            true => 1,
            false => 0,
        };
        self.vars[code.x as usize] = left - right
    }

    fn shift(&mut self, shift_left: bool, code: Opcode) {
        match shift_left {
            true => {
                if self.vars[code.x as usize] & 0x80 > 0 {
                    self.vars[0xF] = 1;
                }
                self.vars[code.x as usize] <<= 1
            }
            false => {
                if self.vars[code.x as usize] & 0x1 > 0 {
                    self.vars[0xF] = 1;
                }
                self.vars[code.x as usize] >>= 1
            }
        }
    }

    // https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
    pub fn process(&mut self, code: Opcode) -> Result<(), String> {
        match code.c {
            0x0 => match code.nnn {
                0x0E0 => {
                    self.display_buf.fill(vec![0; 64]);
                    self.display.wipe_screen()
                } // clear the screen
                0x0EE => self.pc = self.stack.pop().unwrap(), // return from a subroutine
                _ => println!("not implemented"),             // exec machine lang subroutine at NNN
            },
            0x1 => self.pc = code.nnn, // jump to NNN
            0x2 => {
                // exec subroutine starting at NNN
                self.stack.push(self.pc);
                self.pc = code.nnn;
            }
            0x3 => self.skip(self.vars[code.x as usize] == code.nn), // skip next instr if Vx == nn
            0x4 => self.skip(self.vars[code.x as usize] != code.nn), // skip next instr if Vx != nn
            0x5 => self.skip(self.vars[code.x as usize] == self.vars[code.y as usize]), // skip next instr if Vx == Vy
            0x6 => self.vars[code.x as usize] = code.nn, // store number NN in register VX
            0x7 => self.vars[code.x as usize] += code.nn, // add the value NN to register VX
            0x8 => match code.n {
                0x0 => self.vars[code.x as usize] = self.vars[code.y as usize], // Vx = Vy
                0x1 => self.vars[code.x as usize] |= self.vars[code.y as usize], // Vx |= Vy
                0x2 => self.vars[code.x as usize] &= self.vars[code.y as usize], // Vx &= Vy
                0x3 => self.vars[code.x as usize] ^= self.vars[code.y as usize], // Vx ^= Vy
                0x4 => self.vars[code.x as usize] += self.vars[code.y as usize], // Vx += Vy
                0x5 => self.sub(true, code),                                    // Vx -= Vy
                0x6 => self.shift(false, code), // bit shift Vx 1 right
                0x7 => self.sub(false, code),   // Vy -= Vx
                0xE => self.shift(true, code),  // bit shift Vx 1 left
                _ => println!("invalid command {}", code),
            },
            0x9 => self.skip(self.vars[code.x as usize] != self.vars[code.y as usize]), // skip next instr if Vx != Vy
            0xA => self.i = code.nnn, // store memory address NNN in register I
            0xB => println!("not implemented"),
            0xC => self.vars[code.x as usize] = code.nn & rand::random::<u8>(), // set Vx to NN bitwise and a random number
            0xD => self.draw(code),
            0xE => match code.nn {
                0x9E => println!("not implemented"), // skip if key beihng pressed
                0xA1 => println!("not implemented"), // skip if key not being pressed
                _ => println!("not implemented"),
            }, // skips if key being pre
            0xF => println!("not implemented"),
            _ => println!("not a valid hex code"),
        }
        Ok(())
    }

    fn draw(&mut self, code: Opcode) {
        // retrieve modded x and y from vx and vy
        let vx = (self.vars[code.x as usize] % 64) as usize;
        let vy = (self.vars[code.y as usize] % 32) as usize;
        self.vars[0xF] = 0;
        for row in 0..code.n as usize {
            let y = vy + row;
            if y > 31 {
                break;
            }
            let spr_data = self.memory[self.i as usize + row];
            for col in 0..8_usize {
                let x = vx + col;
                if x > 63 {
                    break;
                }
                let mask = spr_data & (1 << 7 - col);

                println!("x: {} - y: {}", x, y);
                if self.display_buf[y][x] > 0 && mask > 0 {
                    self.vars[0xF] = 1;
                    self.display.wipe_pixel(x as i32, y as i32);
                    self.display_buf[y][x] ^= mask;
                } else if self.display_buf[y][x] < 1 && mask < 1 {
                    self.vars[0xF] = 0;
                    self.display.wipe_pixel(x as i32, y as i32);
                    self.display_buf[y][x] ^= mask;
                } else {
                    self.vars[0xF] = 0;
                    self.display.fill_pixel(x as i32, y as i32);
                    self.display_buf[y][x] ^= mask;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }
}
