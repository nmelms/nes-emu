use std::ops::Add;



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
    ROM: Vec<u8>,
    
}

enum AddressMode{
    Immediate,
    // Absolute,
}



impl CPU {

    pub fn new() -> Self{
        
        let ROM = vec![0x69, 0xA9u8, 0x05, 0x69, 0x03];
        let a = 0;
        let x = 0;
        let y = 0;
        let sp = 0;
        let pc = 0;
        let p = 0;

        Self{ROM, a, x, y, sp, pc, p}
    }
    pub fn tick(&mut self, opcode: u8){

        match opcode{
            0x69 => self.adc(AddressMode::Immediate),
            _ => panic!("no matching opcode")
        }
    }

    pub fn adc(&mut self, addr_mode: AddressMode) {
        self.pc += 1;
        let value:u16;

        match addr_mode{
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
        }

        // self.a = self.a + value + c;


    }
    
    pub fn am_immediate(&mut self) -> u8 {
        self.pc += 1;
        let addr = self.pc as usize;
        
    }
}