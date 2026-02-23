use crate::mapper_0::{self, Mapper0};
pub struct Bus{
    ram: Vec<u8>,
    rom: Vec<u8>,
}

impl Bus {
    // const mapper0: Mapper0 = mapper_0::Mapper0;

    pub fn new(rom: Vec<u8>) -> Self{
        let rom = rom;
        let ram = vec![0u8; 2048];

        Self{ ram, rom }
    }

    pub fn write(){
        todo!("impelement write")
    }
}