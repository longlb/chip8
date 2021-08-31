use super::display::Display;
use super::opcode::Opcode;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    memory: [u8; 4096],      // memory: ram access of 4 kB
    display_buf: [u8; 2048], // display buffer: 64 x 32 pixel boolean array
    display: Display,        // actual visual display
    pc: u16,                 // program counter: points at current instr in memory
    i: u16,                  // index register: points at wherever in memory
    stack: Vec<u16>,         // stack: call/return from subroutines/functions
    vars: [u8; 16],          // register: general purpose variable registers
    keys: [bool; 16],
    delay: u8,
    pub sound: u8,
}

impl State {
    pub fn new(context: &sdl2::Sdl, scale: u32) -> Self {
        // build the state object
        let mut item = Self {
            memory: [0; 4096],
            display_buf: [0; 2048],
            // generate a new display window
            display: Display::new(&context, scale),
            pc: 0x200,
            i: 0x200,
            stack: Vec::new(),
            vars: [0; 16],
            keys: [false; 16],
            delay: 0,
            sound: 0,
        };

        // store the font into memory
        item.paste(0, &FONT);
        item
    }

    pub fn decrement(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    // paste the contents of the given rom into memory
    pub fn load_rom(&mut self, filename: &str) {
        // create a path to the desired file
        let path = Path::new(filename);

        // open the path as an iterator of its bytes
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file.bytes(),
        };

        // paste the rom onto the beginning our memory array
        let mut cursor = self.pc;
        for byte in file {
            self.memory[cursor as usize] = byte.unwrap();
            cursor += 1;
        }
    }

    // return the next opcode instruction at pc in memory
    pub fn fetch(&mut self) -> Opcode {
        let byte1 = self.memory[self.pc as usize];
        let byte2 = self.memory[self.pc as usize + 1];
        self.pc += 2;
        Opcode::from(byte1, byte2)
    }

    // marks a key pressed or not pressed
    pub fn key_moved(&mut self, key: u8, pressed: bool) {
        self.keys[key as usize - 1] = pressed;
        // println!("{:?}", self.keys);
    }

    // https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
    // process the given opcode instruction
    pub fn process(&mut self, code: Opcode) -> Result<(), String> {
        let cx = self.vars[code.x as usize];
        match code.c {
            0x0 => match code.nnn {
                0x0E0 => self.clear(),                        // clear the screen
                0x0EE => self.pc = self.stack.pop().unwrap(), // return from a subroutine
                _ => println!("not implemented"),             // exec machine lang subroutine at NNN
            },
            0x1 => self.pc = code.nnn,        // jump to NNN
            0x2 => self.subroutine(code.nnn), // exec subroutine starting at NNN
            0x3 => self.skip(cx == code.nn),  // skip next instr if Vx == nn
            0x4 => self.skip(cx != code.nn),  // skip next instr if Vx != nn
            0x5 => self.skip(cx == self.vars[code.y as usize]), // skip next instr if Vx == Vy
            0x6 => self.vars[code.x as usize] = code.nn, // store number NN in register VX
            0x7 => self.vars[code.x as usize] = cx.wrapping_add(code.nn), // add the value NN to register VX
            0x8 => match code.n {
                0x0 => self.vars[code.x as usize] = self.vars[code.y as usize], // Vx = Vy
                0x1 => self.vars[code.x as usize] |= self.vars[code.y as usize], // Vx |= Vy
                0x2 => self.vars[code.x as usize] &= self.vars[code.y as usize], // Vx &= Vy
                0x3 => self.vars[code.x as usize] ^= self.vars[code.y as usize], // Vx ^= Vy
                0x4 => self.vars[code.x as usize] = self.add(code.x, code.y),   // Vx += Vy
                0x5 => self.vars[code.x as usize] = self.sub(code.x, code.y),   // Vx -= Vy
                0x6 => self.shift(false, code), // bit shift Vx 1 right
                0x7 => self.vars[code.x as usize] = self.sub(code.y, code.x), // Vy -= Vx
                0xE => self.shift(true, code),  // bit shift Vx 1 left
                _ => println!("invalid command {}", code),
            },
            0x9 => self.skip(cx != self.vars[code.y as usize]), // skip next instr if Vx != Vy
            0xA => self.i = code.nnn, // store memory address NNN in register I
            0xB => self.pc = code.nnn + self.vars[0] as u16,
            0xC => self.vars[code.x as usize] = code.nn & rand::random::<u8>(), // set Vx to NN bitwise and a random number
            0xD => self.draw(code.x, code.y, code.n)?,
            0xE => match code.nn {
                0x9E => self.skip(self.keys[cx as usize]), // skip if key being pressed
                0xA1 => self.skip(!self.keys[cx as usize]), // skip if key not being pressed
                _ => println!("invalid command {}", code),
            },
            0xF => match code.nn {
                0x07 => self.vars[code.x as usize] = self.delay,
                0x15 => self.delay = cx,
                0x18 => self.sound = cx,
                0x1E => self.i += cx as u16,
                0x0A => self.pause_til_key(code.x),
                0x29 => self.i = cx as u16 * 5,
                0x33 => self.dec_conv(self.i as usize, cx),
                0x55 => self.paste(self.i as usize, &self.vars.clone()),
                0x65 => self.load_regs(self.i as usize),
                _ => println!("invalid command {}", code),
            },
            _ => println!("not a valid hex code"),
        }
        Ok(())
    }

