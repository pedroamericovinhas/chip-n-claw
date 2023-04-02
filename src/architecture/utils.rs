pub struct Hex;

impl Hex {
    pub fn swap_hex_digits(num: u16, i: u16, j: u16) -> u16 {
        let temp: u16 = (num >> (4 * i)) ^ (num >> (4 * j)) & 0xF;
        num ^ (temp << (4 * i)) ^ (temp << (4 * j))
    } 
}