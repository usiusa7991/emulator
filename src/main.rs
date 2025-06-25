mod cpu;
mod register;

fn main() {
    println!("Emulator is starting!");

    let mut cpu = cpu::CPU::new();

    loop {
        cpu.step();
    }
}
