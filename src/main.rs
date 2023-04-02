use std::env;
use std::fs;
use std::process;

const WIDTH:  usize = 64;
const HEIGHT: usize = 32;
const STACK_SIZE: usize = 16;
fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = init_rom(args[1].as_str());
    let mut ram: [u8; 0x1000] = [0; 0x1000];
    let v: [u8;16] = [0;16];
    let mut arch = Architecture::new();
    arch.display = [1; WIDTH*HEIGHT];
    let stack = Stack::new();
    loop {
        // TODO: 60hz loop
        dbg!(rom[arch.pc as usize], v, arch.display[1]);
        execute(rom[arch.pc as usize], &mut arch, stack, v);
    }
    
}

fn init_rom(file_path: &str) -> Vec<u16> {
    let rom = fs::read(file_path).unwrap();
    rom.chunks_exact(2)
       .map(|chunk| u16::from_le_bytes([chunk[1], chunk[0]]))
       .collect()
}

fn execute(instruction: u16, arch: &mut Architecture, stack: Stack, v: [u8; 16]) -> () {
    match instruction {
        0x00E0 => OpCodes::cls(&mut arch.display),
        0x1000..=0x1FFF => {
            OpCodes::jp(arch, instruction % 0x1000)
        },
        0x2000..=0x2FFF => {
            OpCodes::call(arch, stack, instruction % 0x1000)
        },
        0x6000..=0x6FFF => {
            let x: u8 = ((instruction & 0x0F00) >> 8).try_into().unwrap(); // 2 * 4 bits
            let kk: u8 = (instruction & 0x00FF).try_into().unwrap();
             OpCodes::load_byte(v, arch, x, kk);
        },
        0xDEAD => OpCodes::exit(),
        _ => panic!("OpCode does not exist!")
    }
    arch.pc += 1;
}
#[derive(Debug, Clone, Copy)]
pub struct Architecture {
    
    v: [u8; 16],
    display: [u8; WIDTH*HEIGHT],
    i:  u16,
    pc: u16,
    dt: u8,
    st: u8,
}
impl Architecture {
    fn new()->Self {
        Self { i: 0, pc: 0, dt: 0, st: 0, display: [0; WIDTH*HEIGHT], v: [0;16]}
    }
}

pub struct OpCodes;

impl OpCodes {
    fn sys(nnn:u16) -> () {
        /*    0nnn
         *   
         *    Jump to a machine code routine at nnn.
         *   
         *    This instruction is only used on the old computers on which
         *    Chip-8 was originally implemented.
         *    It is ignored by modern interpreters.
         */
        unimplemented!();
    }
    fn cls(display: &mut [u8; 64*32]) -> () {
        /*    00E0
         *   
         *    Clear the display.
         */ 
        *display = [0u8; 64*32];
    }
    fn ret(registers: &mut Architecture, stack:&mut Stack) -> () {
        /*    00EE
         *   
         *    Return from a subroutine.
         * 
         *    The interpreter sets the program counter to the address
         *    at the top of the stack, then subtracts 1 from the stack pointer.
         */
        
        registers.pc = todo!();
    }
    fn jp(registers: &mut Architecture, nnn:u16) -> () {
        /*    1nnn
         *
         *    Jump to location nnn.
         *
         *    The interpreter sets the program counter to nnn.
         */
        registers.pc = nnn;
    }
    fn call(registers: &mut Architecture, mut stack:Stack, nnn:u16) -> () {
        /*    2nnn
         *
         *    Call subroutine at nnn.
         *
         *    The interpreter increments the stack pointer,
         *    then puts the current PC on the top of the stack.
         *    The PC is then set to nnn.
         */
        stack.sp += 1;
        stack.push(registers.pc);
        registers.pc = nnn;
    }
    fn s_e_byte(v: [u8; 16], registers: &mut Architecture, x: u8, kk:u8) -> () {
        /*   3xkk
        *
        *    Skip next instruction if Vx == kk.
        *
        *    The interpreter compares register Vx to kk,
        *    and if they are equal, increments the program counter by 2.
        */
        if v[x as usize] == kk{
            registers.pc += 2;
        }
    }
    fn s_n_e_byte(v: [u8; 16], registers: &mut Architecture, x: u8, kk:u8) -> () {
        /*   4xkk
        *
        *    Skip next instruction if Vx != kk.
        *
        *    The interpreter compares register Vx to kk,
        *    and if they are not equal, increments the program counter by 2.
        */
        if v[x as usize] != kk{
            registers.pc += 2;
        }
    }
    fn s_e_register(v: [u8; 16], registers: &mut Architecture, x: u8, y:u8) -> () {
        /*   5xy0
        *
        *    Skip next instruction if Vx == Vy.
        *
        *    The interpreter compares register Vx to register Vy,
        *    and if they are equal, increments the program counter by 2.
        */
        if v[x as usize] == v[y as usize]{
            registers.pc += 2;
        }
    }
    fn load_byte(mut v: [u8; 16], registers: &mut Architecture, x: u8, kk:u8) -> () {
        /*   6xkk
         *   
         *   Set Vx = kk.
         * 
         *   The interpreter puts the value kk into register Vx.
         */
        v[x as usize] = kk;
    }
    fn add_byte(mut v: [u8; 16], registers: &mut Architecture, x: u8, kk:u8) -> () {
        /*   7xkk
         *   
         *   Set Vx = Vx + kk.
         * 
         *   Adds the value kk to the value of register Vx,
         *   then stores the result in Vx. 
         */
        v[x as usize] += kk;
    }
    fn ld(mut v: [u8; 16], x: u8, y:u8) -> () {
        /*   8xy0
         *   
         *   Set Vx = Vy.
         * 
         *   Stores the value of register Vy in register Vx.
         */
        v[x as usize] = v[y as usize];
    }
    fn exit() -> () {
        process::exit(0);
    }


}

#[derive(Debug, Clone, Copy)]
struct Stack {
    memory: [u16; STACK_SIZE],
    sp: usize,
}
impl Stack {
    fn new() -> Self {
        Stack {
            memory: [0; STACK_SIZE],
            sp: 0,
        }
    }

    fn push(&mut self, value: u16) {
        if self.sp < STACK_SIZE {
            self.memory[self.sp] = value;
            self.sp += 1;
        } else {
            dbg!(self);
            panic!("Stack overflow!")
        }
    }

    fn pop(&mut self) -> Option<u16> {
        if self.sp > 0 {
            self.sp -= 1;
            let val = self.memory[self.sp]; 
            self.memory[self.sp] = 0;
            Some(val)
        } else {
            None
        }
    }


}
