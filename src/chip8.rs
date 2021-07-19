use std::io;
use std::io::prelude::*;
use std::fs::File;
use rand::Rng;

pub struct Cpu {
    fontset: [u8;80],
    memory: [u8;4096],
    pub v: [u8;16],
    i: u16,
    pc: u16,
    pub screen: [u32;32*64],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: u16,
    pub keys: [bool; 16],
    pub is_wating: bool,
    pub key_register: usize,
}

impl Cpu {
    pub fn initialize(&mut self) {
        let mut i = 0;
        while i < self.fontset.len() {
            self.memory[i] = self.fontset[i];
            i+=1;
        }
    }
    pub fn load_rom(&mut self, name: &str) -> io::Result<()> {
        let mut file = File::open(name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Couldn't read file!");

        let mut i = 0;
        while i < buffer.len() {
            self.memory[512 + i] = buffer[i];
            i+=1;
        }
        Ok(())
    }
    pub fn do_cycle(&mut self) {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc+1) as usize] as u16;

        // Break up into nibbles to identify opcodes
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];

        self.pc += 2;

        match (op_1, op_2, op_3, op_4) {
            // CLS
            (0, 0, 0xE, 0) => self.clear_screen(),
            // RET
            (_, _, 0xE, 0xE) => {
                self.pc = self.stack[self.stack_pointer as usize];
                self.stack_pointer -= 1;
            },
            // JP nnn
            (1, _, _, _) => self.pc = nnn,
            // CALL nnn
            (2, _, _, _) => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.pc;
                self.pc = nnn;
            },
            // SE Vx, kk
            (3, _, _, _) => self.pc += if vx == kk {2} else {0},
            // SNE Vx, kk
            (4, _, _, _) => self.pc += if vx != kk {2} else {0},
            // SE Vx, Vy
            (5, _, _, 0) => self.pc += if vx == vy {2} else {0},
            // LD Vx, kk
            (6, _, _, _) => self.v[x] = kk,
            // ADD Vx, kk
            (7, _, _, _) => {
                self.v[x] = self.v[x].wrapping_add(kk);
            },
            // LD Vx, Vy
            (8, _, _, 0) => self.v[x] = vy,
            // OR Vx, Vy
            (8, _, _, 1) => self.v[x] = vx | vy,
            // AND Vx, Vy
            (8, _, _, 2) => self.v[x] = vx & vy,
            // XOR Vx, Vy
            (8, _, _, 3) => self.v[x] = vx ^ vy,
            // ADD Vx, Vy
            (8, _, _, 4) => {
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                match overflow {
                    true => self.v[15] = 1,
                    false => self.v[15] = 0,
                }
                self.v[x] = res;
            },
            // SUB Vx, Vy
            (8, _, _, 5) => {
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                match overflow {
                    true => self.v[15] = 0,
                    false => self.v[15] = 1,
                }
                self.v[x] = res;
            },
            // SHR Vx, Vy
            (8, _, _, 6) => {
                self.v[15] = vx & 0x1;
                self.v[x] >>= 1;
            },
            // SUBN Vx, Vy
            (8, _, _, 7) => {
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                match overflow {
                    true => self.v[15] = 0,
                    false => self.v[15] = 1,
                }
                self.v[x] = res;
            },
            // SHL Vx
            (8, _, _, 0xE) => {
                self.v[15] = (vx & 0x80) >> 7;
                self.v[x] <<= 1;
            },
            // SNE Vx, Vy
            (9, _, _, 0) => self.pc += if vx != vy {2} else {0},
            // LD I, nnn
            (0xA, _, _, _) => self.i = nnn,
            // JP V0, nnn
            (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16, //TODO: overflow handling
            // RND Vx, byte
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                self.v[x] = rng.gen::<u8>() & kk;
            },
            // DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                self.draw(vx as usize, vy as usize, n as u16);
            }
            // SKP Vx
            (0xE, _, 9, 0xE) => self.pc += if self.keys[vx as usize] {2}else {0},
            // SKNP Vx
            (0xE, _, 0xA, 1) => self.pc += if !self.keys[vx as usize] {2}else {0},
            // LD Vx, DT
            (0xF, _, 0, 7) => self.v[x] = self.delay_timer,
            // LD Vx, K
            (0xF, _, 0, 0xA) => {
                self.is_wating = true;
                self.key_register = x;
            },
            // LD DT, Vx
            (0xF, _, 1, 5) => self.delay_timer = vx,
            // LD ST, Vx
            (0xF, _, 1, 8) => self.sound_timer = vx,
            // ADD I, Vx
            (0xF, _, 1, 0xE) => self.i += vx as u16,
            // LD F, Vx
            (0xF, _, 2, 9) => self.i = vx as u16 * 5,
            // LD B, Vx
            (0xF, _, 3, 3) => {
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i as usize + 1] = (vx / 10) % 10;
                self.memory[self.i as usize + 2] = (vx % 100) % 10;
            },
            // LD [I], Vx
            (0xF, _, 5, 5) => self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]
                .copy_from_slice(&self.v[0..(x as usize + 1)]),
            // LD Vx, [I]
            (0xF, _, 6, 5) => self.v[0..(x as usize + 1)]
                .copy_from_slice(&self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]),
            (_, _, _, _) => println!("Unhandled opcode {}", format!("{:X}", opcode)),
        }
        if self.delay_timer > 0 {self.delay_timer -= 1;}
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            println!("\u{0007}");
        }
        
    }

    fn clear_screen(&mut self) {
        self.screen = [0;32*64];
    }

    fn draw(&mut self, x: usize, y: usize, height: u16) {
        let sprite = &self.memory[self.i as usize .. (self.i + height) as usize];
        self.v[0xF] = 0;

        for j in 0..height as usize {
            let line = sprite[j];

            for i in 0..8 {
                if line & (0x80 >> i) != 0 {
                    let pixel = ((x + i) % 64) + ((y +j) % 32) * 64;
                    if self.screen[pixel] == 0xffffffff {
                        self.v[0xF] = 1;
                        self.screen[pixel] = 0xff000000; 
                    }else {
                        self.screen[pixel] = 0xffffffff; 
                    }
                }
            }
        }
    }
}
pub const fn create_cpu() -> Cpu {
    let cpu =  Cpu {
        fontset: [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ],
        
        memory: [0;4096],
        v: [0;16],
        i: 0,
        pc: 0x200,
        screen: [0;32*64],
        
        delay_timer: 0,
        sound_timer: 0,
        
        stack: [0;16],
        stack_pointer: 0,
        
        keys: [false; 16],
        is_wating: false,
        key_register: 0,
    };
    return cpu
}