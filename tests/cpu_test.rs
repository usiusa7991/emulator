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

#[test]
fn inc_b() {
    let mut cpu = CPU::new();

    // 1. 通常のインクリメント
    cpu.registers.b = 0x01;
    cpu.bus.write_byte(0x00, 0x04); // INC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x02);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);

    // 2. ハーフキャリー
    cpu.pc = 0;
    cpu.registers.b = 0x0F;
    cpu.bus.write_byte(0x00, 0x04); // INC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x10);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.half_carry);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.b = 0xFF;
    cpu.bus.write_byte(0x00, 0x04); // INC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x00);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
}

#[test]
fn dec_b() {
    let mut cpu = CPU::new();

    // 1. 通常のデクリメント
    cpu.registers.b = 0x02;
    cpu.bus.write_byte(0x00, 0x05); // DEC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x01);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);

    // 2. ハーフキャリー発生（下位4ビットが0のとき）
    cpu.pc = 0;
    cpu.registers.b = 0x10;
    cpu.bus.write_byte(0x00, 0x05); // DEC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x0F);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.half_carry);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.b = 0x01;
    cpu.bus.write_byte(0x00, 0x05); // DEC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x00);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
}

#[test]
fn ld_b_d8() {
    let mut cpu = CPU::new();

    // LD B, 0x08 命令
    // 0x06: LD B, d8 のオペコード
    // 0x08: 即値データ
    cpu.bus.write_byte(0x00, 0x06); // LD B, d8
    cpu.bus.write_byte(0x01, 0x08); // d8 = 0x08

    cpu.step();

    assert_eq!(cpu.registers.b, 0x08);
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn rlca() {
    let mut cpu = CPU::new();

    // 1. キャリーが発生するケース (MSBが1)
    cpu.registers.a = 0b1000_0001;
    cpu.bus.write_byte(0x00, 0x07); // RLCA
    cpu.step();

    assert_eq!(cpu.registers.a, 0b0000_0011);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.carry);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);

    // 2. キャリーが発生しないケース (MSBが0)
    cpu.pc = 0; // PCをリセット
    cpu.registers.a = 0b0100_0010;
    cpu.bus.write_byte(0x00, 0x07); // RLCA
    cpu.step();

    assert_eq!(cpu.registers.a, 0b1000_0100);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn ld_a16_sp() {
    let mut cpu = CPU::new();

    // SPにテスト用の値をセット
    cpu.sp = 0xBEEF;

    // LD (0x1234), SP 命令
    // 0x08: LD (a16), SP のオペコード
    // 0x34: アドレス下位バイト
    // 0x12: アドレス上位バイト
    let program = vec![0x08, 0x34, 0x12];

    // プログラムをメモリに書き込む
    for (i, &byte) in program.iter().enumerate() {
        cpu.bus.write_byte(i as u16, byte);
    }

    // 1命令実行
    cpu.step();

    // 結果を検証
    // 0x1234にSPの下位バイト、0x1235にSPの上位バイトが書き込まれているはず
    assert_eq!(cpu.bus.read_byte(0x1234), 0xEF); // LSB
    assert_eq!(cpu.bus.read_byte(0x1235), 0xBE); // MSB
    assert_eq!(cpu.pc, 0x03); // PCは3進む
}
