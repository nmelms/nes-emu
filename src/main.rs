mod cpu;
use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    while !cpu.is_end_of_program(){
        cpu.tick();
        println!("Value of a: {}", cpu.a)    
    }    

}
