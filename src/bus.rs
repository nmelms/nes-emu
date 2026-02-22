
pub struct Bus{
    ram: Vec<u8>,
    rom: Vec<u8>,
}

impl Bus {

    pub fn new(rom: Vec<u8>) -> Self{
        let rom = rom;
        let ram = vec![0u8; 2048];

        Self{ ram, rom }
    }

    pub fn read(&self, addr: u16) -> u8{
        // read the address in ram and return a u8
        if addr > 0x07ff && addr <= 0x1FFF{
            let mask = addr & 0x7ff;
            self.ram[mask as usize]
        }else if addr > 0x1FFF{
            panic!("addr is outside of rang in read")
        }else{
            self.ram[addr as usize]
        }

    }

    pub fn write(){
        todo!("impelement write")
    }
}