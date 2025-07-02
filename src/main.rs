use emulator::cpu::CPU;

fn main() {
	println!("Emulator is starting!");

	let mut cpu = CPU::new();

	loop {
		cpu.step();
	}
}
