use crate::mapper_0::{self, Mapper0};
pub struct Bus {
    ram: Vec<u8>,
    mapper: Mapper0,
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        let rom = rom;
        let ram = vec![0u8; 2048];
        let mapper = Mapper0::new(rom);
        Self { ram, mapper }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let value: u8;
        if addr <= 0x1FFF {
            if addr > 0x07FF {
                let mask = addr & 0x07FF;
                value = self.ram[mask as usize];
            } else {
                value = self.ram[addr as usize];
            }
        } else {
            value = self.mapper.read(addr)
        }

        return value;
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr <= 0x1FFF {
            if addr > 0x07FF {
                let mask = addr & 0x07FF;
                self.ram[mask as usize] = value;
            } else {
                self.ram[addr as usize] = value;
            }
        }else if addr >= 0x6000 && addr <= 0x7FFF{
            self.mapper.write(addr, value);
        } else {
            panic!(
                "you have nto yet impemented write to the mapper go to bus and finish that now."
            );
        }
    }
}
