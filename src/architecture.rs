mod stack;
mod utils;
use stack::Stack;
use utils::Hex;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const RAM_SIZE: usize = 0x1000;

#[derive(Debug, Clone, Copy)]
pub struct Architecture {
    ram: [u8; RAM_SIZE],
    stack: Stack,
    display: [u8; WIDTH * HEIGHT],
    v: [u8; 16],
    i: u16,
    pc: u16,
    dt: u8,
    st: u8,
}
impl Architecture {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            stack: Stack::new(),
            display: [0; WIDTH * HEIGHT],
            v: [0; 16],
            i: 0,
            pc: 0,
            dt: 0,
            st: 0,
        }
    }
}
impl Architecture {
    pub fn execute(self: &mut Self, rom: &Vec<u16>) -> () {
        let instruction = rom[self.pc as usize];
        match instruction {
            0x00E0 => self.clear(),
            0x00EE => self.ret(),
            0x1000..=0x1FFF => self.jp(instruction),
            0x2000..=0x2FFF => self.call(instruction),
            0x3000..=0x3FFF => self.s_e_byte(instruction),
            0x4000..=0x4FFF => self.s_n_e_byte(instruction),
            0x5000..=0x5FFF => self.s_e_register(instruction),
            0x6000..=0x6FFF => self.load_byte(instruction),
            0x7000..=0x7FFF => self.add_byte(instruction),
            0x8000..=0x8FFF => match instruction & 0xF {
                0x0 => self.ld(instruction),
                0x1 => self.or(instruction),
                0x2 => self.and(instruction),
                0x3 => self.xor(instruction),
                0x4 => self.add(instruction),
                0x5 => self.sub(instruction),
                0x6 => self.shr(instruction),
                0x7 => self.subn(instruction),
                0xE => self.shl(instruction),
                _ => panic!("OpCode does not exist!"),
            },
            0x9000..=0x9FFF => self.s_n_e(instruction),
            0xA000..=0xAFFF => self.ld_i(instruction),
            0xB000..=0xBFFF => self.jp_v0(instruction),
            0xC000..=0xCFFF => self.rnd(instruction),
            0xD000..=0xDFFF => self.drw(instruction),
            0xE000..=0xEFFF => match instruction & 0xFF {
                0x9E => self.skp(instruction),
                0xA1 => self.sknp(instruction),
                _ => panic!("OpCode does not exist!"),
            },
            0xF000..=0xFFFF => match instruction & 0xFF {
                0x07 => self.ld_reg_dt(instruction),
                0x0A => self.ld_wait(instruction),
                0x15 => self.ld_dt_reg(instruction),
                0x18 => self.ld_st(instruction),
                0x1E => self.add_i(instruction),
                0x29 => self.ld_loc(instruction),
                0x33 => self.ld_bcd(instruction),
                0x55 => self.store_regs(instruction),
                0x65 => self.read_regs(instruction),
                _ => panic!("OpCode does not exist!"),
            },
            _ => panic!("OpCode does not exist!"),
        }
        self.pc += 1;
    }
}
impl Architecture {
    /// 00E0 - CLS
    ///
    /// Clear the display.
    fn clear(self: &mut Self) -> () {
        self.display = [0u8; 64 * 32];
    }

    /// 00EE - RET
    ///  
    /// Return from a subroutine.
    ///  
    /// The interpreter sets the program counter to the address at the top of
    /// the stack, then subtracts 1 from the stack pointer.
    fn ret(self: Self) -> () {
        todo!();
    }

    /// 1nnn - JP addr
    ///  
    /// Jump to location nnn.
    ///  
    /// The interpreter sets the program counter to nnn.
    fn jp(self: &mut Self, instruction: u16) -> () {
        self.pc = instruction & 0xFFF;
    }

    /// 2nnn - CALL addr
    ///
    /// Call subroutine at nnn.
    ///
    /// The interpreter increments the stack pointer,
    /// then puts the current PC on the top of the stack.
    /// The PC is then set to nnn.
    fn call(self: &mut Self, instruction: u16) -> () {
        self.stack.sp += 1;
        self.stack.push(self.pc);
        self.pc = instruction & 0xFFF;
    }

