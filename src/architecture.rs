mod stack;
mod utils;
use stack::Stack;
use utils::Hex;
use std::process;

const WIDTH:  usize = 64;
const HEIGHT: usize = 32;
const RAM_SIZE: usize = 0x1000;

#[derive(Debug, Clone, Copy)]
pub struct Architecture {
    ram: [u8; RAM_SIZE],
    stack: Stack,
    display: [u8; WIDTH*HEIGHT],
    v: [u8; 16],
    i:  u16, pc: u16, dt: u8, st: u8,
}
impl Architecture {
  pub fn new()->Self {
      Self { 
          ram: [0; RAM_SIZE],
          stack: Stack::new(),
          display: [0; WIDTH*HEIGHT],
          v: [0;16],
          i: 0, pc: 0, dt: 0, st: 0,}
  }
}
impl Architecture {
  pub fn execute(self: &mut Self, rom: Vec<u16>) -> () {
      let instruction = rom[self.pc as usize];
      match instruction {
          0x00E0 => self.cls(),
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
                      _=> panic!("OpCode does not exist!")
          }
          0xDEAD => Architecture::exit(),
          _ => panic!("OpCode does not exist!")
      }
      self.pc += 1;
  }
}
impl Architecture {
  fn cls(self: &mut Self) -> () {
      /*    00E0
       *   
       *    Clear the display.
       */ 
      self.display = [0u8; 64*32];
  }
  fn ret(self: Self) -> () {
      /*    00EE
       *   
       *    Return from a subroutine.
       * 
       *    The interpreter sets the program counter to the address
       *    at the top of the stack, then subtracts 1 from the stack pointer.
       */
      todo!();
  }
  fn jp(self: &mut Self, instruction:u16) -> () {
      /*    1nnn
       *
       *    Jump to location nnn.
       *
       *    The interpreter sets the program counter to nnn.
       */
      self.pc = instruction & 0xFFF;
  }
  fn call(self: &mut Self, instruction: u16) -> () {
      /*    2nnn
       *
       *    Call subroutine at nnn.
       *
       *    The interpreter increments the stack pointer,
       *    then puts the current PC on the top of the stack.
       *    The PC is then set to nnn.
       */
      self.stack.sp += 1;
      self.stack.push(self.pc);
      self.pc = instruction & 0xFFF;
  }
  fn s_e_byte(self: &mut Self, instruction:u16) -> () {
      /*   3xkk
      *
      *    Skip next instruction if Vx == kk.
      *
      *    The interpreter compares register Vx to kk,
      *    and if they are equal, increments the program counter by 2.
      */
      let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
      let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
      if self.v[x] == kk{
          self.pc += 2;
      }
  }
  fn s_n_e_byte(self: &mut Self, instruction:u16) -> () {
      /*   4xkk
      *
      *    Skip next instruction if Vx != kk.
      *
      *    The interpreter compares register Vx to kk,
      *    and if they are not equal, increments the program counter by 2.
      */
      let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
      let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
      if self.v[x] != kk{
          self.pc += 2;
      }
  }
  fn s_e_register(self: &mut Self, instruction:u16) -> () {
      /*   5xy0
      *
      *    Skip next instruction if Vx == Vy.
      *
      *    The interpreter compares register Vx to register Vy,
      *    and if they are equal, increments the program counter by 2.
      */
      if (instruction & 0xF) != 0x0 {panic!("OpCode does not exist!")};
      let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
      let y: usize = ((instruction & 0x00F0) >> 1*4).try_into().unwrap();
      if self.v[x] == self.v[y]{
          self.pc += 2;
      }
  }
  fn load_byte(self: &mut Self, instruction: u16) -> () {
      /*   6xkk
       *   
       *   Set Vx = kk.
       * 
       *   The interpreter puts the value kk into register Vx.
       */
      let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
      let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
      self.v[x] = kk;
  }
  fn add_byte(self: &mut Self, instruction: u16) -> () {
      /*   7xkk
       *   
       *   Set Vx = Vx + kk.
       * 
       *   Adds the value kk to the value of register Vx,
       *   then stores the result in Vx. 
       */
      let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
      let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
      self.v[x] += kk;
  }
  fn ld(self: &mut Self, instruction: u16) -> () {
      /*   8xy0
       *   
       *   Set Vx = Vy.
       * 
       *   Stores the value of register Vy in register Vx.
       */
      let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
      let y: usize = ((instruction & 0x00F0) >> 1*4).try_into().unwrap();
      self.v[x] = self.v[y];
  }
  fn or(self: &mut Self, instruction: u16) -> () {
    /* 8xy1
     * 
     * Set Vx = Vx OR Vy.
     * 
     * Performs a bitwise OR on the values of Vx and Vy, then stores the result
     * in Vx. A bitwise OR compares the corrseponding bits from two values, and
     * if either bit is 1, then the same bit in the result is also 1. Otherwise,
     * it is 0. 
     */
    let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
    let y: usize = ((instruction & 0x00F0) >> 1*4).try_into().unwrap();
    self.v[x] = self.v[x] | self.v[y];
  }
  fn and(self: &mut Self, instruction: u16) -> () {
    /* 8xy2
     * 
     * Set Vx = Vx AND Vy.
     * 
     * Performs a bitwise AND on the values of Vx and Vy, then stores the result
     * in Vx. A bitwise AND compares the corrseponding bits from two values, and
     * if if both bits are 1, then the same bit in the result is also 1.
     * Otherwise, it is 0.
     */
    let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
    let y: usize = ((instruction & 0x00F0) >> 1*4).try_into().unwrap();
    self.v[x] = self.v[x] & self.v[y];
  }
  fn xor(self: &mut Self, instruction: u16) -> () {
    /* 8xy3
     * 
     * Set Vx = Vx XOR Vy.
     * 
     * Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
     * the result in Vx. An exclusive OR compares the corrseponding bits from
     * two values, and if the bits are not both the same, then the corresponding
     * bit in the result is set to 1. Otherwise, it is 0. 
     */
    let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
    let y: usize = ((instruction & 0x00F0) >> 1*4).try_into().unwrap();
    self.v[x] = self.v[x] ^ self.v[y];
  }
  fn add(self: &mut Self, instruction: u16) -> () {
    /* 8xy4
     * 
     * Set Vx = Vx + Vy, set VF = carry.
     * 
     * The values of Vx and Vy are added together. If the result is greater than
     * 8 bits (i.e., > 255,) VF is set to 1,
     * otherwise 0. Only the lowest 8 bits of the result are kept,
     * and stored in Vx.
     */
    let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
    let y: usize = ((instruction & 0x00F0) >> 1*4).try_into().unwrap();
    let sum: u16 = self.v[x] as u16 + self.v[y] as u16;
    if sum > 0x0FF {
      let sum: u8 = (sum >> 1*4).try_into().unwrap();
      self.v[x] = sum;
      self.v[0xF] = 1;
    } else {
      let sum:u8 = sum.try_into().unwrap();
      self.v[x] = sum;
      self.v[0xF] = 1;
    }
  }
  fn sub(self: &mut Self, instruction: u16) -> () {
    /* 8xy5
     * 
     * Set Vx = Vx - Vy, set VF = NOT borrow.
     * 
     * If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from
     * Vx, and the results stored in Vx.
     */
    let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
    let y: usize = ((instruction & 0x00F0) >> 1*4).try_into().unwrap();
    self.v[0xF] = if self.v[x] > self.v[y] {1} else {0};
    let subs: u8 = self.v[x]- self.v[y];
    self.v[x] = subs;
  }
  fn shr(self: &mut Self, instruction: u16) -> () {
    /* 8xy6
     * 
     * Set Vx = Vx SHR 1.
     * 
     * If the least-significant bit of Vx is 1, then VF is set to 1,
     * otherwise 0. Then Vx is divided by 2.
     */
    let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
    self.v[0xF] = self.v[x] & 0x1;
    self.v[x] >>= 1;
  }
  fn subn(self: &mut Self, instruction: u16) -> () {
    /* 8xy7
     * 
     * Set Vx = Vy - Vx, set VF = NOT borrow.
     * 
     * If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from
     * Vy, and the results stored in Vx.
     */
    self.sub(Hex::swap_hex_digits(instruction, 1, 2));
  }
  fn shl(self: &mut Self, instruction: u16) -> () {
    /* 8xy6
     * 
     * Set Vx = Vx SHL 1.
     * 
     * If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to
     * 0. Then Vx is multiplied by 2.
     */
    let x: usize = ((instruction & 0x0F00) >> 2*4).try_into().unwrap();
    self.v[0xF] = self.v[x] >> 7;
    self.v[x] <<= 1;
  }
  fn exit() -> () {
      process::exit(0);
  }
}