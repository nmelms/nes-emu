mod bus;
mod cpu;
mod mapper_0;
use std::fs;

use bus::Bus;
use cpu::CPU;

fn main() {
    let res = std::fs::read("nestest.nes");
    let rom = res.unwrap();
    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    let running = true;

    while running {
        cpu.tick();
        println!("Value of a: {}", cpu.a)
    }
}
