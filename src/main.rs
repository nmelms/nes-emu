mod cpu;
use std::{fs};

use cpu::CPU;

fn main() {
    let res = std::fs::read("nestest.nes");
    let rom = res.unwrap();
    let mut cpu = CPU::new(rom.clone());

    while !cpu.is_end_of_program() {
        cpu.tick();
        println!("Value of a: {}", cpu.a)
    }
}
