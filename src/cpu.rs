

pub struct CPU {
    // registers
    // accumulator
    a: u8,
    // indexes
    x: u8,
    y: u8,
    // stack pointer
    sp: u8,
    // status
    p: u8,
    // program countergi
    pc: u16,
    RAM: Vec<u16>,
    running: bool,
    
}

impl CPU {

    pub fn main(){
        println!("this thing is running")
    }
    
    pub fn am_immediate(&mut self) -> u16 {
        self.pc += 1;
        let addr = self.pc as usize;
        self.RAM[addr]
        
    }
}