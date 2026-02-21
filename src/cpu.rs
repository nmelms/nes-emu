pub struct CPU {
    // registers
    // accumulator
    pub a: u8,
    // indexes
    x: u8,
    y: u8,
    // // stack pointer
    // sp: u8,
    // status
    p: u8,
    // program countergi
    pc: u16,
    ram: Vec<u8>,
}

pub enum AddressMode {
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> Self {
        let ram = rom;
        let a = 0;
        let x = 0;
        let y = 0;
        // let sp = 0;
        let pc = 0;
        let p = 0;

        Self { ram, a, pc, p, x, y }
    }

    pub fn tick(&mut self) {
        println!("tick");
        let opcode = self.ram[self.pc as usize];
        if !self.is_end_of_program() {
            self.pc += 1;
        }

        match opcode {
            0x69 => self.adc(AddressMode::Immediate),
            0x6D => self.adc(AddressMode::Absolute),
            0x7D => self.adc(AddressMode::AbsoluteX),
            0x79 => self.adc(AddressMode::AbsoluteY),
            0x65 => self.adc(AddressMode::ZeroPage),
            0x75 => self.adc(AddressMode::ZeroPageX),
            0x61 => self.adc(AddressMode::IndirectX),
            0x71 => self.adc(AddressMode::IndirectY),
            _ => panic!("no matching opcode. Got {}", opcode),
        }
    }

    pub fn adc(&mut self, addr_mode: AddressMode) {
        let value: u8;

        match addr_mode {
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.ram[addr as usize];
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.ram[addr as usize];
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.ram[addr as usize];
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.ram[addr as usize];
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.ram[addr as usize];
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                value = self.ram[addr as usize];
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y();
                value = self.ram[addr as usize];
            }
        }

        self.a = self.a + value + (self.p & 0x01);
    }

    pub fn indirect_x(&mut self) -> u16{
        let arg = self.ram[self.pc as usize];
        let addr1 = arg.wrapping_add(self.x);
        let addr2 = arg.wrapping_add(self.x + 1);
        let val1 = self.ram[addr1 as usize] as u16;
        let val2 = (self.ram[addr2 as usize] as u16) << 8;
        let combined_addr = val1 | val2;
        combined_addr
    }

    pub fn indirect_y(&mut self) -> u16{
        let arg = self.ram[self.pc as usize];
        let addr1 = self.ram[arg as usize] as u16;
        let addr2_idx = arg + 1;
        let addr2 = (self.ram[addr2_idx as usize] as u16) << 8 ;
        let combined_addr = (addr1 | addr2) + self.y as u16;
        combined_addr
    }

    pub fn am_immediate(&mut self) -> u8 {
        self.ram[self.pc as usize]
    }

    pub fn zero_page(&mut self) -> u8 {
        let addr = self.ram[self.pc as usize];
        addr
    }
    pub fn zero_page_x(&mut self) -> u8 {
        let arg = self.ram[self.pc as usize];
        // using wrapping add instead of % 256
        let addr = arg.wrapping_add(self.x);
        addr
    }

    pub fn absolute_y(&mut self) -> u16 {
        let first_byte: u16;
        let second_byte: u16;
        first_byte = (self.ram[self.pc as usize] as u16) << 8;
        self.pc += 1;
        second_byte = self.ram[self.pc as usize] as u16;
        let arg = first_byte | second_byte;
        let addr = arg + self.y as u16;
        addr
    }

    pub fn absolute_x(&mut self) -> u16 {
        let first_byte: u16;
        let second_byte: u16;
        first_byte = (self.ram[self.pc as usize] as u16) << 8;
        self.pc += 1;
        second_byte = self.ram[self.pc as usize] as u16;
        let arg = first_byte | second_byte;
        let addr = arg + self.x as u16;
        addr
    }

    pub fn am_absolute(&mut self) -> u16 {
        let first_byte: u16;
        let second_byte: u16;
        first_byte = (self.ram[self.pc as usize] as u16) << 8;
        self.pc += 1;
        second_byte = self.ram[self.pc as usize] as u16;
        first_byte | second_byte
    }

    pub fn is_end_of_program(&self) -> bool {
        if self.pc >= (self.ram.len() - 1) as u16 {
            true
        } else {
            false
        }
    }
}
