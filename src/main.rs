mod cpu;
use cpu::CPU;

fn main() {
    CPU::tick(program[0]);
    println!("Hello, world!");
}
