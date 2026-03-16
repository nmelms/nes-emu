use std::{mem, ops::Add};

use crate::bus::Bus;

pub struct CPU {
    // registers
    // accumulator
    pub a: u8,
    // indexes
    x: u8,
    y: u8,
    // // stack pointer
    sp: u8,
    // status
    p: u8,
    // program countergi
    pc: u16,
    bus: Bus,
    line: u16,
}

#[derive(PartialEq)]
pub enum AddressMode {
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Accumulator,
    Indirect,
    Relative,
}

impl CPU {
    pub fn new(mut bus: Bus) -> Self {
        let a = 0;
        let x = 0;
        let y = 0;
        let sp = 0xFD;
        // let low = bus.read(0xFFFC) as u16;
        // let high = bus.read(0xFFFD) as u16;
        // high << 8 | low;

        let pc = 0xC000;
        let p = 0x24;
        let line = 0;

        Self {
            a,
            pc,
            p,
            x,
            y,
            bus,
            sp,
            line,
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.bus.read(self.pc);
        self.line += 1;
        println!(
            "{} {:04X}  {:02X}  A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.line, self.pc, opcode, self.a, self.x, self.y, self.p, self.sp
        );
        // println!("stack pointer: {:02X}", self.bus.read(self.sp as u16 + 0x0100));
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
            // Rotate Right
            0x6A => self.ror(AddressMode::Accumulator),
            0x66 => self.ror(AddressMode::ZeroPage),
            0x76 => self.ror(AddressMode::ZeroPageX),
            0x6E => self.ror(AddressMode::Absolute),
            0x7E => self.ror(AddressMode::AbsoluteX),
            // JMP
            0x4C => self.jmp(AddressMode::Absolute),
            0x6C => self.jmp(AddressMode::Indirect),
            // LDX
            0xA2 => self.ldx(AddressMode::Immediate),
            0xA6 => self.ldx(AddressMode::ZeroPage),
            0xB6 => self.ldx(AddressMode::ZeroPageY),
            0xAE => self.ldx(AddressMode::Absolute),
            0xBE => self.ldx(AddressMode::AbsoluteY),
            // Store x
            0x86 => self.stx(AddressMode::ZeroPage),
            0x96 => self.stx(AddressMode::ZeroPageY),
            0x8E => self.stx(AddressMode::Absolute),
            // Jump to subroutine
            0x20 => self.jsr(AddressMode::Absolute),
            // set Carry
            0x38 => self.sec(),
            // Branch if Carry Set
            0xB0 => self.bcs(AddressMode::Relative),
            // Clear Carry
            0x18 => self.clc(),
            // Branch if Carry Clear
            0x90 => self.bcc(AddressMode::Relative),
            // Branch if Equal
            0xF0 => self.beq(),
            // branch if not equal
            0xD0 => self.bne(),
            // Store A
            0x85 => self.sta(AddressMode::ZeroPage),
            0x95 => self.sta(AddressMode::ZeroPageX),
            0x8D => self.sta(AddressMode::Absolute),
            0x9D => self.sta(AddressMode::AbsoluteX),
            0x99 => self.sta(AddressMode::AbsoluteY),
            0x81 => self.sta(AddressMode::IndirectX),
            0x91 => self.sta(AddressMode::IndirectY),
            // Bit Test
            0x24 => self.bit(AddressMode::ZeroPage),
            0x2C => self.bit(AddressMode::Absolute),
            // Branch if Overflow Set
            0x70 => self.bvs(),
            // Branch if overflow clear
            0x50 => self.bvc(),
            // branch if Plus
            0x10 => self.bpl(),
            // return to subroutine
            0x60 => self.rts(),
            // Set interrupt Disable
            0x78 => self.sei(),
            // Set Decimal
            0xF8 => self.sed(),
            // Push Processor Status
            0x08 => self.php(),
            // Pull A
            0x68 => self.pla(),
            // Bitwise And
            0x29 => self.and(AddressMode::Immediate),
            0x25 => self.and(AddressMode::ZeroPage),
            0x35 => self.and(AddressMode::ZeroPageX),
            0x2D => self.and(AddressMode::Absolute),
            0x3D => self.and(AddressMode::AbsoluteX),
            0x39 => self.and(AddressMode::AbsoluteY),
            0x21 => self.and(AddressMode::IndirectX),
            0x31 => self.and(AddressMode::IndirectY),
            // Compare A
            0xC9 => self.cmp(AddressMode::Immediate),
            0xC5 => self.cmp(AddressMode::ZeroPage),
            0xD5 => self.cmp(AddressMode::ZeroPageX),
            0xCD => self.cmp(AddressMode::Absolute),
            0xDD => self.cmp(AddressMode::AbsoluteX),
            0xD9 => self.cmp(AddressMode::AbsoluteY),
            0xC1 => self.cmp(AddressMode::IndirectX),
            0xD1 => self.cmp(AddressMode::IndirectY),
            // Clear Dcimal
            0xD8 => self.cld(),
            // Push A
            0x48 => self.pha(),
            // Pull processor status
            0x28 => self.plp(),
            // Branch if minus
            0x30 => self.bmi(),
            // Bitwise  OR
            0x09 => self.or(AddressMode::Immediate),
            0x05 => self.or(AddressMode::ZeroPage),
            0x15 => self.or(AddressMode::ZeroPageX),
            0x0D => self.or(AddressMode::Absolute),
            0x1D => self.or(AddressMode::AbsoluteX),
            0x19 => self.or(AddressMode::AbsoluteY),
            0x01 => self.or(AddressMode::IndirectX),
            0x11 => self.or(AddressMode::IndirectY),
            // clear overflow
            0xB8 => self.clv(),
            // Bitwise Exclusive OR
            0x49 => self.eor(AddressMode::Immediate),
            0x45 => self.eor(AddressMode::ZeroPage),
            0x55 => self.eor(AddressMode::ZeroPageX),
            0x4D => self.eor(AddressMode::Absolute),
            0x5D => self.eor(AddressMode::AbsoluteX),
            0x59 => self.eor(AddressMode::AbsoluteY),
            0x41 => self.eor(AddressMode::IndirectX),
            0x51 => self.eor(AddressMode::IndirectY),
            // Load Y
            0xA0 => self.ldy(AddressMode::Immediate),
            0xA4 => self.ldy(AddressMode::ZeroPage),
            0xB4 => self.ldy(AddressMode::ZeroPageX),
            0xAC => self.ldy(AddressMode::Absolute),
            0xBC => self.ldy(AddressMode::AbsoluteX),
            // Compare Y
            0xC0 => self.cpy(AddressMode::Immediate),
            0xC4 => self.cpy(AddressMode::ZeroPage),
            0xCC => self.cpy(AddressMode::Absolute),
            // Compare X
            0xE0 => self.cpx(AddressMode::Immediate),
            0xE4 => self.cpx(AddressMode::ZeroPage),
            0xEC => self.cpx(AddressMode::Absolute),
            // Subtract with Carry
            0xE9 => self.sbc(AddressMode::Immediate),
            0xE5 => self.sbc(AddressMode::ZeroPage),
            0xF5 => self.sbc(AddressMode::ZeroPageX),
            0xED => self.sbc(AddressMode::Absolute),
            0xFD => self.sbc(AddressMode::AbsoluteX),
            0xF9 => self.sbc(AddressMode::AbsoluteY),
            0xE1 => self.sbc(AddressMode::IndirectX),
            0xF1 => self.sbc(AddressMode::IndirectY),
            // LDA
            0xA9 => self.lda(AddressMode::Immediate),
            0xA5 => self.lda(AddressMode::ZeroPage),
            0xB5 => self.lda(AddressMode::ZeroPageX),
            0xAD => self.lda(AddressMode::Absolute),
            0xBD => self.lda(AddressMode::AbsoluteX),
            0xB9 => self.lda(AddressMode::AbsoluteY),
            0xA1 => self.lda(AddressMode::IndirectX),
            0xB1 => self.lda(AddressMode::IndirectY),
            // Increment Y
            0xC8 => self.iny(),
            // Increment X
            0xE8 => self.inx(),
            // Decrement Y
            0x88 => self.dey(),
            // Decrement X
            0xCA => self.dex(),
            // Transfer A to Y
            0xA8 => self.tay(),
            // Transfer A to X
            0xAA => self.tax(),
            // Transfer Y to A
            0x98 => self.tya(),
            // Trasnfer X to A
            0x8A => self.txa(),
            // Tansfer Stack Pointer to X
            0xBA => self.tsx(),
            // Trasnfer X to Stack pointer
            0x9A => self.txs(),
            // Return From Interrupt
            0x40 => self.rti(),
            // Logical Shift Right
            0x4A => self.lsr(AddressMode::Accumulator),
            0x46 => self.lsr(AddressMode::ZeroPage),
            0x56 => self.lsr(AddressMode::ZeroPageX),
            0x4E => self.lsr(AddressMode::Absolute),
            0x5E => self.lsr(AddressMode::AbsoluteX),
            // Arithmetic Shift Left
            0x0A => self.asl(AddressMode::Accumulator),
            0x06 => self.asl(AddressMode::ZeroPage),
            0x16 => self.asl(AddressMode::ZeroPageX),
            0x0E => self.asl(AddressMode::Absolute),
            0x1E => self.asl(AddressMode::AbsoluteX),




            // unoffical noop
            0xEA => self.noop(),
            0xFA => self.noop(),
            0x67 => self.noop(),

            _ => {

                panic!("Opcode not implemented: Got {:02X}", opcode)
            }
        }
    }
    pub fn asl(&mut self,addr_mode: AddressMode){
        let value: u8;
        let mut addr: u16 = 0x00;

        match addr_mode {
            AddressMode::Relative => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::Indirect => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::Accumulator => {
                value = self.a;
            }
            AddressMode::Immediate => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::Absolute => {
                addr = self.am_absolute();
                value = self.bus.read(addr)
            }
            AddressMode::ZeroPage => {
                addr = self.zero_page() as u16;
                value = self.bus.read(addr as u16)
            }
            AddressMode::ZeroPageX => {
                addr = self.zero_page_x() as u16;
                value = self.bus.read(addr as u16)
            }
            AddressMode::ZeroPageY => {
                panic!("lsr addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                addr = self.absolute_x();
                value = self.bus.read(addr as u16)
            }
            AddressMode::AbsoluteY => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::IndirectX => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::IndirectY => {
                panic!("lsr does not use indeirect")
            }
        }



        let carry_flag = value & 0x80;
        let res = value << 1;


        // set carry flag
        if carry_flag == 0x80 {
            self.p = self.p | 0x01;
        }else{
            self.p = self.p & 0xFE;
        }
        // set zero flag
        if res as u8 == 0{
            self.p = self.p | 0x02
        }else{
            self.p = self.p & 0xFD
        }
        // set negative
        if res & 0x80 == 0x80{
            self.p = self.p | 0x80;
        }else{
            self.p = self.p & 0x7F;
        }
        
        
        if addr_mode == AddressMode::Accumulator{
            self.a = res as u8; 
        }else{
            self.bus.write(addr, res)
        }

    }
    pub fn lsr(&mut self, addr_mode: AddressMode){
        let value: u8;
        let mut addr: u16 = 0x00;

        match addr_mode {
            AddressMode::Relative => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::Indirect => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::Accumulator => {
                value = self.a;
            }
            AddressMode::Immediate => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::Absolute => {
                addr = self.am_absolute();
                value = self.bus.read(addr)
            }
            AddressMode::ZeroPage => {
                addr = self.zero_page() as u16;
                value = self.bus.read(addr as u16)
            }
            AddressMode::ZeroPageX => {
                addr = self.zero_page_x() as u16;
                value = self.bus.read(addr as u16)
            }
            AddressMode::ZeroPageY => {
                panic!("lsr addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                addr = self.absolute_x();
                value = self.bus.read(addr as u16)
            }
            AddressMode::AbsoluteY => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::IndirectX => {
                panic!("lsr does not use indeirect")
            }
            AddressMode::IndirectY => {
                panic!("lsr does not use indeirect")
            }
        }



        let carry_flag = value & 0x01;
        let res = value >> 1;


        // set carry flag
        if carry_flag == 0x01 {
            self.p = self.p | 0x01;
        }else{
            self.p = self.p & 0xFE
        }
        // set zero flag
        if res as u8 == 0{
            self.p = self.p | 0x02
        }else{
            self.p = self.p & 0xFD
        }
        // set negative
        self.p = self.p & 0x7F;
        
        if addr_mode == AddressMode::Accumulator{
            self.a = res as u8; 
        }else{
            self.bus.write(addr, res)
        }

    }
    pub fn rti(&mut self){
        self.sp += 1;
        let status_flag = self.bus.read(0x0100 + self.sp as u16);
        self.p = status_flag & 0xEF | 0x20;

        self.sp += 1;
        let low_byte = self.bus.read(0x0100 + self.sp as u16);

        self.sp += 1;
        let high_byte = self.bus.read(0x0100 + self.sp as u16);

        self.pc = (high_byte as u16) << 8 | low_byte as u16;

    }
    pub fn txs(&mut self){
        self.sp = self.x;

    }
    pub fn tsx(&mut self){
        self.x = self.sp;


        // set zero flag
        if self.x == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.x & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }

    }
    pub fn txa(&mut self){
        self.a = self.x;

        // set zero flag
        if self.a == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.a & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }

    }
    pub fn tya(&mut self){
        self.a = self.y;

        // set zero flag
        if self.a == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.a & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }

    }
    pub fn tax(&mut self){
        self.x = self.a;

        // set zero flag
        if self.x == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.x & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }

    }
    pub fn tay(&mut self){
        self.y = self.a;

        // set zero flag
        if self.y == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.y & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }

    }
    pub fn dex(&mut self){
        self.x = self.x.wrapping_sub(1);

        // set zero flag
        if self.x == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.x & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn dey(&mut self){
        self.y = self.y.wrapping_sub(1);

        // set zero flag
        if self.y == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.y & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn inx(&mut self){
        self.x = self.x.wrapping_add(1);

        // set zero flag 
        if self.x == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.x & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn iny(&mut self){
        self.y = self.y.wrapping_add(1);

        // set zero flag
        if self.y == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.y & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn sbc(&mut self, addr_mode: AddressMode) {
        let value: u8;

        match addr_mode {
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("adc does not use indeirect")
            }
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
            AddressMode::ZeroPageY => {
                panic!("adc addrmode not implemented")
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
        let carry_flag = !self.p & 0x01;
        let res = self.a as i32  - value as i32 -  carry_flag as i32;


        // set carry flag
        if !(res < 0x00){
            self.p = self.p | 0x01;
        }else{
            self.p = self.p & 0xFE
        }
        // set zero flag
        if res as u8 == 0{
            self.p = self.p | 0x02
        }else{
            self.p = self.p & 0xFD
        }
        // set overflow
        if (res ^ self.a as i32) & (res ^ !(value as i32)) & 0x80 != 0{
            self.p = self.p | 0x40;
        }else{
            self.p = self.p & 0xBF
        }
        // set negative
        if res & 0x80 == 0x80{
            self.p = self.p | 0x80
        }else{
            self.p = self.p & 0x7F
        }
        self.a = res as u8;
    }
    pub fn cpx(&mut self, addr_mode: AddressMode) {
        let mut value = 0x00;
        match addr_mode {
            AddressMode::Relative => {
                panic!("cpx does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("cpx does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("cpx addrmode not implemented")
            }
            AddressMode::Immediate => {
                let addr = self.am_immediate();
                value = addr;
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                panic!("cpx addrmode not implemented")
            }
            AddressMode::ZeroPageY => {
                panic!("cpx addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                panic!("cpx addrmode not implemented")
            }
            AddressMode::AbsoluteY => {
                panic!("cpx addrmode not implemented")
            }
            AddressMode::IndirectX => {
                panic!("cpx does not use indrect")
            }
            AddressMode::IndirectY => {
                panic!("cpx does not use indrect")
            }
        }

        let res = self.x as i32 - value as i32;

        if self.x >= value {
            self.p = self.p | 0x01;
        } else {
            self.p = self.p & 0xFE;
        }

        // set zero flag
        if self.x == value {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = res as u16 & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn cpy(&mut self, addr_mode: AddressMode) {
        let mut value = 0x00;
        match addr_mode {
            AddressMode::Relative => {
                panic!("ldy does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("ldy does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("ldy addrmode not implemented")
            }
            AddressMode::Immediate => {
                let addr = self.am_immediate();
                value = addr;
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                panic!("ldy addrmode not implemented")
            }
            AddressMode::ZeroPageY => {
                panic!("ldy addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                panic!("ldy addrmode not implemented")
            }
            AddressMode::AbsoluteY => {
                panic!("ldy addrmode not implemented")
            }
            AddressMode::IndirectX => {
                panic!("ldy does not use indrect")
            }
            AddressMode::IndirectY => {
                panic!("ldy does not use indrect")
            }
        }

        let res = self.y as i32 - value as i32;

        if self.y >= value {
            self.p = self.p | 0x01;
        } else {
            self.p = self.p & 0xFE;
        }

        // set zero flag
        if self.y == value {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = res as u16 & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn ldy(&mut self, addr_mode: AddressMode) {
        let mut value = 0x00;
        match addr_mode {
            AddressMode::Relative => {
                panic!("ldy does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("ldy does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("ldy addrmode not implemented")
            }
            AddressMode::Immediate => {
                let addr = self.am_immediate();
                value = addr;
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageY => {
                panic!("ldy addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.bus.read(addr);
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.bus.read(addr);
            }
            AddressMode::IndirectX => {
                panic!("ldy does not use indrect")
            }
            AddressMode::IndirectY => {
                panic!("ldy does not use indrect")
            }
        }

        self.y = value;

        // set zero flag
        if self.y == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.y & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn eor(&mut self, addr_mode: AddressMode) {
        let mut value = 0x00;
        match addr_mode {
            AddressMode::Relative => {
                panic!("or does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("or does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("or addrmode not implemented")
            }
            AddressMode::Immediate => {
                let addr = self.am_immediate();
                value = addr;
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageY => {
                panic!("or addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.bus.read(addr);
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.bus.read(addr);
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                value = self.bus.read(addr);
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y();
                value = self.bus.read(addr);
            }
        }

        self.a = self.a ^ value;

        // set zero flag
        if self.a == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.a & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn clv(&mut self) {
        self.p = self.p & 0xBF;
    }
    pub fn or(&mut self, addr_mode: AddressMode) {
        let mut value = 0x00;
        match addr_mode {
            AddressMode::Relative => {
                panic!("or does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("or does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("or addrmode not implemented")
            }
            AddressMode::Immediate => {
                let addr = self.am_immediate();
                value = addr;
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageY => {
                panic!("or addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.bus.read(addr);
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.bus.read(addr);
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                value = self.bus.read(addr);
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y();
                value = self.bus.read(addr);
            }
        }

        self.a = self.a | value;

        // set zero flag
        if self.a == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.a & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn bmi(&mut self) {
        let negative_flag = self.p & 0x80;
        if negative_flag != 0 {
            let offset = self.bus.read(self.pc) as i8;
            self.pc += 1;
            self.pc = (self.pc as u32 + offset as u32) as u16;
        } else {
            self.pc += 1;
        }
    }
    pub fn plp(&mut self) {
        self.sp += 1;
        let p_flag_value = self.bus.read(0x0100 + (self.sp as u16));
        self.p = (p_flag_value & 0xEF) | 0x20;
    }
    pub fn pha(&mut self) {
        let addr = self.sp as u16 + 0x0100;
        // println!("push a. A = {:02X}", self.a);
        self.bus.write(addr, self.a);
        self.sp -= 1;
    }
    pub fn cld(&mut self) {
        self.p = self.p & 0xF7;
    }
    pub fn cmp(&mut self, addr_mode: AddressMode) {
        let value: u8;
        match addr_mode {
            AddressMode::Relative => {
                panic!("cmp does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("cmp does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("cmp addrmode not implemented")
            }
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageY => {
                panic!("cmp addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.bus.read(addr as u16);
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y();
                value = self.bus.read(addr as u16);
            }
        }

        let compare = self.a as i16 - value as i16;
        // Carry flag
        if self.a >= value {
            self.p = self.p | 0x01;
        } else {
            self.p = self.p & 0xFE;
        }

        // set zero flag
        if compare == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = compare & 0x80;
        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }

    pub fn and(&mut self, addr_mode: AddressMode) {
        let value: u8;
        match addr_mode {
            AddressMode::Relative => {
                panic!("and does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("and does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("and addrmode not implemented")
            }
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageY => {
                panic!("and addrmode not implemented")
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr as u16);
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.bus.read(addr as u16);
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y();
                value = self.bus.read(addr as u16);
            }
        }

        self.a = self.a & value;

        // set zero flag
        if self.a == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.a & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }
    pub fn pla(&mut self) {
        self.sp += 1;
        let addr = 0x0100 + self.sp as u16;
        // println!("in pla this is the read value: {:02X}", self.bus.read(addr));
        self.a = self.bus.read(addr);

        // set zero flag
        if self.a == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }
        // negative
        let is_negative = self.a & 0x80;

        if is_negative == 0x80 {
            self.p = self.p | 0x80;
        } else {
            self.p = self.p & 0x7F;
        }
    }

    pub fn php(&mut self) {
        let sp_addr = 0x0100 + self.sp as u16;
        let status_flag = self.p | 0x10;
        // println!("status flag in php: {:02X}", status_flag);
        self.bus.write(sp_addr, status_flag);
        self.sp -= 1;
    }
    pub fn sed(&mut self) {
        self.p = self.p | 0x08;
    }
    pub fn sei(&mut self) {
        self.p = self.p | 0x04;
    }

    pub fn rts(&mut self) {
        self.sp += 1;
        let low = self.bus.read(self.sp as u16 + 0x0100) as u16;
        self.sp += 1;
        let high = (self.bus.read(self.sp as u16 + 0x0100) as u16) << 8;

        let addr = high | low;
        self.pc = addr as u16 + 1;
    }

    pub fn bpl(&mut self) {
        let negative = self.p & 0x80;

        if negative == 0 {
            let offset = self.bus.read(self.pc) as i8;
            self.pc += 1;
            self.pc = (self.pc as u32 + offset as u32) as u16;
        } else {
            self.pc += 1;
        }
    }
    pub fn bvc(&mut self) {
        let overflow = self.p & 0x40;

        if overflow == 0 {
            let offset = self.bus.read(self.pc) as i8;
            self.pc += 1;
            self.pc = (self.pc as u32 + offset as u32) as u16;
        } else {
            self.pc += 1;
        }
    }
    pub fn bvs(&mut self) {
        let overflow = self.p & 0x40;

        if overflow != 0 {
            let offset = self.bus.read(self.pc) as i8;
            self.pc += 1;
            self.pc = (self.pc as u32 + offset as u32) as u16;
        } else {
            self.pc += 1;
        }
    }
    pub fn bit(&mut self, addr_mode: AddressMode) {
        let value;
        let mask;
        match addr_mode {
            AddressMode::Relative => {
                panic!("bit does not use addrmode")
            }
            AddressMode::Indirect => {
                panic!("sta does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("sta addrmode not implemented")
            }
            AddressMode::Immediate => {
                panic!("sta addrmod e not implemented")
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr as u16);
                mask = self.a & value;
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
                mask = self.a & value;
            }
            AddressMode::ZeroPageX => {
                panic!("bit does not use addrmode")
            }
            AddressMode::ZeroPageY => {
                panic!("sta addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                panic!("bit does not use addrmode")
            }
            AddressMode::AbsoluteY => {
                panic!("bit does not use addrmode")
            }
            AddressMode::IndirectX => {
                panic!("bit does not use addrmode")
            }
            AddressMode::IndirectY => {
                panic!("bit does not use addrmode")
            }
        }

        // set zero flag
        if mask == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD
        }

        // Overflow
        let overflow = value & 0x40;
        if overflow == 0 {
            self.p = self.p & 0xBF;
        } else {
            self.p = self.p | 0x40
        }

        // negative
        let negative = value & 0x80;
        if negative == 0 {
            self.p = self.p & 0x7F;
        } else {
            self.p = self.p | 0x80;
        }
    }

    pub fn sta(&mut self, addr_mode: AddressMode) {
        match addr_mode {
            AddressMode::Relative => {
                panic!("sta does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("sta does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("sta addrmode not implemented")
            }
            AddressMode::Immediate => {
                panic!("sta addrmod e not implemented")
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute() as u16;
                self.bus.write(addr, self.a);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page() as u16;
                self.bus.write(addr, self.a);
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x() as u16;
                self.bus.write(addr, self.a);
            }
            AddressMode::ZeroPageY => {
                panic!("sta addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x() as u16;
                self.bus.write(addr, self.a);
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y() as u16;
                self.bus.write(addr, self.a);
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x() as u16;
                self.bus.write(addr, self.a);
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y() as u16;
                self.bus.write(addr, self.a);
            }
        }
    }

    pub fn bne(&mut self) {
        let zero_flag = self.p & 0x02;
        if zero_flag == 0 {
            let offset: i8 = self.bus.read(self.pc) as i8;
            self.pc += 1;
            self.pc = (self.pc as i32 + offset as i32) as u16
        } else {
            self.pc += 1;
        }
    }
    pub fn beq(&mut self) {
        let zero_flag = self.p & 0x02;
        if zero_flag == 0x02 {
            let offset = self.bus.read(self.pc) as i8;
            self.pc += 1;
            self.pc = (self.pc as i32 + offset as i32) as u16;
        } else {
            self.pc += 1;
        }
    }

    pub fn bcc(&mut self, addr_mode: AddressMode) {
        match addr_mode {
            AddressMode::Relative => {
                let carry = self.p & 0x01;
                if carry == 0x00 {
                    let offset = self.bus.read(self.pc) as i8;
                    self.pc = (self.pc as i32 + 1 + offset as i32) as u16;
                } else {
                    self.pc += 1;
                }
            }
            AddressMode::Indirect => {
                panic!("bcs does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::Immediate => {
                panic!("bcs addrmod e not implemented")
            }
            AddressMode::Absolute => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::ZeroPage => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::ZeroPageX => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::ZeroPageY => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::AbsoluteY => {
                panic!("bcs addrmode not implemented");
            }
            AddressMode::IndirectX => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::IndirectY => {
                panic!("bcs addrmode not implemented")
            }
        }
    }

    pub fn clc(&mut self) {
        self.p = self.p & 0xFE;
    }
    pub fn bcs(&mut self, addr_mode: AddressMode) {
        match addr_mode {
            AddressMode::Relative => {
                let carry = self.p & 0x01;
                if carry == 0x01 {
                    let offset = self.bus.read(self.pc) as i8;
                    self.pc = (self.pc as u32 + 1 + offset as u32) as u16;
                } else {
                    self.pc += 1;
                }
            }
            AddressMode::Indirect => {
                panic!("bcs does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::Immediate => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::Absolute => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::ZeroPage => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::ZeroPageX => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::ZeroPageY => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::AbsoluteY => {
                panic!("bcs addrmode not implemented");
            }
            AddressMode::IndirectX => {
                panic!("bcs addrmode not implemented")
            }
            AddressMode::IndirectY => {
                panic!("bcs addrmode not implemented")
            }
        }
    }

    pub fn sec(&mut self) {
        self.p = self.p | 0x01;
    }
    pub fn jsr(&mut self, addr_mode: AddressMode) {
        match addr_mode {
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("jsr does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("jsr addrmode not implemented")
            }
            AddressMode::Immediate => {
                panic!("jsr addrmode not implemented")
            }
            AddressMode::Absolute => {
                let mut sp = 0x0100 + self.sp as u16;
                let memory = self.am_absolute();
                self.pc -= 1;
                let high_byte = (self.pc >> 8) as u8;
                let low_byte = self.pc as u8;

                self.bus.write(sp, high_byte);
                sp -= 0x01;
                self.sp -= 0x01;
                self.bus.write(sp, low_byte);
                self.sp -= 0x01;

                self.pc = memory;
            }
            AddressMode::ZeroPage => {
                panic!("jsr addrmode not implemented")
            }
            AddressMode::ZeroPageX => {
                panic!("jsr addrmode not implemented")
            }
            AddressMode::ZeroPageY => {
                panic!("jsr addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                panic!("jsr addrmode not implemented")
            }
            AddressMode::AbsoluteY => {
                panic!("jsr addrmode not implemented");
            }
            AddressMode::IndirectX => {
                panic!("jsr addrmode not implemented")
            }
            AddressMode::IndirectY => {
                panic!("jsr addrmode not implemented")
            }
        }
    }

    pub fn stx(&mut self, addr_mode: AddressMode) {
        let mem_addr: u16;
        match addr_mode {
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("stx does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("stx addrmode not implemented")
            }
            AddressMode::Immediate => {
                panic!("stx addrmode not implemented")
            }
            AddressMode::Absolute => {
                mem_addr = self.am_absolute();
            }
            AddressMode::ZeroPage => {
                mem_addr = self.zero_page() as u16;

            }
            AddressMode::ZeroPageX => {
                panic!("stx addrmode not implemented")
            }
            AddressMode::ZeroPageY => {
                mem_addr = self.zero_page_y() as u16;

            }
            AddressMode::AbsoluteX => {
                panic!("stx addrmode not implemented")
            }
            AddressMode::AbsoluteY => {
                panic!("stx addrmode not implemented");
            }
            AddressMode::IndirectX => {
                panic!("stx addrmode not implemented")
            }
            AddressMode::IndirectY => {
                panic!("stx addrmode not implemented")
            }
        }
        // println!("sore x {} : x = {:02X}", mem_addr, self.x);
        self.bus.write(mem_addr as u16, self.x);
    }

    pub fn ldx(&mut self, addr_mode: AddressMode) {
        let value: u8;
        match addr_mode {
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("lda does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("lad addrmode not implemented")
            }
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                panic!("lad addrmode not implemented")
            }
            AddressMode::ZeroPageY => {
                let addr = self.zero_page_y();
                value = self.bus.read(addr as u16);
            }
            AddressMode::AbsoluteX => {
                panic!("lad addrmode not implemented")
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();

                value = self.bus.read(addr);
            }
            AddressMode::IndirectX => {
                panic!("lad addrmode not implemented")
            }
            AddressMode::IndirectY => {
                panic!("lad addrmode not implemented")
            }
        }
        if value == 0x00 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD;
        }

        let bit = value & 0x80;
        if bit != 0 {
            self.p = self.p | 0x80
        } else {
            self.p = self.p & 0x7F
        }

        self.x = value;
    }

    pub fn lda(&mut self, addr_mode: AddressMode) {
        let mut value: u8 = 0x00;
        match addr_mode {
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("lda does not use indrect")
            }
            AddressMode::Accumulator => {
                panic!("lad addrmode not implemented")
            }
            AddressMode::Immediate => {
                value = self.am_immediate();
            }
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPage => {
                let addr = self.zero_page();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageX => {
                let addr = self.zero_page_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::ZeroPageY => {
                panic!("lad addrmode not implemented")
            }
            AddressMode::AbsoluteX => {
                let addr = self.absolute_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::AbsoluteY => {
                let addr = self.absolute_y();
                value = self.bus.read(addr as u16);
            }
            AddressMode::IndirectX => {
                let addr = self.indirect_x();
                value = self.bus.read(addr as u16);
            }
            AddressMode::IndirectY => {
                let addr = self.indirect_y ();
                value = self.bus.read(addr as u16);
            }
        }
        self.a = value;
        if value == 0 {
            self.p = self.p | 0x02;
        } else {
            self.p = self.p & 0xFD;
        }

        self.p = self.p & 0x7F;
        self.p = self.p | (value & 0x80)
    }

    pub fn noop(&mut self) {
        return;
    }

    pub fn jmp(&mut self, addr_mode: AddressMode) {
        match addr_mode {
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => self.pc = self.indirect(),
            AddressMode::Accumulator => {}
            AddressMode::Immediate => {}
            AddressMode::Absolute => {
                let addr = self.am_absolute();
                self.pc = addr;
                // value = self.bus.read(addr)
            }
            AddressMode::ZeroPage => {}
            AddressMode::ZeroPageX => {}
            AddressMode::AbsoluteX => {}
            AddressMode::AbsoluteY => {}
            AddressMode::IndirectX => {}
            AddressMode::IndirectY => {}
            AddressMode::ZeroPageY => {
                panic!("jmp addrmode not implemented")
            }
        }
    }

    pub fn ror(&mut self, addr_mode: AddressMode) {
        let mut value: u8;
        let mut address: Option<u16> = None;

        match addr_mode {
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("ror does not use indrect")
            }
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
            AddressMode::ZeroPageY => {
                panic!("ror addrmode not implemented")
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
            value = value & 0xFE | 0x80;
        }else{
            value = value & 0x7F;
        }



        // clears the lsb on status and or's it with new lsb
        self.p = self.p & 0xFE;
        self.p = self.p | lsb;
        // set negative
        let neg_flag = value & 0x80;
        if neg_flag == 0x80{
            self.p = self.p | 0x80;
        }else{
            self.p = self.p & 0x7F;
        }
        // 1110 0101 -> E5 suppoed to be this
        // 0110 0101

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
            AddressMode::Relative => {
                panic!("does not use indrect")
            }
            AddressMode::Indirect => {
                panic!("adc does not use indeirect")
            }
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
            AddressMode::ZeroPageY => {
                panic!("adc addrmode not implemented")
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


        let res = self.a as u16 + value as u16 + (self.p & 0x01) as u16;

        // set carry flag
        if res as u16 > 0xFF{
            self.p = self.p | 0x01;
        }else{
            self.p = self.p & 0xFE
        }
        // set zero flag
        if res as u8 == 0{
            self.p = self.p | 0x02
        }else{
            self.p = self.p & 0xFD
        }
        // set overflow
        if (res ^ self.a as u16) & (res ^ value as u16) & 0x80 != 0{
            self.p = self.p | 0x40;
        }else{
            self.p = self.p & 0xBF
        }
        // set negative
        if res & 0x80 == 0x80{
            self.p = self.p | 0x80
        }else{
            self.p = self.p & 0x7F
        }
        self.a = res as u8;
    }

    pub fn indirect(&mut self) -> u16 {
        self.pc += 1;
        let low = self.bus.read(self.pc) as u16;
        if low == 0xFF {
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
        let value = self.bus.read(self.pc);
        self.pc += 1;
        value
    }

    pub fn zero_page(&mut self) -> u8 {
        let addr = self.bus.read(self.pc);
        self.pc += 1;
        addr
    }
    pub fn zero_page_x(&mut self) -> u8 {
        let arg = self.bus.read(self.pc);
        // using wrapping add instead of % 256
        let addr = arg.wrapping_add(self.x);
        addr
    }
    pub fn zero_page_y(&mut self) -> u8 {
        let arg = self.bus.read(self.pc);
        // using wrapping add instead of % 256
        let addr = arg.wrapping_add(self.y);
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
        first_byte = self.bus.read(self.pc) as u16;
        self.pc += 1;
        second_byte = self.bus.read(self.pc) as u16;
        let value = second_byte << 8 | first_byte;
        self.pc += 1;
        value
    }

    // pub fn is_end_of_program(&self) -> bool {
    //     if self.pc >= (self.ram.len() - 1) as u16 {
    //         true
    //     } else {
    //         false
    //     }
    // }
}
