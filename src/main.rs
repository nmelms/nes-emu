mod cpu;
mod bus;
use std::{fs};

use cpu::CPU;
use bus::Bus;


fn main() {
    let res = std::fs::read("nestest.nes");
    let rom = res.unwrap();
    let bus =  Bus::new(rom);
    let mut cpu = CPU::new(bus);

    while !cpu.is_end_of_program() {
        cpu.tick();
        println!("Value of a: {}", cpu.a)
    }
}
