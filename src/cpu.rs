
pub struct CPU {
    // registers
    // accumulator
    a: u8,
    // indexes
    // x: u8,
    // y: u8,
    // // stack pointer
    // sp: u8,
    // status
    p: u8,
    // program countergi
    pc: u16,
    rom: Vec<u8>,
    
}

pub enum AddressMode{
    Immediate,
    // Absolute,
}



impl CPU {

    pub fn new() -> Self{
        
        let rom = vec![0x69u8, 0x05];
        let a = 0;
        // let x = 0;
        // let y = 0;
        // let sp = 0;
        let pc = 0;
        let p = 0;

        Self{rom, a, pc, p}
    }

    pub fn tick(&mut self,){
        println!("tick");
        let opcode = self.rom[self.pc as usize];
        if !self.is_end_of_program(){
            self.pc += 1;
        }


        match opcode{
            0x69 => {
                self.adc(AddressMode::Immediate)},
            _ => panic!("no matching opcode")
        }
    }

    pub fn adc(&mut self, addr_mode: AddressMode) {
        let value:u8;

        match addr_mode{
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
        }

        self.a = self.a + value + (self.p & 0x01);
    }
    
    pub fn am_immediate(&mut self) -> u8 {
        self.rom[self.pc as usize]
    }

    pub fn is_end_of_program(&self) -> bool{
        if self.pc >= (self.rom.len() -1)  as u16{
            true
        }else{
            false
        }
    }


}