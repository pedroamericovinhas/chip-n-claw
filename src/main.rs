use std::env;
use std::fs;
mod architecture;
use architecture::Architecture;
// Dev branch??

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom = init_rom(args[1].as_str());
    let mut arch = Architecture::new();
    
    loop {
        // TODO: 60hz loop                    
        arch.execute(rom.clone());
    }
}

fn init_rom(file_path: &str) -> Vec<u16> {
    let rom = fs::read(file_path).unwrap();
    rom.chunks_exact(2)
       .map(|chunk| u16::from_le_bytes([chunk[1], chunk[0]]))
       .collect()
}
