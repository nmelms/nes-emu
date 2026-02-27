pub struct Mapper0 {
    rom: Vec<u8>,
    is_nes_rom_128: bool,
    debug: Vec<u8>,
}

impl Mapper0 {
    pub fn new(rom: Vec<u8>) -> Self {
        // see if the rom is 128 or 256 so we can decided how the banks work
        let is_nes_rom_128 = if rom[4] == 1 { true } else { false };
        let debug = vec![0u8; 8192];
        Self {
            rom,
            is_nes_rom_128,
            debug
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        if addr >= 0x6000 && addr <= 0x7FFF {
            let offset = addr - 0x6000;
            self.debug[offset as usize]
        } else if addr >= 0x8000 && addr <= 0xBFFF {
            let offset_addr = addr + 0x10 - 0x8000;
            self.rom[offset_addr as usize]
        } else if addr >= 0xC000 && addr <= 0xFFFF {
            if self.is_nes_rom_128 {
                // mod by 16kb
                let offset_addr = (addr - 0x8000) % 0x4000 + 0x10;
                self.rom[offset_addr as usize]
            } else {
                let offset_addr = addr - 0xC000 + 0x10;
                self.rom[offset_addr as usize]
            }
        } else {
            panic!("read out of bounds in mapper 0")
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        if addr >= 0x6000 && addr <= 0x7FFF {
            let offset = addr - 0x6000;
            self.debug[offset as usize] = value;
        }else{
        panic!(
                "You impelented write in mapper 0 only for the debug test statements, you got value: {}",
                value
            )
        }
    }
}

// 0x6000 - 0x7FFF unbanked rom mirrored
// 0x8000 - 0xBFFF first 16kb
// 0xC000 - 0xFFFF last 16kb
// 0x0000 - 0x1FFF -- ppu
