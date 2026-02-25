use std::ops::Add;

use crate::bus::Bus;

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
    bus: Bus,
}

#[derive(PartialEq)]
pub enum AddressMode {
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Accumulator,
    Indirect,
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        let a = 0;
        let x = 0;
        let y = 0;
        // let sp = 0;
        let pc = 0xC000;
        let p = 0;

        Self {
            a,
            pc,
            p,
            x,
            y,
            bus,
        }
    }

    pub fn tick(&mut self) {
        println!("tick");
        let opcode = self.bus.read(self.pc);
        self.pc += 1;

        match opcode {
            // ADC
            0x69 => self.adc(AddressMode::Immediate),
            0x6D => self.adc(AddressMode::Absolute),
            0x7D => self.adc(AddressMode::AbsoluteX),
            0x79 => self.adc(AddressMode::AbsoluteY),
            0x65 => self.adc(AddressMode::ZeroPage),
            0x75 => self.adc(AddressMode::ZeroPageX),
            0x61 => self.adc(AddressMode::IndirectX),
            0x71 => self.adc(AddressMode::IndirectY),
            // ROR
            0x76 => self.ror(AddressMode::Immediate),
            0x6A => self.ror(AddressMode::Accumulator),
            0x66 => self.ror(AddressMode::ZeroPage),
            0x6E => self.ror(AddressMode::Absolute),
            0x7E => self.ror(AddressMode::AbsoluteX),
            // JMP
            0x4C => self.jmp(AddressMode::Absolute),
            _ => panic!("no matching opcode. Got {:02X}", opcode),
        }
    }

    pub fn jmp(&mut self, addr_mode: AddressMode) {
        match addr_mode {
            AddressMode::Indirect => {
               self.pc = self.indirect()

            }
            AddressMode::Accumulator => {
                
            }
            AddressMode::Immediate => {
                
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                self.pc = addr;
                // value = self.bus.read(addr)
            }
            AddressMode::ZeroPage => {
 

            }
            AddressMode::ZeroPageX => {

            }
            AddressMode::AbsoluteX => {

            }
            AddressMode::AbsoluteY => {

            }
            AddressMode::IndirectX => {

            }
            AddressMode::IndirectY => {

            }
        }

    }

    pub fn ror(&mut self, addr_mode: AddressMode) {
        let mut value: u8;
        let mut address: Option<u16> = None;

        match addr_mode {
            AddressMode::Accumulator => {
                value = self.a;
            }
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                address = Some(addr);
                value = self.bus.read(addr)
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                address = Some(addr as u16);
                value = self.bus.read(addr as u16)
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                address = Some(addr as u16);
                value = self.bus.read(addr as u16)
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                address = Some(addr);
                value = self.bus.read(addr as u16)
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                address = Some(addr);
                value = self.bus.read(addr as u16)
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                address = Some(addr);
                value = self.bus.read(addr)
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y();
                address = Some(addr);
                value = self.bus.read(addr)
            }
        }

        let lsb = value & 0x01;
        let carry = self.p & 0x01;

        value = value >> 1;
        if carry == 0x01 {
            value = value | 0x80;
        }

        // clears the lsb on status and or's it with new lsb
        self.p = self.p & 0xFE;
        self.p = self.p | lsb;

        if addr_mode == AddressMode::Accumulator {
            self.a = value;
        } else {
            if let Some(addr) = address {
                self.bus.write(addr, value);
            }
        }
    }

    pub fn adc(&mut self, addr_mode: AddressMode) {
        let value: u8;

        match addr_mode {
            AddressMode::Accumulator => {
                value = self.a;
            }
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr)
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16)
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.bus.read(addr as u16)
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.bus.read(addr as u16)
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.bus.read(addr as u16)
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                value = self.bus.read(addr)
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y();
                value = self.bus.read(addr)
            }
        }

        self.a = self.a + value + (self.p & 0x01);
    }

    pub fn indirect(&mut self) -> u16 {
        self.pc += 1;
        let low = self.bus.read(self.pc) as u16;
        if low == 0xFF{
            panic!("you need to impmenent the jmp bug")
        }
        self.pc += 1;
        let high = (self.bus.read(self.pc) as u16) << 8;
        high | low

    }

    pub fn indirect_x(&mut self) -> u16 {
        let arg = self.bus.read(self.pc);
        let addr1 = arg.wrapping_add(self.x);
        let addr2 = arg.wrapping_add(self.x + 1);
        let val1 = self.bus.read(addr1 as u16) as u16;
        let val2 = ((self.bus.read(addr2 as u16)) as u16) << 8;
        let combined_addr = val1 | val2;
        combined_addr
    }

    pub fn indirect_y(&mut self) -> u16 {
        let arg = self.bus.read(self.pc);
        let addr1 = self.bus.read(arg as u16) as u16;
        let addr2_idx = arg + 1;
        let addr2 = (self.bus.read(addr2_idx as u16) as u16) << 8;
        let combined_addr = (addr1 | addr2) + self.y as u16;
        combined_addr
    }

    pub fn am_immediate(&mut self) -> u8 {
        self.bus.read(self.pc)
    }

    pub fn zero_page(&mut self) -> u8 {
        let addr = self.bus.read(self.pc);
        addr
    }
    pub fn zero_page_x(&mut self) -> u8 {
        let arg = self.bus.read(self.pc);
        // using wrapping add instead of % 256
        let addr = arg.wrapping_add(self.x);
        addr
    }

    pub fn absolute_y(&mut self) -> u16 {
        let first_byte: u16;
        let second_byte: u16;
        first_byte = (self.bus.read(self.pc) as u16) << 8;
        self.pc += 1;
        second_byte = self.bus.read(self.pc) as u16;
        let arg = first_byte | second_byte;
        let addr = arg + self.y as u16;
        addr
    }

    pub fn absolute_x(&mut self) -> u16 {
        let first_byte: u16;
        let second_byte: u16;
        first_byte = (self.bus.read(self.pc) as u16) << 8;
        self.pc += 1;
        second_byte = self.bus.read(self.pc) as u16;
        let arg = first_byte | second_byte;
        let addr = arg + self.x as u16;
        addr
    }

    pub fn am_absolute(&mut self) -> u16 {
        let first_byte: u16;
        let second_byte: u16;
        first_byte = (self.bus.read(self.pc) as u16) << 8;
        self.pc += 1;
        second_byte = self.bus.read(self.pc) as u16;
        first_byte | second_byte
    }

    // pub fn is_end_of_program(&self) -> bool {
    //     if self.pc >= (self.ram.len() - 1) as u16 {
    //         true
    //     } else {
    //         false
    //     }
    // }
}
