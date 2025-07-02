use emulator::cpu::CPU;

#[test]
fn nop() {
    let mut cpu = CPU::new();
    cpu.bus.write_byte(0x00, 0x00);
    cpu.step();
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn ld_bc_d16() {
    let mut cpu = CPU::new();

    // LD BC, 0x0305 という命令のバイト列
    // 0x01: LD BC, d16 のオペコード
    // 0x05: 16ビット即値の下位バイト (d16のLSB)
    // 0x03: 16ビット即値の上位バイト (d16のMSB)
    let program = vec![0x01, 0x05, 0x03];

    // プログラムをメモリに書き込む
    for (i, &byte) in program.iter().enumerate() {
        cpu.bus.write_byte(i as u16, byte);
    }

    // 1命令実行する
    cpu.step();

    // 結果を検証する
    assert_eq!(cpu.pc, 0x03); // プログラムカウンタは3進む
    assert_eq!(cpu.registers.get_bc(), 0x0305); // BCレジスタに値がロードされたか
}

#[test]
fn ld_bca_a() {
    let mut cpu = CPU::new();

    // レジスタの初期値を設定
    cpu.registers.a = 0xAB; // Aレジスタにテスト用の値を設定
    cpu.registers.set_bc(0x1234); // BCレジスタにメモリアドレスを設定

    // LD (BC), A 命令 (0x02)
    cpu.bus.write_byte(0x00, 0x02);

    // 1命令実行
    cpu.step();

    // 結果を検証
    assert_eq!(cpu.pc, 0x01); // PCは1進む
    assert_eq!(cpu.bus.read_byte(0x1234), 0xAB); // BCが指すアドレスにAの値が書き込まれたか
}

#[test]
fn inc_bc() {
    let mut cpu = CPU::new();

    // レジスタの初期値を設定
    cpu.registers.set_bc(0x1234);

    // INC BC 命令 (0x03)
    cpu.bus.write_byte(0x00, 0x03);

    // 1命令実行
    cpu.step();

    // 結果を検証
    assert_eq!(cpu.pc, 0x01); // PCは1進む
    assert_eq!(cpu.registers.get_bc(), 0x1235); // BCレジスタの値が1増加したか
}