    // enter a subroutine
    fn subroutine(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    // draw the given sprite onto the display
    fn draw(&mut self, x: u8, y: u8, n: u8) -> Result<(), String> {
        // retrieve modded x and y from vx and vy
        let vx = (self.vars[x as usize] % 64) as usize;
        let vy = (self.vars[y as usize] % 32) as usize;
        // reset register f to 0
        self.vars[0xF] = 0;

        // loop over each row
        for row in 0..n as usize {
            // check that row y within bounds
            let y = vy + row;
            if y > 31 {
                break;
            }
            // retrieve the row's sprite data
            let spr_data = self.memory[self.i as usize + row];
            // loop through each pixel in the row
            for col in 0..8 as usize {
                // check that col x within bounds
                let x = vx + col;
                if x > 63 {
                    break;
                }
                // make a shifting mask for the sprite bits
                let mask = spr_data & (1 << 7 - col);
                // if both pixels in buf and sprite are the same, blank out the pixel
                let diff_pixel = (self.display_buf[y * 64 + x] > 0) ^ (mask > 0);
                // set register f to on if both pixels lit, else off
                match !diff_pixel && mask > 0 {
                    true => self.vars[0xF] = 1,
                    false => self.vars[0xF] = 0,
                }
                // draw or erase the pixel at x,y based on if they are the same
                self.display.draw_pixel(x as i32, y as i32, diff_pixel);
                // change the pixel in the display array based on mask
                self.display_buf[y * 64 + x] ^= mask;
            }
        }
        // show drawn changes to screen
        self.display.present();
        Ok(())
    }

    // clear the display of sprite
    fn clear(&mut self) {
        self.display_buf.fill(0);
        self.display.wipe();
        self.display.present()
    }

    // skip past the next opcode in memory if condition passes
    fn skip(&mut self, cond: bool) {
        if cond {
            self.pc += 2;
        }
    }

    // add the values in two registers
    fn add(&mut self, left: u8, right: u8) -> u8 {
        let test = self.vars[left as usize] as u16 + self.vars[right as usize] as u16;
        self.vars[0xF] = match test > 255 {
            true => 1,
            false => 0,
        };
        test as u8 % 255
    }

    // subtract the values in two registers
    fn sub(&mut self, left: u8, right: u8) -> u8 {
        self.vars[0xF] = match self.vars[left as usize] > self.vars[right as usize] {
            true => 1,
            false => 0,
        };
        self.vars[left as usize].wrapping_sub(self.vars[right as usize])
    }

    // shift a Vx's value either left or right
    fn shift(&mut self, shift_left: bool, code: Opcode) {
        match shift_left {
            true => {
                self.vars[0xF] = match self.vars[code.x as usize] & 0x80 > 0 {
                    true => 1,
                    false => 0,
                };
                self.vars[code.x as usize] <<= 1
            }
            false => {
                self.vars[0xF] = match self.vars[code.x as usize] & 0x1 > 0 {
                    true => 1,
                    false => 0,
                };
                self.vars[code.x as usize] >>= 1
            }
        }
    }

    // paste registers into memory
    fn paste(&mut self, index: usize, arr: &[u8]) {
        let mut curs = index;
        for byte in arr {
            self.memory[curs] = *byte;
            curs += 1;
        }
    }

    // paste memory into registers
    fn load_regs(&mut self, index: usize) {
        println!("nok");
        println!("{:?}", self.i);
        println!("{:?}", self.vars);
        for i in self.i..self.i + 16 {
            println!("{}: {}", i, self.memory[i as usize]);
        }
        for ind in 0..16 {
            self.vars[ind] = self.memory[ind + index];
        }
        println!("ok");
        println!("{:?}", self.i);
        println!("{:?}", self.vars);
        for i in self.i..self.i + 16 {
            println!("{}: {}", i, self.memory[i as usize]);
        }
    }

    // load Vx into memory as digits of its value
    fn dec_conv(&mut self, index: usize, val: u8) {
        let mut val = val;
        for i in 3..0 {
            self.memory[index + i - 1] = val % 10;
            val /= 10;
        }
    }

    // wait until a key is pressed
    fn pause_til_key(&mut self, vx: u8) {
        match self.keys.iter().position(|&x| x) {
            Some(x) => self.vars[vx as usize] = x as u8,
            None => self.i -= 2,
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pc: {}   i: {}\nstack: {:?}\nregisters: {:?}\n keys: {:?}\ndelay: {} sound: {}",
            self.pc, self.i, self.stack, self.vars, self.keys, self.delay, self.sound
        )
    }
}