    /// 3xkk - SE Vx, byte
    ///
    /// Skip next instruction if Vx == kk.
    ///
    /// The interpreter compares register Vx to kk,
    /// and if they are equal, increments the program counter by 2.
    fn s_e_byte(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    /// 4xkk - SNE Vx, byte
    ///
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk,
    /// and if they are not equal, increments the program counter by 2.
    fn s_n_e_byte(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    /// 5xy0 - SE Vx, Vy
    ///
    /// Skip next instruction if Vx == Vy.
    ///
    /// The interpreter compares register Vx to register Vy,
    /// and if they are equal, increments the program counter by 2.
    fn s_e_register(self: &mut Self, instruction: u16) -> () {
        if (instruction & 0xF) != 0x0 {
            panic!("OpCode does not exist!")
        };
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    /// 6xkk - LD Vx, byte
    ///
    /// Set Vx = kk.
    ///
    /// The interpreter puts the value kk into register Vx.
    fn load_byte(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
        self.v[x] = kk;
    }

    /// 7xkk - ADD Vx, byte
    ///
    /// Set Vx = Vx + kk.
    ///
    /// Adds the value kk to the value of register Vx,
    /// then stores the result in Vx.
    fn add_byte(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
        self.v[x] += kk;
    }

    /// 8xy0 - LD Vx, Vy
    ///
    /// Set Vx = Vy.
    ///
    /// Stores the value of register Vy in register Vx.
    ///
    fn ld(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        self.v[x] = self.v[y];
    }

    /// 8xy1 - OR Vx, Vy
    ///
    /// Set Vx = Vx OR Vy.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise OR compares the corrseponding bits from two
    /// values, and if either bit is 1, then the same bit in the result is
    /// also 1. Otherwise, it is 0.
    ///
    fn or(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        self.v[x] = self.v[x] | self.v[y];
    }

    /// 8xy2 - AND Vx, Vy
    ///
    /// Set Vx = Vx AND Vy.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise AND compares the corresponding bits from two
    /// values, and if if both bits are 1, then the same bit in the result is
    /// also 1. Otherwise, it is 0.
    fn and(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        self.v[x] = self.v[x] & self.v[y];
    }

    /// 8xy3 - XOR Vx, Vy
    ///
    /// Set Vx = Vx XOR Vy.
    ///
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    /// the result in Vx. An exclusive OR compares the corrseponding bits from
    /// two values, and if the bits are not both the same, then the corresponding
    /// bit in the result is set to 1. Otherwise, it is 0.
    fn xor(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        self.v[x] = self.v[x] ^ self.v[y];
    }

    /// 8xy4 - ADD Vx, Vy
    ///
    /// Set Vx = Vx + Vy, set VF = carry.
    ///
    /// The values of Vx and Vy are added together. If the result is greater than
    /// 8 bits (i.e., > 255,) VF is set to 1,
    /// otherwise 0. Only the lowest 8 bits of the result are kept,
    /// and stored in Vx.
    fn add(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        let sum: u16 = self.v[x] as u16 + self.v[y] as u16;
        if sum > 0x0FF {
            let sum: u8 = (sum >> 1 * 4).try_into().unwrap();
            self.v[x] = sum;
            self.v[0xF] = 1;
        } else {
            let sum: u8 = sum.try_into().unwrap();
            self.v[x] = sum;
            self.v[0xF] = 1;
        }
    }

    /// 8xy5 - SUB Vx, Vy
    ///
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from
    /// Vx, and the results stored in Vx.
    fn sub(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
        let subs: u8 = self.v[x] - self.v[y];
        self.v[x] = subs;
    }

    /// 8xy6 - SHR Vx {, Vy}
    ///
    /// Set Vx = Vx SHR 1.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1,
    /// otherwise 0. Then Vx is divided by 2.
    fn shr(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        self.v[0xF] = self.v[x] & 0x1;
        self.v[x] >>= 1;
    }

    /// 8xy7 - SUBN Vx, Vy
    ///
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from
    /// Vy, and the results stored in Vx.
    fn subn(self: &mut Self, instruction: u16) -> () {
        self.sub(Hex::swap_hex_digits(instruction, 1, 2));
    }

    /// 8xyE - SHL Vx {, Vy}
    ///
    /// Set Vx = Vx SHL 1.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to
    /// 0. Then Vx is multiplied by 2.
    fn shl(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        self.v[0xF] = self.v[x] >> 7;
        self.v[x] <<= 1;
    }

    /// 9xy0 - SNE Vx, Vy
    ///
    /// Skip next instruction if Vx != Vy.
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the
    /// program counter is increased by 2.
    fn s_n_e(self: &mut Self, instruction: u16) -> () {
        let x: usize = ((instruction & 0x0F00) >> 2 * 4).try_into().unwrap();
        let y: usize = ((instruction & 0x00F0) >> 1 * 4).try_into().unwrap();
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    /// Annn - LD I, addr
    /// 
    /// Set I = nnn.
    ///
    /// The value of register I is set to nnn.
    fn ld_i(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Bnnn - JP V0, addr
    /// 
    /// Jump to location nnn + V0.
    ///
    /// The program counter is set to nnn plus the value of V0.
    fn jp_v0(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Cxkk - RND Vx, byte
    /// 
    /// Set Vx = random byte AND kk.
    ///
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx.
    fn rnd(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Dxyn - DRW Vx, Vy, nibble
    /// 
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set
    /// VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address
    /// stored in I. These bytes are then displayed as sprites on screen at
    /// coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
    /// If this causes any pixels to be erased, VF is set to 1, otherwise it is
    /// set to 0. If the sprite is positioned so part of it is outside the
    /// coordinates of the display, it wraps around to the opposite side of the
    /// screen.
    fn drw(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Ex9E - SKP Vx
    /// 
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    fn skp(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// ExA1 - SKNP Vx
    /// 
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// 
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    fn sknp(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Fx07 - LD Vx, DT
    /// 
    /// Set Vx = delay timer value.
    /// 
    /// The value of DT is placed into Vx.
    fn ld_reg_dt(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Fx0A - LD Vx, K
    /// 
    /// Wait for a key press, store the value of the key in Vx.
    /// 
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn ld_wait(self: &mut Self, instruction: u16) -> () {
        todo!()
    }
    /// Fx15 - LD DT, Vx
    /// 
    /// Set delay timer = Vx.
    /// 
    /// DT is set equal to the value of Vx.
    fn ld_dt_reg(self: &mut Self, instruction: u16) -> () {
        todo!()
    }
    
    /// Fx18 - LD ST, Vx
    /// 
    /// Set sound timer = Vx.
    /// 
    /// ST is set equal to the value of Vx.
    fn ld_st(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Fx1E - ADD I, Vx
    /// 
    /// Set I = I + Vx.
    /// 
    /// The values of I and Vx are added, and the results are stored in I.
    fn add_i(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// 
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx.
    fn ld_loc(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Fx33 - LD B, Vx
    /// 
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// 
    /// The interpreter takes the decimal value of Vx, and places the hundreds
    /// digit in memory at location in I, the tens digit at location I+1, and
    /// the ones digit at location I+2.
    fn ld_bcd(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Fx55 - LD [I], Vx
    /// 
    /// Store registers V0 through Vx in memory starting at location I.
    /// 
    /// The interpreter copies the values of registers V0 through Vx into
    /// memory, starting at the address in I.
    fn store_regs(self: &mut Self, instruction: u16) -> () {
        todo!()
    }

    /// Fx65 - LD Vx, [I]
    /// 
    /// Read registers V0 through Vx from memory starting at location I.
    /// 
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn read_regs(self: &mut Self, instruction: u16) -> () {
        todo!()
    }
}
