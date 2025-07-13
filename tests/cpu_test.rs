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
    assert!(cpu.registers.f.subtract);

    // 2. ハーフキャリー発生（下位4ビットが0のとき）
    cpu.pc = 0;
    cpu.registers.b = 0x10;
    cpu.bus.write_byte(0x00, 0x05); // DEC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x0F);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.b = 0x01;
    cpu.bus.write_byte(0x00, 0x05); // DEC B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x00);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
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
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);

    // 3. 結果が0になるケース
    cpu.pc = 0; // PCをリセット
    cpu.registers.a = 0b0000_0000;
    cpu.bus.write_byte(0x00, 0x07); // RLCA
    cpu.step();

    assert_eq!(cpu.registers.a, 0b0000_0000);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.carry);
    assert!(!cpu.registers.f.zero); // ゼロフラグが立たない
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
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

#[test]
fn add_hl_bc() {
    let mut cpu = CPU::new();
    
    // 初期値を設定
    cpu.registers.set_hl(0x1000);
    cpu.registers.set_bc(0x2000);
    
    // ADD HL, BC 命令 (0x09)
    cpu.bus.write_byte(0x00, 0x09);
    
    // 実行
    cpu.step();
    
    // 結果を検証
    assert_eq!(cpu.registers.get_hl(), 0x3000);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn ld_a_bci() {
    let mut cpu = CPU::new();
    
    // BCレジスタにアドレスを設定
    cpu.registers.set_bc(0x1234);
    
    // そのアドレスにテスト値を書き込み
    cpu.bus.write_byte(0x1234, 0xAB);
    
    // LD A, (BC) 命令 (0x0A)
    cpu.bus.write_byte(0x00, 0x0A);
    
    // 実行
    cpu.step();
    
    // 結果を検証
    assert_eq!(cpu.registers.a, 0xAB);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn dec_bc() {
    let mut cpu = CPU::new();
    
    // 初期値を設定
    cpu.registers.set_bc(0x1234);
    
    // DEC BC 命令 (0x0B)
    cpu.bus.write_byte(0x00, 0x0B);
    
    // 実行
    cpu.step();
    
    // 結果を検証
    assert_eq!(cpu.registers.get_bc(), 0x1233);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn inc_c() {
    let mut cpu = CPU::new();
    
    // 初期値を設定
    cpu.registers.c = 0x42;
    
    // INC C 命令 (0x0C)
    cpu.bus.write_byte(0x00, 0x0C);
    
    // 実行
    cpu.step();
    
    // 結果を検証
    assert_eq!(cpu.registers.c, 0x43);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn dec_c() {
    let mut cpu = CPU::new();
    
    // 初期値を設定
    cpu.registers.c = 0x42;
    
    // DEC C 命令 (0x0D)
    cpu.bus.write_byte(0x00, 0x0D);
    
    // 実行
    cpu.step();
    
    // 結果を検証
    assert_eq!(cpu.registers.c, 0x41);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn ld_c_d8() {
    let mut cpu = CPU::new();
    
    // LD C, 0x42 命令
    cpu.bus.write_byte(0x00, 0x0E); // LD C, d8
    cpu.bus.write_byte(0x01, 0x42); // d8 = 0x42
    
    // 実行
    cpu.step();
    
    // 結果を検証
    assert_eq!(cpu.registers.c, 0x42);
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn rrca() {
    let mut cpu = CPU::new();
    
    // 1. 最下位ビットが1の場合（キャリーが発生）
    cpu.registers.a = 0b0000_0001;
    cpu.bus.write_byte(0x00, 0x0F); // RRCA
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0b1000_0000); // 最下位ビットが最上位に移動
    assert!(cpu.registers.f.carry); // キャリーフラグが設定される
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert_eq!(cpu.pc, 0x01);
    
    // 2. 最下位ビットが0の場合（キャリーが発生しない）
    cpu.pc = 0; // PCをリセット
    cpu.registers.a = 0b0000_0010;
    cpu.bus.write_byte(0x00, 0x0F); // RRCA
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0b0000_0001); // 右に1ビットシフト
    assert!(!cpu.registers.f.carry); // キャリーフラグがクリアされる
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert_eq!(cpu.pc, 0x01);

    // 3. 結果が0になるケース
    cpu.pc = 0; // PCをリセット
    cpu.registers.a = 0b0000_0000;
    cpu.bus.write_byte(0x00, 0x0F); // RRCA
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0b0000_0000);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.carry);
    assert!(!cpu.registers.f.zero); // ゼロフラグが立たない
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
}

#[test]
fn ld_de_d16() {
    let mut cpu = CPU::new();
    cpu.bus.write_byte(0x00, 0x11); // LD DE, d16
    cpu.bus.write_byte(0x01, 0x34); // LSB
    cpu.bus.write_byte(0x02, 0x12); // MSB

    cpu.step();

    assert_eq!(cpu.registers.get_de(), 0x1234);
    assert_eq!(cpu.pc, 0x03);
}

#[test]
fn ld_dei_a() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0xAB;
    cpu.registers.set_de(0x1234);

    cpu.bus.write_byte(0x00, 0x12); // LD (DE), A

    cpu.step();

    assert_eq!(cpu.bus.read_byte(0x1234), 0xAB);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn inc_de() {
    let mut cpu = CPU::new();
    cpu.registers.set_de(0x1234);

    cpu.bus.write_byte(0x00, 0x13); // INC DE

    cpu.step();

    assert_eq!(cpu.registers.get_de(), 0x1235);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn inc_d() {
    let mut cpu = CPU::new();
    cpu.registers.d = 0x42;

    cpu.bus.write_byte(0x00, 0x14); // INC D

    cpu.step();

    assert_eq!(cpu.registers.d, 0x43);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn dec_d() {
    let mut cpu = CPU::new();
    cpu.registers.d = 0x42;

    cpu.bus.write_byte(0x00, 0x15); // DEC D

    cpu.step();

    assert_eq!(cpu.registers.d, 0x41);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn ld_d_d8() {
    let mut cpu = CPU::new();
    cpu.bus.write_byte(0x00, 0x16); // LD D, d8
    cpu.bus.write_byte(0x01, 0x42); // d8 = 0x42

    cpu.step();

    assert_eq!(cpu.registers.d, 0x42);
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn rla() {
    let mut cpu = CPU::new();
    
    // 1. キャリーが発生するケース
    cpu.registers.a = 0b1000_0000;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x17); // RLA
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0b0000_0000); // 左シフト + キャリー
    assert!(cpu.registers.f.carry); // キャリーフラグが設定される
    assert!(!cpu.registers.f.zero); // 結果が0でもゼロフラグは立たない
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert_eq!(cpu.pc, 0x01);
    
    // 2. キャリーが発生しないケース
    cpu.pc = 0; // PCをリセット
    cpu.registers.a = 0b0100_0000;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x17); // RLA
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0b1000_0001); // 左シフト + キャリー
    assert!(!cpu.registers.f.carry); // キャリーフラグがクリアされる
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert_eq!(cpu.pc, 0x01);

    // 3. 結果が0になるケース
    cpu.pc = 0; // PCをリセット
    cpu.registers.a = 0b0000_0000;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x17); // RLA
    
    cpu.step();
    
    assert_eq!(cpu.registers.a, 0b0000_0000);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.carry);
    assert!(!cpu.registers.f.zero); // ゼロフラグがタタナイ
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
}

#[test]
fn jr_s8() {
    let mut cpu = CPU::new();

    // 1. 正のオフセット（前方ジャンプ）
    // JR +5 命令: 0x18 (JR) + 0x05 (オフセット)
    cpu.bus.write_byte(0x00, 0x18); // JR命令
    cpu.bus.write_byte(0x01, 0x05); // +5のオフセット

    cpu.step();

    // PC = 0x00 + 2 + 5 = 0x07
    assert_eq!(cpu.pc, 0x07);

    // 2. 負のオフセット（後方ジャンプ）
    cpu.pc = 0x100; // PCを0x100に設定
    cpu.bus.write_byte(0x100, 0x18); // JR命令
    cpu.bus.write_byte(0x101, 0xFE); // -2のオフセット (0xFE as i8 = -2)

    cpu.step();

    // PC = 0x100 + 2 + (-2) = 0x100
    assert_eq!(cpu.pc, 0x100);

    // 3. 大きな正のオフセット
    cpu.pc = 0x200;
    cpu.bus.write_byte(0x200, 0x18); // JR命令
    cpu.bus.write_byte(0x201, 0x7F); // +127のオフセット

    cpu.step();

    // PC = 0x200 + 2 + 127 = 0x281
    assert_eq!(cpu.pc, 0x281);

    // 4. 大きな負のオフセット
    cpu.pc = 0x300;
    cpu.bus.write_byte(0x300, 0x18); // JR命令
    cpu.bus.write_byte(0x301, 0x80); // -128のオフセット (0x80 as i8 = -128)

    cpu.step();

    // PC = 0x300 + 2 + (-128) = 0x282
    assert_eq!(cpu.pc, 0x282);
}

#[test]
fn add_hl_de() {
    let mut cpu = CPU::new();

    // HL = 0x1234, DE = 0x1111
    cpu.registers.set_hl(0x1234);
    cpu.registers.set_de(0x1111);

    // ADD HL, DE 命令 (0x19)
    cpu.bus.write_byte(0x00, 0x19);

    cpu.step();

    // 結果: HL = 0x2345
    assert_eq!(cpu.registers.get_hl(), 0x2345);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);
    // Hフラグ: (0x1234 & 0xFFF) + (0x1111 & 0xFFF) = 0x345 < 0xFFF → false
    assert!(!cpu.registers.f.half_carry);
    // Cフラグ: 0x1234 + 0x1111 = 0x2345 < 0x10000 → false
    assert!(!cpu.registers.f.carry);

    // キャリー発生のケース
    cpu.registers.set_hl(0xFFFF);
    cpu.registers.set_de(0x0001);
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x19);

    cpu.step();

    // HL = 0x0000
    assert_eq!(cpu.registers.get_hl(), 0x0000);
    // Cフラグ: 0xFFFF + 0x0001 = 0x10000 → キャリー発生
    assert!(cpu.registers.f.carry);
    // Hフラグ: (0xFFF + 0x001) = 0x1000 > 0xFFF → true
    assert!(cpu.registers.f.half_carry);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);
}

#[test]
fn ld_a_dei() {
    let mut cpu = CPU::new();

    // DE = 0x1234, メモリ[0x1234] = 0xAB
    cpu.registers.set_de(0x1234);
    cpu.bus.write_byte(0x1234, 0xAB);

    // LD A, (DE) 命令 (0x1A)
    cpu.bus.write_byte(0x00, 0x1A);

    cpu.step();

    // Aレジスタに0xABがロードされていること
    assert_eq!(cpu.registers.a, 0xAB);
    // PCは1進む
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn dec_de() {
    let mut cpu = CPU::new();

    // DE = 0x1234
    cpu.registers.set_de(0x1234);

    // DEC DE 命令 (0x1B)
    cpu.bus.write_byte(0x00, 0x1B);

    cpu.step();

    // DEが1減る
    assert_eq!(cpu.registers.get_de(), 0x1233);

    // アンダーフローのテスト
    cpu.registers.set_de(0x0000);
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x1B);

    cpu.step();

    // 0x0000 - 1 = 0xFFFF
    assert_eq!(cpu.registers.get_de(), 0xFFFF);
}

#[test]
fn inc_e() {
    let mut cpu = CPU::new();

    // 1. 通常のインクリメント
    cpu.registers.e = 0x01;
    cpu.bus.write_byte(0x00, 0x1C); // INC E
    cpu.step();
    assert_eq!(cpu.registers.e, 0x02);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);

    // 2. ハーフキャリー
    cpu.pc = 0;
    cpu.registers.e = 0x0F;
    cpu.bus.write_byte(0x00, 0x1C); // INC E
    cpu.step();
    assert_eq!(cpu.registers.e, 0x10);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.half_carry);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.e = 0xFF;
    cpu.bus.write_byte(0x00, 0x1C); // INC E
    cpu.step();
    assert_eq!(cpu.registers.e, 0x00);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
}

#[test]
fn dec_e() {
    let mut cpu = CPU::new();

    // 1. 通常のデクリメント
    cpu.registers.e = 0x02;
    cpu.bus.write_byte(0x00, 0x1D); // DEC E
    cpu.step();
    assert_eq!(cpu.registers.e, 0x01);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 2. ハーフキャリー発生（下位4ビットが0のとき）
    cpu.pc = 0;
    cpu.registers.e = 0x10;
    cpu.bus.write_byte(0x00, 0x1D); // DEC E
    cpu.step();
    assert_eq!(cpu.registers.e, 0x0F);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.e = 0x01;
    cpu.bus.write_byte(0x00, 0x1D); // DEC E
    cpu.step();
    assert_eq!(cpu.registers.e, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
}

#[test]
fn ld_e_d8() {
    let mut cpu = CPU::new();

    // LD E, 0x42 命令
    // 0x1E: LD E, d8 のオペコード
    // 0x42: 即値データ
    cpu.bus.write_byte(0x00, 0x1E); // LD E, d8
    cpu.bus.write_byte(0x01, 0x42); // d8 = 0x42

    cpu.step();

    assert_eq!(cpu.registers.e, 0x42);
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn rra() {
    let mut cpu = CPU::new();

    // 1. キャリーが立っていない場合
    cpu.registers.a = 0b1000_0001;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x1F); // RRA
    cpu.step();

    // 0b1000_0001 >> 1 = 0b0100_0000, キャリーイン=0, new_value=0b0100_0000
    assert_eq!(cpu.registers.a, 0b0100_0000);
    // Cフラグは元のAのビット0（1）なのでtrue
    assert!(cpu.registers.f.carry);
    // Z, N, Hは常にfalse
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);

    // 2. キャリーが立っている場合
    cpu.pc = 0;
    cpu.registers.a = 0b0000_0010;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x1F); // RRA
    cpu.step();

    // 0b0000_0010 >> 1 = 0b0000_0001, キャリーイン=1, new_value=0b1000_0001
    assert_eq!(cpu.registers.a, 0b1000_0001);
    // Cフラグは元のAのビット0（0）なのでfalse
    assert!(!cpu.registers.f.carry);

    // 3. 結果が0になる場合
    cpu.pc = 0; // PCをリセット
    cpu.registers.a = 0b0000_0000;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x1F); // RRA
    cpu.step();

    assert_eq!(cpu.registers.a, 0b0000_0000);
    assert!(!cpu.registers.f.carry);
    assert!(!cpu.registers.f.zero);
}

#[test]
fn jr_nz_s8() {
    let mut cpu = CPU::new();

    // Zeroフラグが0（ジャンプする場合）
    cpu.registers.f.zero = false;
    cpu.pc = 0x100;
    cpu.bus.write_byte(0x100, 0x20); // JR NZ, s8
    cpu.bus.write_byte(0x101, 0x05); // +5

    cpu.step();

    // PC = 0x100 + 2 + 5 = 0x107
    assert_eq!(cpu.pc, 0x107);

    // Zeroフラグが1（ジャンプしない場合）
    cpu.registers.f.zero = true;
    cpu.pc = 0x200;
    cpu.bus.write_byte(0x200, 0x20); // JR NZ, s8
    cpu.bus.write_byte(0x201, 0x05); // +5

    cpu.step();

    // PC = 0x200 + 2 = 0x202
    assert_eq!(cpu.pc, 0x202);
}

#[test]
fn ld_hl_d16() {
    let mut cpu = CPU::new();

    // LD HL, 0x1234 命令
    // 0x21: LD HL, d16 のオペコード
    // 0x34: 下位バイト
    // 0x12: 上位バイト
    cpu.bus.write_byte(0x00, 0x21); // LD HL, d16
    cpu.bus.write_byte(0x01, 0x34); // d16 LSB
    cpu.bus.write_byte(0x02, 0x12); // d16 MSB

    cpu.step();

    assert_eq!(cpu.registers.get_hl(), 0x1234);
    assert_eq!(cpu.pc, 0x03);
}

#[test]
fn ld_hlp_a() {
    let mut cpu = CPU::new();

    // HL = 0x1234, A = 0xAB
    cpu.registers.set_hl(0x1234);
    cpu.registers.a = 0xAB;

    // LD (HL+), A 命令 (0x22)
    cpu.bus.write_byte(0x00, 0x22);

    cpu.step();

    // HLの指すアドレスにAの値が書き込まれている
    assert_eq!(cpu.bus.read_byte(0x1234), 0xAB);
    // HLが+1されている
    assert_eq!(cpu.registers.get_hl(), 0x1235);
    // PCは1進む
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn inc_hl() {
    let mut cpu = CPU::new();

    // HL = 0x1234
    cpu.registers.set_hl(0x1234);

    // INC HL 命令 (0x23)
    cpu.bus.write_byte(0x00, 0x23);

    cpu.step();

    // HLが1増える
    assert_eq!(cpu.registers.get_hl(), 0x1235);
    // フラグは変化しない（全てfalseのまま）
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);

    // アンダーフローのテスト
    cpu.registers.set_hl(0xFFFF);
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x23);

    cpu.step();

    // 0xFFFF + 1 = 0x0000
    assert_eq!(cpu.registers.get_hl(), 0x0000);
}

#[test]
fn inc_h() {
    let mut cpu = CPU::new();

    // 1. 通常のインクリメント
    cpu.registers.h = 0x01;
    cpu.bus.write_byte(0x00, 0x24); // INC H
    cpu.step();
    assert_eq!(cpu.registers.h, 0x02);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);

    // 2. ハーフキャリー
    cpu.pc = 0;
    cpu.registers.h = 0x0F;
    cpu.bus.write_byte(0x00, 0x24); // INC H
    cpu.step();
    assert_eq!(cpu.registers.h, 0x10);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.half_carry);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.h = 0xFF;
    cpu.bus.write_byte(0x00, 0x24); // INC H
    cpu.step();
    assert_eq!(cpu.registers.h, 0x00);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
}

#[test]
fn dec_h() {
    let mut cpu = CPU::new();

    // 1. 通常のデクリメント
    cpu.registers.h = 0x02;
    cpu.bus.write_byte(0x00, 0x25); // DEC H
    cpu.step();
    assert_eq!(cpu.registers.h, 0x01);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 2. ハーフキャリー発生（下位4ビットが0のとき）
    cpu.pc = 0;
    cpu.registers.h = 0x10;
    cpu.bus.write_byte(0x00, 0x25); // DEC H
    cpu.step();
    assert_eq!(cpu.registers.h, 0x0F);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.h = 0x01;
    cpu.bus.write_byte(0x00, 0x25); // DEC H
    cpu.step();
    assert_eq!(cpu.registers.h, 0x00);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
}

#[test]
fn ld_h_d8() {
    let mut cpu = CPU::new();

    // LD H, 0x77 命令
    // 0x26: LD H, d8 のオペコード
    // 0x77: 即値データ
    cpu.bus.write_byte(0x00, 0x26); // LD H, d8
    cpu.bus.write_byte(0x01, 0x77); // d8 = 0x77

    cpu.step();

    assert_eq!(cpu.registers.h, 0x77);
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn daa() {
    let mut cpu = CPU::new();

    // 1. 加算後の調整 - 下位4ビットが0x0A-0x0Fの場合
    cpu.registers.a = 0x0A; // 0x0A > 0x09
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x27); // DAA
    cpu.step();

    assert_eq!(cpu.registers.a, 0x10); // 0x0A + 0x06 = 0x10
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract); // 変更されない - DAAはsubtractフラグを変更しない
    assert!(!cpu.registers.f.half_carry); // 常にクリア - DAA実行後はhalf_carryは常にfalse
    assert!(!cpu.registers.f.carry);

    // 2. 加算後の調整 - 上位4ビットが0x9A-0xFFの場合
    cpu.pc = 0;
    cpu.registers.a = 0x9A; // 0x9A > 0x99
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x27); // DAA
    cpu.step();

    assert_eq!(cpu.registers.a, 0x00); // 0x9A + 0x60 = 0xFA, オーバーフローで0x00
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.carry); // キャリーが発生 - 0x9A + 0x60 = 0xFAでオーバーフロー

    // 3. 加算後の調整 - 両方の条件が満たされる場合
    cpu.pc = 0;
    cpu.registers.a = 0x9F; // 0x9F > 0x99 かつ (0x9F & 0x0F) = 0x0F > 0x09
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x27); // DAA
    cpu.step();

    assert_eq!(cpu.registers.a, 0x05); // 0x9F + 0x06 + 0x60 = 0x105, オーバーフローで0x05
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.carry);

    // 4. 減算後の調整 - 調整なし
    cpu.pc = 0;
    cpu.registers.a = 0x45;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x27); // DAA
    cpu.step();

    assert_eq!(cpu.registers.a, 0x45); // 調整なし
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract); // 変更されない
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);

    // 5. 減算後の調整 - half_carryフラグがある場合
    cpu.pc = 0;
    cpu.registers.a = 0x45;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x27); // DAA
    cpu.step();

    assert_eq!(cpu.registers.a, 0x3F); // 0x45 - 0x06 = 0x3F
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);

    // 6. 減算後の調整 - carryフラグがある場合
    // DAAの仕様では、減算時にcarryフラグが立っている場合は0x60を引く
    cpu.pc = 0;
    cpu.registers.a = 0x45;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x27); // DAA
    cpu.step();

    assert_eq!(cpu.registers.a, 0xE5); // 0x45 - 0x60 = 0xE5 (wrapping_sub)
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.carry); // 保持される - 最終的なBCDの値 = 0xE5 > 0x99 であるため

    // 7. 結果が0になる場合
    cpu.pc = 0;
    cpu.registers.a = 0x00;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x27); // DAA
    cpu.step();

    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn jr_z_s8() {
    // 1. Zeroフラグが立っている場合（ジャンプする）
    let mut cpu = CPU::new();
    cpu.pc = 0x1000;
    cpu.registers.f.zero = true;
    cpu.bus.write_byte(0x1000, 0x28); // JR Z, s8
    cpu.bus.write_byte(0x1001, 0x05); // +5
    cpu.step();
    assert_eq!(cpu.pc, 0x1000 + 2 + 5); // 0x1007

    // 2. Zeroフラグが立っていない場合（ジャンプしない）
    let mut cpu = CPU::new();
    cpu.pc = 0x2000;
    cpu.registers.f.zero = false;
    cpu.bus.write_byte(0x2000, 0x28); // JR Z, s8
    cpu.bus.write_byte(0x2001, 0x05); // +5
    cpu.step();
    assert_eq!(cpu.pc, 0x2002); // 通常通り次命令へ

    // 3. 負のオフセット（-2）でジャンプ
    let mut cpu = CPU::new();
    cpu.pc = 0x3000;
    cpu.registers.f.zero = true;
    cpu.bus.write_byte(0x3000, 0x28); // JR Z, s8
    cpu.bus.write_byte(0x3001, 0xFE); // -2（0xFE as i8 = -2）
    cpu.step();
    assert_eq!(cpu.pc, 0x3000 + 2 - 2); // 0x3000

    // 4. オフセット0（ジャンプ先は次命令と同じ）
    let mut cpu = CPU::new();
    cpu.pc = 0x4000;
    cpu.registers.f.zero = true;
    cpu.bus.write_byte(0x4000, 0x28); // JR Z, s8
    cpu.bus.write_byte(0x4001, 0x00); // 0
    cpu.step();
    assert_eq!(cpu.pc, 0x4002); // 0x4000 + 2 + 0
}

#[test]
fn add_hl_hl() {
    let mut cpu = CPU::new();
    // 1. 通常の加算
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x00, 0x29); // ADD HL, HL
    cpu.step();
    assert_eq!(cpu.registers.get_hl(), 0x2468);
    assert_eq!(cpu.pc, 0x01);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);
    // Hフラグ: (0x1234 & 0xFFF) + (0x1234 & 0xFFF) = 0x468 < 0xFFF → false
    assert!(!cpu.registers.f.half_carry);
    // Cフラグ: 0x1234 + 0x1234 = 0x2468 < 0x10000 → false
    assert!(!cpu.registers.f.carry);

    // 2. キャリー発生のケース
    cpu.registers.set_hl(0x8000);
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x29); // ADD HL, HL
    cpu.step();
    // 0x8000 + 0x8000 = 0x10000 → 0x0000
    assert_eq!(cpu.registers.get_hl(), 0x0000);
    // Cフラグ: キャリー発生
    assert!(cpu.registers.f.carry);
    // Hフラグ: (0x8000 & 0xFFF) + (0x8000 & 0xFFF) = 0x0000 + 0x0000 = 0x0000 → false
    assert!(!cpu.registers.f.half_carry);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);

    // 3. ハーフキャリー発生のケース
    cpu.registers.set_hl(0x0FFF);
    cpu.pc = 0x20;
    cpu.bus.write_byte(0x20, 0x29); // ADD HL, HL
    cpu.step();
    // 0x0FFF + 0x0FFF = 0x1FFE
    assert_eq!(cpu.registers.get_hl(), 0x1FFE);
    // Hフラグ: (0x0FFF & 0xFFF) + (0x0FFF & 0xFFF) = 0x0FFF + 0x0FFF = 0x1FFE > 0x0FFF → true
    assert!(cpu.registers.f.half_carry);
    // Cフラグ: 0x1FFE < 0x10000 → false
    assert!(!cpu.registers.f.carry);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);
}

#[test]
fn ld_a_hlp() {
    let mut cpu = CPU::new();
    // HL = 0x1234, メモリ[0x1234] = 0xAB
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x1234, 0xAB);
    cpu.bus.write_byte(0x00, 0x2A); // LD A, (HL+)
    cpu.step();
    // Aに0xABがロードされていること
    assert_eq!(cpu.registers.a, 0xAB);
    // HLが+1されている
    assert_eq!(cpu.registers.get_hl(), 0x1235);
    // PCは1進む
    assert_eq!(cpu.pc, 0x01);

    // HLが0xFFFFの場合のラップアラウンドも確認
    cpu.registers.set_hl(0xFFFF);
    cpu.bus.write_byte(0xFFFF, 0x42);
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x2A); // LD A, (HL+)
    cpu.step();
    assert_eq!(cpu.registers.a, 0x42);
    assert_eq!(cpu.registers.get_hl(), 0x0000); // 0xFFFF + 1 = 0x0000
    assert_eq!(cpu.pc, 0x11);
}

#[test]
fn dec_hl() {
    let mut cpu = CPU::new();
    // 1. 通常のデクリメント
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x00, 0x2B); // DEC HL
    cpu.step();
    assert_eq!(cpu.registers.get_hl(), 0x1233);
    assert_eq!(cpu.pc, 0x01);
    // フラグは変化しない（全てfalseのまま）
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);

    // 2. アンダーフローのテスト
    cpu.registers.set_hl(0x0000);
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x2B); // DEC HL
    cpu.step();
    assert_eq!(cpu.registers.get_hl(), 0xFFFF);
    // フラグは変化しない
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn inc_l() {
    let mut cpu = CPU::new();
    // 1. 通常のインクリメント
    cpu.registers.l = 0x01;
    cpu.bus.write_byte(0x00, 0x2C); // INC L
    cpu.step();
    assert_eq!(cpu.registers.l, 0x02);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.subtract);

    // 2. ハーフキャリー
    cpu.pc = 0;
    cpu.registers.l = 0x0F;
    cpu.bus.write_byte(0x00, 0x2C); // INC L
    cpu.step();
    assert_eq!(cpu.registers.l, 0x10);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.l = 0xFF;
    cpu.bus.write_byte(0x00, 0x2C); // INC L
    cpu.step();
    assert_eq!(cpu.registers.l, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.subtract);
}

#[test]
fn dec_l() {
    let mut cpu = CPU::new();
    // 1. 通常のデクリメント
    cpu.registers.l = 0x02;
    cpu.bus.write_byte(0x00, 0x2D); // DEC L
    cpu.step();
    assert_eq!(cpu.registers.l, 0x01);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 2. ハーフキャリー発生（下位4ビットが0のとき）
    cpu.pc = 0;
    cpu.registers.l = 0x10;
    cpu.bus.write_byte(0x00, 0x2D); // DEC L
    cpu.step();
    assert_eq!(cpu.registers.l, 0x0F);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.l = 0x01;
    cpu.bus.write_byte(0x00, 0x2D); // DEC L
    cpu.step();
    assert_eq!(cpu.registers.l, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);

    // 4. アンダーフロー（0x00→0xFF）
    cpu.pc = 0;
    cpu.registers.l = 0x00;
    cpu.bus.write_byte(0x00, 0x2D); // DEC L
    cpu.step();
    assert_eq!(cpu.registers.l, 0xFF);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
}

#[test]
fn ld_l_d8() {
    let mut cpu = CPU::new();
    // LD L, 0x77 命令
    // 0x2E: LD L, d8 のオペコード
    // 0x77: 即値データ
    cpu.bus.write_byte(0x00, 0x2E); // LD L, d8
    cpu.bus.write_byte(0x01, 0x77); // d8 = 0x77

    cpu.step();

    assert_eq!(cpu.registers.l, 0x77);
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn cpl() {
    let mut cpu = CPU::new();
    // 1. 通常の反転
    cpu.registers.a = 0b1010_0101;
    cpu.registers.f.zero = false;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x2F); // CPL
    cpu.step();
    assert_eq!(cpu.registers.a, 0b0101_1010);
    // N/Hフラグがセット
    assert!(cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    // Z/Cフラグは変化しない
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.carry);
    assert_eq!(cpu.pc, 0x01);

    // 2. すべてのビットが1の場合
    cpu.pc = 0;
    cpu.registers.a = 0xFF;
    cpu.registers.f.zero = false;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x2F); // CPL
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    // N/Hフラグがセット
    assert!(cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    // Z/Cフラグは変化しない
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.carry);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn jr_nc_s8() {
    let mut cpu = CPU::new();
    // 1. キャリーフラグが立っていない場合（ジャンプする）
    cpu.pc = 0x1000;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x1000, 0x30); // JR NC, s8
    cpu.bus.write_byte(0x1001, 0x05); // +5
    cpu.step();
    assert_eq!(cpu.pc, 0x1000 + 2 + 5); // 0x1007

    // 2. キャリーフラグが立っている場合（ジャンプしない）
    let mut cpu = CPU::new();
    cpu.pc = 0x2000;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x2000, 0x30); // JR NC, s8
    cpu.bus.write_byte(0x2001, 0x05); // +5
    cpu.step();
    assert_eq!(cpu.pc, 0x2002); // 通常通り次命令へ

    // 3. 負のオフセット（-2）でジャンプ
    let mut cpu = CPU::new();
    cpu.pc = 0x3000;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x3000, 0x30); // JR NC, s8
    cpu.bus.write_byte(0x3001, 0xFE); // -2（0xFE as i8 = -2）
    cpu.step();
    assert_eq!(cpu.pc, 0x3000 + 2 - 2); // 0x3000
}

#[test]
fn ld_sp_d16() {
    let mut cpu = CPU::new();
    // LD SP, 0xBEEF 命令
    // 0x31: LD SP, d16 のオペコード
    // 0xEF: 下位バイト
    // 0xBE: 上位バイト
    cpu.bus.write_byte(0x00, 0x31); // LD SP, d16
    cpu.bus.write_byte(0x01, 0xEF); // LSB
    cpu.bus.write_byte(0x02, 0xBE); // MSB

    cpu.step();

    assert_eq!(cpu.sp, 0xBEEF);
    assert_eq!(cpu.pc, 0x03);
}

#[test]
fn ld_hlm_a() {
    let mut cpu = CPU::new();
    // HL = 0x1234, A = 0xAB
    cpu.registers.set_hl(0x1234);
    cpu.registers.a = 0xAB;
    cpu.bus.write_byte(0x00, 0x32); // LD (HL-), A
    cpu.step();
    // HLの指すアドレスにAの値が書き込まれている
    assert_eq!(cpu.bus.read_byte(0x1234), 0xAB);
    // HLが-1されている
    assert_eq!(cpu.registers.get_hl(), 0x1233);
    // PCは1進む
    assert_eq!(cpu.pc, 0x01);

    // HLが0x0000の場合のラップアラウンドも確認
    cpu.registers.set_hl(0x0000);
    cpu.registers.a = 0x42;
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x32); // LD (HL-), A
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x0000), 0x42);
    assert_eq!(cpu.registers.get_hl(), 0xFFFF); // 0x0000 - 1 = 0xFFFF
    assert_eq!(cpu.pc, 0x11);
}

#[test]
fn inc_sp() {
    let mut cpu = CPU::new();
    // 1. 通常のインクリメント
    cpu.sp = 0x1234;
    cpu.bus.write_byte(0x00, 0x33); // INC SP
    cpu.step();
    assert_eq!(cpu.sp, 0x1235);
    assert_eq!(cpu.pc, 0x01);
    // フラグは変化しない（全てfalseのまま）
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);

    // 2. アンダーフローのテスト
    cpu.sp = 0xFFFF;
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x33); // INC SP
    cpu.step();
    assert_eq!(cpu.sp, 0x0000);
    // フラグは変化しない
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn inc_hli() {
    let mut cpu = CPU::new();
    // 1. 通常のインクリメント
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x1234, 0x01);
    cpu.bus.write_byte(0x00, 0x34); // INC (HL)
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x1234), 0x02);
    assert_eq!(cpu.pc, 0x01);
    // HLレジスタ自体は変化しない
    assert_eq!(cpu.registers.get_hl(), 0x1234);
    // フラグ
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.subtract);

    // 2. ハーフキャリー
    cpu.pc = 0;
    cpu.bus.write_byte(0x1234, 0x0F);
    cpu.bus.write_byte(0x00, 0x34); // INC (HL)
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x1234), 0x10);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.bus.write_byte(0x1234, 0xFF);
    cpu.bus.write_byte(0x00, 0x34); // INC (HL)
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x1234), 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.half_carry);
}

#[test]
fn dec_hli() {
    let mut cpu = CPU::new();
    // 1. 通常のデクリメント
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x1234, 0x02);
    cpu.bus.write_byte(0x00, 0x35); // DEC (HL)
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x1234), 0x01);
    assert_eq!(cpu.pc, 0x01);
    // HLレジスタ自体は変化しない
    assert_eq!(cpu.registers.get_hl(), 0x1234);
    // フラグ
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 2. ハーフキャリー発生（下位4ビットが0のとき）
    cpu.pc = 0;
    cpu.bus.write_byte(0x1234, 0x10);
    cpu.bus.write_byte(0x00, 0x35); // DEC (HL)
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x1234), 0x0F);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.bus.write_byte(0x1234, 0x01);
    cpu.bus.write_byte(0x00, 0x35); // DEC (HL)
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x1234), 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);

    // 4. アンダーフロー（0x00→0xFF）
    cpu.pc = 0;
    cpu.bus.write_byte(0x1234, 0x00);
    cpu.bus.write_byte(0x00, 0x35); // DEC (HL)
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x1234), 0xFF);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
}

#[test]
fn ld_hli_d8() {
    let mut cpu = CPU::new();
    // HL = 0x1234, d8 = 0xAB
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x00, 0x36); // LD (HL), d8
    cpu.bus.write_byte(0x01, 0xAB); // d8 = 0xAB
    cpu.step();
    // HLの指すアドレスに即値が書き込まれている
    assert_eq!(cpu.bus.read_byte(0x1234), 0xAB);
    // PCは2進む
    assert_eq!(cpu.pc, 0x02);
}

#[test]
fn scf() {
    let mut cpu = CPU::new();
    // 1. Cフラグが0のとき
    cpu.registers.f.carry = false;
    cpu.registers.f.zero = true; // Zフラグは変化しない
    cpu.registers.f.subtract = true; // Nフラグはクリアされる
    cpu.registers.f.half_carry = true; // Hフラグはクリアされる
    cpu.bus.write_byte(0x00, 0x37); // SCF
    cpu.step();
    // Cフラグがセットされる
    assert!(cpu.registers.f.carry);
    // N/Hフラグはクリア
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    // Zフラグは変化しない
    assert!(cpu.registers.f.zero);
    assert_eq!(cpu.pc, 0x01);

    // 2. すでにCフラグが1でも再度セットされるだけ
    cpu.pc = 0;
    cpu.registers.f.carry = true;
    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = true;
    cpu.bus.write_byte(0x00, 0x37); // SCF
    cpu.step();
    assert!(cpu.registers.f.carry);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.zero);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn jr_c_s8() {
    let mut cpu = CPU::new();
    // 1. キャリーフラグが立っている場合（ジャンプする）
    cpu.pc = 0x4000;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x4000, 0x38); // JR C, s8
    cpu.bus.write_byte(0x4001, 0x06); // +6
    cpu.step();
    assert_eq!(cpu.pc, 0x4000 + 2 + 6); // 0x4008

    // 2. キャリーフラグが立っていない場合（ジャンプしない）
    let mut cpu = CPU::new();
    cpu.pc = 0x5000;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x5000, 0x38); // JR C, s8
    cpu.bus.write_byte(0x5001, 0x06); // +6
    cpu.step();
    assert_eq!(cpu.pc, 0x5002); // 通常通り次命令へ

    // 3. 負のオフセット（-4）でジャンプ
    let mut cpu = CPU::new();
    cpu.pc = 0x6000;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x6000, 0x38); // JR C, s8
    cpu.bus.write_byte(0x6001, 0xFC); // -4（0xFC as i8 = -4）
    cpu.step();
    assert_eq!(cpu.pc, 0x6000 + 2 - 4); // 0x5FFE
}

#[test]
fn add_hl_sp() {
    let mut cpu = CPU::new();
    // 1. 通常の加算
    cpu.registers.set_hl(0x1111);
    cpu.sp = 0x2222;
    cpu.bus.write_byte(0x00, 0x39); // ADD HL, SP
    cpu.step();
    assert_eq!(cpu.registers.get_hl(), 0x3333);
    assert_eq!(cpu.pc, 0x01);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);
    // Hフラグ: (0x1111 & 0xFFF) + (0x2222 & 0xFFF) = 0x1111 + 0x2222 = 0x3333 < 0xFFF → false
    assert!(!cpu.registers.f.half_carry);
    // Cフラグ: 0x1111 + 0x2222 = 0x3333 < 0x10000 → false
    assert!(!cpu.registers.f.carry);

    // 2. キャリー発生のケース
    cpu.registers.set_hl(0xFFFF);
    cpu.sp = 0x0001;
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x39); // ADD HL, SP
    cpu.step();
    // 0xFFFF + 0x0001 = 0x0000
    assert_eq!(cpu.registers.get_hl(), 0x0000);
    // Cフラグ: キャリー発生
    assert!(cpu.registers.f.carry);
    // Hフラグ: (0xFFF + 0x001) = 0x1000 > 0xFFF → true
    assert!(cpu.registers.f.half_carry);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);

    // 3. ハーフキャリーのみ発生のケース
    cpu.registers.set_hl(0x0FFF);
    cpu.sp = 0x0001;
    cpu.pc = 0x20;
    cpu.bus.write_byte(0x20, 0x39); // ADD HL, SP
    cpu.step();
    // 0x0FFF + 0x0001 = 0x1000
    assert_eq!(cpu.registers.get_hl(), 0x1000);
    // Hフラグ: 0x0FFF + 0x0001 = 0x1000 > 0x0FFF → true
    assert!(cpu.registers.f.half_carry);
    // Cフラグ: 0x1000 < 0x10000 → false
    assert!(!cpu.registers.f.carry);
    // Nフラグはクリア
    assert!(!cpu.registers.f.subtract);
}

#[test]
fn ld_a_hlm() {
    let mut cpu = CPU::new();
    // 1. 通常のロードとHLデクリメント
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x1234, 0xAB);
    cpu.bus.write_byte(0x00, 0x3A); // LD A, (HL-)
    cpu.step();
    // Aに0xABがロードされていること
    assert_eq!(cpu.registers.a, 0xAB);
    // HLが-1されている
    assert_eq!(cpu.registers.get_hl(), 0x1233);
    // PCは1進む
    assert_eq!(cpu.pc, 0x01);

    // 2. HLが0x0000の場合のラップアラウンド
    cpu.registers.set_hl(0x0000);
    cpu.bus.write_byte(0x0000, 0x42);
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x3A); // LD A, (HL-)
    cpu.step();
    assert_eq!(cpu.registers.a, 0x42);
    assert_eq!(cpu.registers.get_hl(), 0xFFFF); // 0x0000 - 1 = 0xFFFF
    assert_eq!(cpu.pc, 0x11);
}

#[test]
fn dec_sp() {
    let mut cpu = CPU::new();
    // 1. 通常のデクリメント
    cpu.sp = 0x1234;
    cpu.bus.write_byte(0x00, 0x3B); // DEC SP
    cpu.step();
    assert_eq!(cpu.sp, 0x1233);
    assert_eq!(cpu.pc, 0x01);
    // フラグは変化しない（全てfalseのまま）
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);

    // 2. アンダーフローのテスト
    cpu.sp = 0x0000;
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x3B); // DEC SP
    cpu.step();
    assert_eq!(cpu.sp, 0xFFFF);
    assert_eq!(cpu.pc, 0x11);
    // フラグは変化しない
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn inc_a() {
    let mut cpu = CPU::new();
    // 1. 通常のインクリメント
    cpu.registers.a = 0x01;
    cpu.bus.write_byte(0x00, 0x3C); // INC A
    cpu.step();
    assert_eq!(cpu.registers.a, 0x02);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.subtract);

    // 2. ハーフキャリー
    cpu.pc = 0;
    cpu.registers.a = 0x0F;
    cpu.bus.write_byte(0x00, 0x3C); // INC A
    cpu.step();
    assert_eq!(cpu.registers.a, 0x10);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.a = 0xFF;
    cpu.bus.write_byte(0x00, 0x3C); // INC A
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.subtract);
}

#[test]
fn dec_a() {
    let mut cpu = CPU::new();
    // 1. 通常のデクリメント
    cpu.registers.a = 0x02;
    cpu.bus.write_byte(0x00, 0x3D); // DEC A
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.pc, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 2. ハーフキャリー発生（下位4ビットが0のとき）
    cpu.pc = 0;
    cpu.registers.a = 0x10;
    cpu.bus.write_byte(0x00, 0x3D); // DEC A
    cpu.step();
    assert_eq!(cpu.registers.a, 0x0F);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.a = 0x01;
    cpu.bus.write_byte(0x00, 0x3D); // DEC A
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);

    // 4. アンダーフロー（0x00→0xFF）
    cpu.pc = 0;
    cpu.registers.a = 0x00;
    cpu.bus.write_byte(0x00, 0x3D); // DEC A
    cpu.step();
    assert_eq!(cpu.registers.a, 0xFF);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
}

#[test]
fn ld_a_d8() {
    let mut cpu = CPU::new();
    // LD A, 0x42 命令
    cpu.bus.write_byte(0x00, 0x3E); // LD A, d8
    cpu.bus.write_byte(0x01, 0x42); // d8 = 0x42
    cpu.step();
    assert_eq!(cpu.registers.a, 0x42);
    assert_eq!(cpu.pc, 0x02);

    // 2. 0x00のロード
    cpu.pc = 0x10;
    cpu.bus.write_byte(0x10, 0x3E); // LD A, d8
    cpu.bus.write_byte(0x11, 0x00); // d8 = 0x00
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.pc, 0x12);
}

#[test]
fn ccf() {
    let mut cpu = CPU::new();
    // 1. Cフラグが0のとき
    cpu.registers.f.carry = false;
    cpu.registers.f.zero = true; // Zフラグは変化しない
    cpu.registers.f.subtract = true; // Nフラグはクリアされる
    cpu.registers.f.half_carry = true; // Hフラグはクリアされる
    cpu.bus.write_byte(0x00, 0x3F); // CCF
    cpu.step();
    // Cフラグが反転して1になる
    assert!(cpu.registers.f.carry);
    // N/Hフラグはクリア
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    // Zフラグは変化しない
    assert!(cpu.registers.f.zero);
    assert_eq!(cpu.pc, 0x01);

    // 2. Cフラグが1のとき
    cpu.pc = 0;
    cpu.registers.f.carry = true;
    cpu.registers.f.zero = false;
    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = true;
    cpu.bus.write_byte(0x00, 0x3F); // CCF
    cpu.step();
    assert!(!cpu.registers.f.carry);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.zero);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn ld_b_group() {
    let mut cpu = CPU::new();
    // LD B, B
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x40); // LD B, B
    cpu.step();
    assert_eq!(cpu.registers.b, 0x12);
    // LD B, C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x41); // LD B, C
    cpu.pc = 0x01;
    cpu.step();
    assert_eq!(cpu.registers.b, 0x34);
    // LD B, D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x42); // LD B, D
    cpu.pc = 0x02;
    cpu.step();
    assert_eq!(cpu.registers.b, 0x56);
    // LD B, E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x43); // LD B, E
    cpu.pc = 0x03;
    cpu.step();
    assert_eq!(cpu.registers.b, 0x78);
    // LD B, H
    cpu.registers.h = 0x9A;
    cpu.bus.write_byte(0x04, 0x44); // LD B, H
    cpu.pc = 0x04;
    cpu.step();
    assert_eq!(cpu.registers.b, 0x9A);
    // LD B, L
    cpu.registers.l = 0xBC;
    cpu.bus.write_byte(0x05, 0x45); // LD B, L
    cpu.pc = 0x05;
    cpu.step();
    assert_eq!(cpu.registers.b, 0xBC);
    // LD B, (HL)
    cpu.registers.set_hl(0x2000);
    cpu.bus.write_byte(0x2000, 0xDE);
    cpu.bus.write_byte(0x06, 0x46); // LD B, (HL)
    cpu.pc = 0x06;
    cpu.step();
    assert_eq!(cpu.registers.b, 0xDE);
    // LD B, A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x07, 0x47); // LD B, A
    cpu.pc = 0x07;
    cpu.step();
    assert_eq!(cpu.registers.b, 0xF0);
    // LD (HL), B
    cpu.registers.set_hl(0x3000);
    cpu.registers.b = 0x55;
    cpu.bus.write_byte(0x08, 0x70); // LD (HL), B
    cpu.pc = 0x08;
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x3000), 0x55);
    // LD A, B
    cpu.registers.b = 0x77;
    cpu.bus.write_byte(0x09, 0x78); // LD A, B
    cpu.pc = 0x09;
    cpu.step();
    assert_eq!(cpu.registers.a, 0x77);
}

#[test]
fn ld_c_group() {
    let mut cpu = CPU::new();
    // LD C, B
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x48); // LD C, B
    cpu.step();
    assert_eq!(cpu.registers.c, 0x12);
    // LD C, C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x49); // LD C, C
    cpu.pc = 0x01;
    cpu.step();
    assert_eq!(cpu.registers.c, 0x34);
    // LD C, D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x4A); // LD C, D
    cpu.pc = 0x02;
    cpu.step();
    assert_eq!(cpu.registers.c, 0x56);
    // LD C, E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x4B); // LD C, E
    cpu.pc = 0x03;
    cpu.step();
    assert_eq!(cpu.registers.c, 0x78);
    // LD C, H
    cpu.registers.h = 0x9A;
    cpu.bus.write_byte(0x04, 0x4C); // LD C, H
    cpu.pc = 0x04;
    cpu.step();
    assert_eq!(cpu.registers.c, 0x9A);
    // LD C, L
    cpu.registers.l = 0xBC;
    cpu.bus.write_byte(0x05, 0x4D); // LD C, L
    cpu.pc = 0x05;
    cpu.step();
    assert_eq!(cpu.registers.c, 0xBC);
    // LD C, (HL)
    cpu.registers.set_hl(0x2100);
    cpu.bus.write_byte(0x2100, 0xDE);
    cpu.bus.write_byte(0x06, 0x4E); // LD C, (HL)
    cpu.pc = 0x06;
    cpu.step();
    assert_eq!(cpu.registers.c, 0xDE);
    // LD C, A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x07, 0x4F); // LD C, A
    cpu.pc = 0x07;
    cpu.step();
    assert_eq!(cpu.registers.c, 0xF0);
    // LD (HL), C
    cpu.registers.set_hl(0x3100);
    cpu.registers.c = 0x55;
    cpu.bus.write_byte(0x08, 0x71); // LD (HL), C
    cpu.pc = 0x08;
    cpu.step();
    assert_eq!(cpu.bus.read_byte(0x3100), 0x55);
    // LD A, C
    cpu.registers.c = 0x77;
    cpu.bus.write_byte(0x09, 0x79); // LD A, C
    cpu.pc = 0x09;
    cpu.step();
    assert_eq!(cpu.registers.a, 0x77);
}

#[test]
fn ld_d_group() {
    let mut cpu = CPU::new();
    // LD D, B
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x50); cpu.step(); assert_eq!(cpu.registers.d, 0x12);
    // LD D, C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x51); cpu.pc = 0x01; cpu.step(); assert_eq!(cpu.registers.d, 0x34);
    // LD D, D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x52); cpu.pc = 0x02; cpu.step(); assert_eq!(cpu.registers.d, 0x56);
    // LD D, E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x53); cpu.pc = 0x03; cpu.step(); assert_eq!(cpu.registers.d, 0x78);
    // LD D, H
    cpu.registers.h = 0x9A;
    cpu.bus.write_byte(0x04, 0x54); cpu.pc = 0x04; cpu.step(); assert_eq!(cpu.registers.d, 0x9A);
    // LD D, L
    cpu.registers.l = 0xBC;
    cpu.bus.write_byte(0x05, 0x55); cpu.pc = 0x05; cpu.step(); assert_eq!(cpu.registers.d, 0xBC);
    // LD D, (HL)
    cpu.registers.set_hl(0x2200);
    cpu.bus.write_byte(0x2200, 0xDE);
    cpu.bus.write_byte(0x06, 0x56); cpu.pc = 0x06; cpu.step(); assert_eq!(cpu.registers.d, 0xDE);
    // LD D, A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x07, 0x57); cpu.pc = 0x07; cpu.step(); assert_eq!(cpu.registers.d, 0xF0);
    // LD (HL), D
    cpu.registers.set_hl(0x3200);
    cpu.registers.d = 0x55;
    cpu.bus.write_byte(0x08, 0x72); cpu.pc = 0x08; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3200), 0x55);
    // LD A, D
    cpu.registers.d = 0x77;
    cpu.bus.write_byte(0x09, 0x7A); cpu.pc = 0x09; cpu.step(); assert_eq!(cpu.registers.a, 0x77);
}

#[test]
fn ld_e_group() {
    let mut cpu = CPU::new();
    // LD E, B
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x58); cpu.step(); assert_eq!(cpu.registers.e, 0x12);
    // LD E, C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x59); cpu.pc = 0x01; cpu.step(); assert_eq!(cpu.registers.e, 0x34);
    // LD E, D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x5A); cpu.pc = 0x02; cpu.step(); assert_eq!(cpu.registers.e, 0x56);
    // LD E, E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x5B); cpu.pc = 0x03; cpu.step(); assert_eq!(cpu.registers.e, 0x78);
    // LD E, H
    cpu.registers.h = 0x9A;
    cpu.bus.write_byte(0x04, 0x5C); cpu.pc = 0x04; cpu.step(); assert_eq!(cpu.registers.e, 0x9A);
    // LD E, L
    cpu.registers.l = 0xBC;
    cpu.bus.write_byte(0x05, 0x5D); cpu.pc = 0x05; cpu.step(); assert_eq!(cpu.registers.e, 0xBC);
    // LD E, (HL)
    cpu.registers.set_hl(0x2300);
    cpu.bus.write_byte(0x2300, 0xDE);
    cpu.bus.write_byte(0x06, 0x5E); cpu.pc = 0x06; cpu.step(); assert_eq!(cpu.registers.e, 0xDE);
    // LD E, A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x07, 0x5F); cpu.pc = 0x07; cpu.step(); assert_eq!(cpu.registers.e, 0xF0);
    // LD (HL), E
    cpu.registers.set_hl(0x3300);
    cpu.registers.e = 0x55;
    cpu.bus.write_byte(0x08, 0x73); cpu.pc = 0x08; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3300), 0x55);
    // LD A, E
    cpu.registers.e = 0x77;
    cpu.bus.write_byte(0x09, 0x7B); cpu.pc = 0x09; cpu.step(); assert_eq!(cpu.registers.a, 0x77);
}

#[test]
fn ld_h_group() {
    let mut cpu = CPU::new();
    // LD H, B
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x60); cpu.step(); assert_eq!(cpu.registers.h, 0x12);
    // LD H, C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x61); cpu.pc = 0x01; cpu.step(); assert_eq!(cpu.registers.h, 0x34);
    // LD H, D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x62); cpu.pc = 0x02; cpu.step(); assert_eq!(cpu.registers.h, 0x56);
    // LD H, E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x63); cpu.pc = 0x03; cpu.step(); assert_eq!(cpu.registers.h, 0x78);
    // LD H, H
    cpu.registers.h = 0x9A;
    cpu.bus.write_byte(0x04, 0x64); cpu.pc = 0x04; cpu.step(); assert_eq!(cpu.registers.h, 0x9A);
    // LD H, L
    cpu.registers.l = 0xBC;
    cpu.bus.write_byte(0x05, 0x65); cpu.pc = 0x05; cpu.step(); assert_eq!(cpu.registers.h, 0xBC);
    // LD H, (HL)
    cpu.registers.set_hl(0x2400);
    cpu.bus.write_byte(0x2400, 0xDE);
    cpu.bus.write_byte(0x06, 0x66); cpu.pc = 0x06; cpu.step(); assert_eq!(cpu.registers.h, 0xDE);
    // LD H, A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x07, 0x67); cpu.pc = 0x07; cpu.step(); assert_eq!(cpu.registers.h, 0xF0);
    // LD (HL), H
    cpu.registers.set_hl(0x3400); // H自体の値を入れるため注意
    cpu.bus.write_byte(0x08, 0x74); cpu.pc = 0x08; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3400), 0x34);
    // LD A, H
    cpu.registers.h = 0x77;
    cpu.bus.write_byte(0x09, 0x7C); cpu.pc = 0x09; cpu.step(); assert_eq!(cpu.registers.a, 0x77);
}

#[test]
fn ld_l_group() {
    let mut cpu = CPU::new();
    // LD L, B
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x68); cpu.step(); assert_eq!(cpu.registers.l, 0x12);
    // LD L, C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x69); cpu.pc = 0x01; cpu.step(); assert_eq!(cpu.registers.l, 0x34);
    // LD L, D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x6A); cpu.pc = 0x02; cpu.step(); assert_eq!(cpu.registers.l, 0x56);
    // LD L, E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x6B); cpu.pc = 0x03; cpu.step(); assert_eq!(cpu.registers.l, 0x78);
    // LD L, H
    cpu.registers.h = 0x9A;
    cpu.bus.write_byte(0x04, 0x6C); cpu.pc = 0x04; cpu.step(); assert_eq!(cpu.registers.l, 0x9A);
    // LD L, L
    cpu.registers.l = 0xBC;
    cpu.bus.write_byte(0x05, 0x6D); cpu.pc = 0x05; cpu.step(); assert_eq!(cpu.registers.l, 0xBC);
    // LD L, (HL)
    cpu.registers.set_hl(0x2500);
    cpu.bus.write_byte(0x2500, 0xDE);
    cpu.bus.write_byte(0x06, 0x6E); cpu.pc = 0x06; cpu.step(); assert_eq!(cpu.registers.l, 0xDE);
    // LD L, A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x07, 0x6F); cpu.pc = 0x07; cpu.step(); assert_eq!(cpu.registers.l, 0xF0);
    // LD (HL), L
    cpu.registers.set_hl(0x3555); // HL=0x3555　（HL自身の値を変えようとしてる）
    cpu.bus.write_byte(0x08, 0x75); cpu.pc = 0x08; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3555), 0x55);
    // LD A, L
    cpu.registers.l = 0x77;
    cpu.bus.write_byte(0x09, 0x7D); cpu.pc = 0x09; cpu.step(); assert_eq!(cpu.registers.a, 0x77);
}

#[test]
fn ld_hli_group() {
    let mut cpu = CPU::new();
    // LD (HL), B
    cpu.registers.set_hl(0x3600);
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x70); cpu.step(); assert_eq!(cpu.bus.read_byte(0x3600), 0x12);
    // LD (HL), C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x71); cpu.pc = 0x01; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3600), 0x34);
    // LD (HL), D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x72); cpu.pc = 0x02; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3600), 0x56);
    // LD (HL), E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x73); cpu.pc = 0x03; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3600), 0x78);
    // LD (HL), H
    // H自体の値を入れるので注意
    cpu.bus.write_byte(0x04, 0x74); cpu.pc = 0x04; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3600), 0x36);
    // LD (HL), L
    // L自体の値を入れるので注意
    cpu.bus.write_byte(0x05, 0x75); cpu.pc = 0x05; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3600), 0x00);
    // LD (HL), A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x06, 0x77); cpu.pc = 0x06; cpu.step(); assert_eq!(cpu.bus.read_byte(0x3600), 0xF0);
}

#[test]
fn ld_a_group() {
    let mut cpu = CPU::new();
    // LD A, B
    cpu.registers.b = 0x12;
    cpu.bus.write_byte(0x00, 0x78); cpu.step(); assert_eq!(cpu.registers.a, 0x12);
    // LD A, C
    cpu.registers.c = 0x34;
    cpu.bus.write_byte(0x01, 0x79); cpu.pc = 0x01; cpu.step(); assert_eq!(cpu.registers.a, 0x34);
    // LD A, D
    cpu.registers.d = 0x56;
    cpu.bus.write_byte(0x02, 0x7A); cpu.pc = 0x02; cpu.step(); assert_eq!(cpu.registers.a, 0x56);
    // LD A, E
    cpu.registers.e = 0x78;
    cpu.bus.write_byte(0x03, 0x7B); cpu.pc = 0x03; cpu.step(); assert_eq!(cpu.registers.a, 0x78);
    // LD A, H
    cpu.registers.h = 0x9A;
    cpu.bus.write_byte(0x04, 0x7C); cpu.pc = 0x04; cpu.step(); assert_eq!(cpu.registers.a, 0x9A);
    // LD A, L
    cpu.registers.l = 0xBC;
    cpu.bus.write_byte(0x05, 0x7D); cpu.pc = 0x05; cpu.step(); assert_eq!(cpu.registers.a, 0xBC);
    // LD A, (HL)
    cpu.registers.set_hl(0x3700);
    cpu.bus.write_byte(0x3700, 0xDE);
    cpu.bus.write_byte(0x06, 0x7E); cpu.pc = 0x06; cpu.step(); assert_eq!(cpu.registers.a, 0xDE);
    // LD A, A
    cpu.registers.a = 0xF0;
    cpu.bus.write_byte(0x07, 0x7F); cpu.pc = 0x07; cpu.step(); assert_eq!(cpu.registers.a, 0xF0);
}
#[test]
fn add_a_b() {
  let mut cpu = CPU::new();
  cpu.registers.a = 1;
  cpu.registers.b = 2;
  cpu.bus.write_byte(0x00, 0x80); // ADD A, B
  cpu.step();
  assert_eq!(cpu.registers.a, 3);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ハーフキャリー
  cpu.pc = 0;
  cpu.registers.a = 0x0F;
  cpu.registers.b = 0x01;
  cpu.bus.write_byte(0x00, 0x80);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x10);
  assert!(cpu.registers.f.half_carry);

  // キャリー
  cpu.pc = 0;
  cpu.registers.a = 0xFF;
  cpu.registers.b = 0x01;
  cpu.bus.write_byte(0x00, 0x80);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x00);
  assert!(cpu.registers.f.zero);
  assert!(cpu.registers.f.carry);
}

#[test]
fn add_a_c() {
  let mut cpu = CPU::new();
  cpu.registers.a = 1;
  cpu.registers.c = 3;
  cpu.bus.write_byte(0x00, 0x81); // ADD A, C
  cpu.step();
  assert_eq!(cpu.registers.a, 4);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn add_a_d() {
  let mut cpu = CPU::new();
  cpu.registers.a = 1;
  cpu.registers.d = 4;
  cpu.bus.write_byte(0x00, 0x82); // ADD A, D
  cpu.step();
  assert_eq!(cpu.registers.a, 5);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn add_a_e() {
  let mut cpu = CPU::new();
  cpu.registers.a = 1;
  cpu.registers.e = 5;
  cpu.bus.write_byte(0x00, 0x83); // ADD A, E
  cpu.step();
  assert_eq!(cpu.registers.a, 6);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn add_a_h() {
  let mut cpu = CPU::new();
  cpu.registers.a = 1;
  cpu.registers.h = 6;
  cpu.bus.write_byte(0x00, 0x84); // ADD A, H
  cpu.step();
  assert_eq!(cpu.registers.a, 7);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn add_a_l() {
  let mut cpu = CPU::new();
  cpu.registers.a = 1;
  cpu.registers.l = 7;
  cpu.bus.write_byte(0x00, 0x85); // ADD A, L
  cpu.step();
  assert_eq!(cpu.registers.a, 8);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn add_a_hli() {
  let mut cpu = CPU::new();
  cpu.registers.a = 1;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 8);
  cpu.bus.write_byte(0x00, 0x86); // ADD A, (HL)
  cpu.step();
  assert_eq!(cpu.registers.a, 9);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn add_a_a() {
  let mut cpu = CPU::new();
  cpu.registers.a = 5;
  cpu.bus.write_byte(0x00, 0x87); // ADD A, A
  cpu.step();
  assert_eq!(cpu.registers.a, 10);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.bus.write_byte(0x00, 0x87);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn adc_a_b() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x0F;
    cpu.registers.b = 0x01;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x88);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x11); // 0x0F + 0x01 + 1
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry); // 0x0F + 1 + 1 = 0x11, half carry
    assert!(!cpu.registers.f.carry);
}

#[test]
fn adc_a_c() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0xFF;
    cpu.registers.c = 0x01;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x89);
    cpu.step();
    assert_eq!(cpu.registers.a, 1); // 0xFF + 0x01 + 1 = 0x101
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry); // 0x0F + 1 + 1 = 0x11, half carry
    assert!(cpu.registers.f.carry);
}

#[test]
fn adc_a_d() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x00;
    cpu.registers.d = 0x00;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x8A);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn adc_a_e() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x8F;
    cpu.registers.e = 0x0F;
    cpu.registers.f.carry = false;
    cpu.bus.write_byte(0x00, 0x8B);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x9E);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn adc_a_h() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0xFF;
    cpu.registers.h = 0x00;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x8C);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.carry);
}

#[test]
fn adc_a_l() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x7F;
    cpu.registers.l = 0x00;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x8D);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x80);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn adc_a_hl() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x0F;
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x1234, 0x01);
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x8E);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x11);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn adc_a_a() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x80;
    cpu.registers.f.carry = true;
    cpu.bus.write_byte(0x00, 0x8F);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x01);
    assert!(!cpu.registers.f.zero);
    assert!(!cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(cpu.registers.f.carry);
}

#[test]
fn sub_a_b() {
    let mut cpu = CPU::new();
    cpu.registers.a = 5;
    cpu.registers.b = 2;
    cpu.bus.write_byte(0x00, 0x90);
    cpu.step();
    assert_eq!(cpu.registers.a, 3);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn sub_a_c() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x10;
    cpu.registers.c = 0x01;
    cpu.bus.write_byte(0x00, 0x91);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x0F);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry); // 下位4bitで借り
    assert!(!cpu.registers.f.carry);
}

#[test]
fn sub_a_d() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x01;
    cpu.registers.d = 0x01;
    cpu.bus.write_byte(0x00, 0x92);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn sub_a_e() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x00;
    cpu.registers.e = 0x01;
    cpu.bus.write_byte(0x00, 0x93);
    cpu.step();
    assert_eq!(cpu.registers.a, 0xFF);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.carry);
}

#[test]
fn sub_a_h() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x80;
    cpu.registers.h = 0x10;
    cpu.bus.write_byte(0x00, 0x94);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x70);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn sub_a_l() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x10;
    cpu.registers.l = 0x11;
    cpu.bus.write_byte(0x00, 0x95);
    cpu.step();
    assert_eq!(cpu.registers.a, 0xFF);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.carry);
}

#[test]
fn sub_a_hl() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x22;
    cpu.registers.set_hl(0x1234);
    cpu.bus.write_byte(0x1234, 0x02);
    cpu.bus.write_byte(0x00, 0x96);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x20);
    assert!(!cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn sub_a_a() {
    let mut cpu = CPU::new();
    cpu.registers.a = 0x55;
    cpu.bus.write_byte(0x00, 0x97);
    cpu.step();
    assert_eq!(cpu.registers.a, 0x00);
    assert!(cpu.registers.f.zero);
    assert!(cpu.registers.f.subtract);
    assert!(!cpu.registers.f.half_carry);
    assert!(!cpu.registers.f.carry);
}

#[test]
fn sbc_a_b() {
  let mut cpu = CPU::new();
  // 通常ケース
  cpu.registers.a = 5;
  cpu.registers.b = 2;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x98); // SBC A, B
  cpu.step();
  assert_eq!(cpu.registers.a, 3);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 5;
  cpu.registers.b = 2;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x98);
  cpu.step();
  assert_eq!(cpu.registers.a, 2);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 1;
  cpu.registers.b = 0;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x98);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // ハーフキャリー
  cpu.pc = 0;
  cpu.registers.a = 0x10;
  cpu.registers.b = 0x01;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x98);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x0E);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.b = 0x01;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x98);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFE);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);
}

#[test]
fn sbc_a_c() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.c = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x99);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x0F);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 0x10;
  cpu.registers.c = 0x01;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x99);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x0E);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.c = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x99);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x00);
  assert!(cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.c = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x99);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFF);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);
}

#[test]
fn sbc_a_d() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x20;
  cpu.registers.d = 0x10;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9A);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x10);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 0x20;
  cpu.registers.d = 0x10;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x9A);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x0F);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.d = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9A);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x00);
  assert!(cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.d = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9A);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFF);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);
}

#[test]
fn sbc_a_e() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.e = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9B);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x0F);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 0x10;
  cpu.registers.e = 0x01;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x9B);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x0E);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.e = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9B);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x00);
  assert!(cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.e = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9B);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFF);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);
}

#[test]
fn sbc_a_h() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x80;
  cpu.registers.h = 0x10;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9C);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x70);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 0x80;
  cpu.registers.h = 0x10;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x9C);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x6F);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.h = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9C);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFF);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);
}

#[test]
fn sbc_a_l() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.l = 0x11;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9D);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFF);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 0x10;
  cpu.registers.l = 0x11;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x9D);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFE);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.l = 0x01;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9D);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x00);
  assert!(cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);
}

#[test]
fn sbc_a_hl() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x22;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 0x02);
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9E);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x20);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 0x22;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 0x02);
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x9E);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x1F);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーフラグ
  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 0x01);
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9E);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFF);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);
}

#[test]
fn sbc_a_a() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x55;
  cpu.registers.f.carry = false;
  cpu.bus.write_byte(0x00, 0x9F);
  cpu.step();
  assert_eq!(cpu.registers.a, 0x00);
  assert!(cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // キャリーあり
  cpu.pc = 0;
  cpu.registers.a = 0x55;
  cpu.registers.f.carry = true;
  cpu.bus.write_byte(0x00, 0x9F);
  cpu.step();
  assert_eq!(cpu.registers.a, 0xFF);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(cpu.registers.f.half_carry);
  assert!(cpu.registers.f.carry);
}

#[test]
fn and_a_b() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1100_1100;
  cpu.registers.b = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xA0); // AND B
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1000_1000);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry); // ANDは常にH=1
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b0000_0001;
  cpu.registers.b = 0b0000_0000;
  cpu.bus.write_byte(0x00, 0xA0);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn and_a_c() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_0000;
  cpu.registers.c = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xA1); // AND C
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1010_0000);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn and_a_d() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_1111;
  cpu.registers.d = 0b0000_1111;
  cpu.bus.write_byte(0x00, 0xA2); // AND D
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0000_1111);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn and_a_e() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_1010;
  cpu.registers.e = 0b0101_0101;
  cpu.bus.write_byte(0x00, 0xA3); // AND E
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0000_0000);
  assert!(cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn and_a_h() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_0000;
  cpu.registers.h = 0b0000_1111;
  cpu.bus.write_byte(0x00, 0xA4); // AND H
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0000_0000);
  assert!(cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn and_a_l() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_1111;
  cpu.registers.l = 0b1111_0000;
  cpu.bus.write_byte(0x00, 0xA5); // AND L
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1111_0000);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn and_a_hli() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_1010;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 0b1111_0000);
  cpu.bus.write_byte(0x00, 0xA6); // AND (HL)
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1010_0000);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b0000_0001;
  cpu.bus.write_byte(0x1234, 0b0000_0000);
  cpu.bus.write_byte(0x00, 0xA6);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn and_a_a() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xA7); // AND A
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1010_1010);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.bus.write_byte(0x00, 0xA7);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn xor_a_b() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1100_1100;
  cpu.registers.b = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xA8); // XOR B
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0110_0110);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
  assert!(!cpu.registers.f.zero);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b1010_1010;
  cpu.registers.b = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xA8);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);
}

#[test]
fn xor_a_c() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_0000;
  cpu.registers.c = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xA9); // XOR C
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0101_1010);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b1010_1010;
  cpu.registers.c = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xA9);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn xor_a_d() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_1111;
  cpu.registers.d = 0b0000_1111;
  cpu.bus.write_byte(0x00, 0xAA); // XOR D
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1111_0000);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b0000_1111;
  cpu.registers.d = 0b0000_1111;
  cpu.bus.write_byte(0x00, 0xAA);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn xor_a_e() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_1010;
  cpu.registers.e = 0b0101_0101;
  cpu.bus.write_byte(0x00, 0xAB); // XOR E
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1111_1111);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b0101_0101;
  cpu.registers.e = 0b0101_0101;
  cpu.bus.write_byte(0x00, 0xAB);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn xor_a_h() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_0000;
  cpu.registers.h = 0b0000_1111;
  cpu.bus.write_byte(0x00, 0xAC); // XOR H
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1111_1111);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b0000_1111;
  cpu.registers.h = 0b0000_1111;
  cpu.bus.write_byte(0x00, 0xAC);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn xor_a_l() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_1111;
  cpu.registers.l = 0b1111_0000;
  cpu.bus.write_byte(0x00, 0xAD); // XOR L
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0000_1111);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b1111_0000;
  cpu.registers.l = 0b1111_0000;
  cpu.bus.write_byte(0x00, 0xAD);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn xor_a_hli() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_1010;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 0b1111_0000);
  cpu.bus.write_byte(0x00, 0xAE); // XOR (HL)
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0101_1010);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0b1010_1010;
  cpu.bus.write_byte(0x1234, 0b1010_1010);
  cpu.bus.write_byte(0x00, 0xAE);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn xor_a_a() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xAF); // XOR A
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // 0とのXOR
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.bus.write_byte(0x00, 0xAF);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn or_a_b() {
  let mut cpu = CPU::new();
  // 通常
  cpu.registers.a = 0b1100_1100;
  cpu.registers.b = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xB0); // OR B
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1110_1110);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.registers.b = 0;
  cpu.bus.write_byte(0x00, 0xB0);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);

  // 片方が0
  cpu.pc = 0;
  cpu.registers.a = 0b0000_0000;
  cpu.registers.b = 0b1111_1111;
  cpu.bus.write_byte(0x00, 0xB0);
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1111_1111);
  assert!(!cpu.registers.f.zero);
}

#[test]
fn or_a_c() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b0101_0000;
  cpu.registers.c = 0b0000_1010;
  cpu.bus.write_byte(0x00, 0xB1); // OR C
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0101_1010);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.registers.c = 0;
  cpu.bus.write_byte(0x00, 0xB1);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn or_a_d() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b0000_1111;
  cpu.registers.d = 0b1111_0000;
  cpu.bus.write_byte(0x00, 0xB2); // OR D
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1111_1111);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.registers.d = 0;
  cpu.bus.write_byte(0x00, 0xB2);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn or_a_e() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_0000;
  cpu.registers.e = 0b0000_0101;
  cpu.bus.write_byte(0x00, 0xB3); // OR E
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1010_0101);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.registers.e = 0;
  cpu.bus.write_byte(0x00, 0xB3);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn or_a_h() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b0000_0001;
  cpu.registers.h = 0b0000_0010;
  cpu.bus.write_byte(0x00, 0xB4); // OR H
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0000_0011);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.registers.h = 0;
  cpu.bus.write_byte(0x00, 0xB4);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn or_a_l() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1111_0000;
  cpu.registers.l = 0b0000_1111;
  cpu.bus.write_byte(0x00, 0xB5); // OR L
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1111_1111);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.registers.l = 0;
  cpu.bus.write_byte(0x00, 0xB5);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn or_a_hli() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b0000_1100;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 0b0011_0000);
  cpu.bus.write_byte(0x00, 0xB6); // OR (HL)
  cpu.step();
  assert_eq!(cpu.registers.a, 0b0011_1100);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.bus.write_byte(0x1234, 0);
  cpu.bus.write_byte(0x00, 0xB6);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn or_a_a() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0b1010_1010;
  cpu.bus.write_byte(0x00, 0xB7); // OR A
  cpu.step();
  assert_eq!(cpu.registers.a, 0b1010_1010);
  assert!(!cpu.registers.f.zero);
  assert!(!cpu.registers.f.carry);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.subtract);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 0;
  cpu.bus.write_byte(0x00, 0xB7);
  cpu.step();
  assert_eq!(cpu.registers.a, 0);
  assert!(cpu.registers.f.zero);
}

#[test]
fn cp_a_b() {
  let mut cpu = CPU::new();
  cpu.registers.a = 5;
  cpu.registers.b = 2;
  cpu.bus.write_byte(0x00, 0xB8); // CP B
  cpu.step();
  assert_eq!(cpu.registers.a, 5);
  assert!(!cpu.registers.f.zero);
  assert!(cpu.registers.f.subtract);
  assert!(!cpu.registers.f.half_carry);
  assert!(!cpu.registers.f.carry);

  // ゼロフラグ
  cpu.pc = 0;
  cpu.registers.a = 2;
  cpu.registers.b = 2;
  cpu.bus.write_byte(0x00, 0xB8);
  cpu.step();
  assert!(cpu.registers.f.zero);

  // キャリーフラグ
  cpu.pc = 0;
  cpu.registers.a = 1;
  cpu.registers.b = 2;
  cpu.bus.write_byte(0x00, 0xB8);
  cpu.step();
  assert!(cpu.registers.f.carry);
}

#[test]
fn cp_a_c() {
  let mut cpu = CPU::new();
  cpu.registers.a = 5;
  cpu.registers.c = 5;
  cpu.bus.write_byte(0x00, 0xB9); // CP C
  cpu.step();
  assert!(cpu.registers.f.zero);

  cpu.pc = 0;
  cpu.registers.a = 0x10;
  cpu.registers.c = 0x01;
  cpu.bus.write_byte(0x00, 0xB9);
  cpu.step();
  assert!(cpu.registers.f.half_carry);

  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.c = 0x01;
  cpu.bus.write_byte(0x00, 0xB9);
  cpu.step();
  assert!(cpu.registers.f.carry);
}

#[test]
fn cp_a_d() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.d = 0x01;
  cpu.bus.write_byte(0x00, 0xBA); // CP D
  cpu.step();
  assert!(cpu.registers.f.half_carry);

  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.d = 0x01;
  cpu.bus.write_byte(0x00, 0xBA);
  cpu.step();
  assert!(cpu.registers.f.zero);

  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.d = 0x01;
  cpu.bus.write_byte(0x00, 0xBA);
  cpu.step();
  assert!(cpu.registers.f.carry);
}

#[test]
fn cp_a_e() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.e = 0x01;
  cpu.bus.write_byte(0x00, 0xBB); // CP E
  cpu.step();
  assert!(cpu.registers.f.half_carry);

  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.e = 0x01;
  cpu.bus.write_byte(0x00, 0xBB);
  cpu.step();
  assert!(cpu.registers.f.zero);

  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.e = 0x01;
  cpu.bus.write_byte(0x00, 0xBB);
  cpu.step();
  assert!(cpu.registers.f.carry);
}

#[test]
fn cp_a_h() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.h = 0x01;
  cpu.bus.write_byte(0x00, 0xBC); // CP H
  cpu.step();
  assert!(cpu.registers.f.half_carry);

  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.h = 0x01;
  cpu.bus.write_byte(0x00, 0xBC);
  cpu.step();
  assert!(cpu.registers.f.zero);

  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.h = 0x01;
  cpu.bus.write_byte(0x00, 0xBC);
  cpu.step();
  assert!(cpu.registers.f.carry);
}

#[test]
fn cp_a_l() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.l = 0x01;
  cpu.bus.write_byte(0x00, 0xBD); // CP L
  cpu.step();
  assert!(cpu.registers.f.half_carry);

  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.registers.l = 0x01;
  cpu.bus.write_byte(0x00, 0xBD);
  cpu.step();
  assert!(cpu.registers.f.zero);

  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.registers.l = 0x01;
  cpu.bus.write_byte(0x00, 0xBD);
  cpu.step();
  assert!(cpu.registers.f.carry);
}

#[test]
fn cp_a_hli() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.registers.set_hl(0x1234);
  cpu.bus.write_byte(0x1234, 0x01);
  cpu.bus.write_byte(0x00, 0xBE); // CP (HL)
  cpu.step();
  assert!(cpu.registers.f.half_carry);

  cpu.pc = 0;
  cpu.registers.a = 0x01;
  cpu.bus.write_byte(0x1234, 0x01);
  cpu.bus.write_byte(0x00, 0xBE);
  cpu.step();
  assert!(cpu.registers.f.zero);

  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.bus.write_byte(0x1234, 0x01);
  cpu.bus.write_byte(0x00, 0xBE);
  cpu.step();
  assert!(cpu.registers.f.carry);
}

#[test]
fn cp_a_a() {
  let mut cpu = CPU::new();
  cpu.registers.a = 0x10;
  cpu.bus.write_byte(0x00, 0xBF); // CP A
  cpu.step();
  assert!(cpu.registers.f.zero);

  cpu.pc = 0;
  cpu.registers.a = 0x00;
  cpu.bus.write_byte(0x00, 0xBF);
  cpu.step();
  assert!(cpu.registers.f.zero);
}

#[test]
fn ret_nz() {
  let mut cpu = CPU::new();

  // スタックにリターンアドレスを積む
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x34); // LSB
  cpu.bus.write_byte(0xFFFD, 0x12); // MSB

  // Zeroフラグが0（リターンする場合）
  cpu.registers.f.zero = false;
  cpu.pc = 0x100;
  cpu.bus.write_byte(0x100, 0xC0); // RET NZ

  cpu.step();

  // スタックからアドレスを読みPCにセット
  assert_eq!(cpu.pc, 0x1234);
  // SPが2増加
  assert_eq!(cpu.sp, 0xFFFE);

  // Zeroフラグが1（リターンしない場合）
  let mut cpu = CPU::new();
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x34);
  cpu.bus.write_byte(0xFFFD, 0x12);
  cpu.registers.f.zero = true;
  cpu.pc = 0x200;
  cpu.bus.write_byte(0x200, 0xC0); // RET NZ

  cpu.step();

  // PCは次命令へ
  assert_eq!(cpu.pc, 0x201);
  // SPは変化しない
  assert_eq!(cpu.sp, 0xFFFC);
}

fn pop_bc() {
  let mut cpu = CPU::new();
  // スタックに値を積む
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x34); // LSB
  cpu.bus.write_byte(0xFFFD, 0x12); // MSB
  // POP BC 命令 (0xC1)
  cpu.bus.write_byte(0x00, 0xC1);
  cpu.step();
  // BCレジスタに値がセットされる
  assert_eq!(cpu.registers.get_bc(), 0x1234);
  // SPが2増加
  assert_eq!(cpu.sp, 0xFFFE);
  // PCが1進む
  assert_eq!(cpu.pc, 0x01);
}

#[test]
fn ret_z() {
  let mut cpu = CPU::new();

  // スタックにリターンアドレスを積む
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x78); // LSB
  cpu.bus.write_byte(0xFFFD, 0x56); // MSB

  // Zeroフラグが1（リターンする場合）
  cpu.registers.f.zero = true;
  cpu.pc = 0x100;
  cpu.bus.write_byte(0x100, 0xC8); // RET Z

  cpu.step();

  // スタックからアドレスを読みPCにセット
  assert_eq!(cpu.pc, 0x5678);
  // SPが2増加
  assert_eq!(cpu.sp, 0xFFFE);

  // Zeroフラグが0（リターンしない場合）
  let mut cpu = CPU::new();
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x78);
  cpu.bus.write_byte(0xFFFD, 0x56);
  cpu.registers.f.zero = false;
  cpu.pc = 0x200;
  cpu.bus.write_byte(0x200, 0xC8); // RET Z

  cpu.step();

  // PCは次命令へ
  assert_eq!(cpu.pc, 0x201);
  // SPは変化しない
  assert_eq!(cpu.sp, 0xFFFC);
}

#[test]
fn ret() {
  let mut cpu = CPU::new();

  // スタックにリターンアドレス
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0xCD); // LSB
  cpu.bus.write_byte(0xFFFD, 0xAB); // MSB

  cpu.pc = 0x100;
  cpu.bus.write_byte(0x100, 0xC9); // RET

  cpu.step();

  // スタックからアドレスを読みPCにセット
  assert_eq!(cpu.pc, 0xABCD);
  // SPが2増加
  assert_eq!(cpu.sp, 0xFFFE);
}

#[test]
fn ret_nc() {
  let mut cpu = CPU::new();

  // スタックにリターンアドレスを積む
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x34); // LSB
  cpu.bus.write_byte(0xFFFD, 0x12); // MSB

  // キャリーフラグが0（リターンする場合）
  cpu.registers.f.carry = false;
  cpu.pc = 0x100;
  cpu.bus.write_byte(0x100, 0xD0); // RET NC

  cpu.step();

  // スタックからアドレスを読みPCにセット
  assert_eq!(cpu.pc, 0x1234);
  // SPが2増加
  assert_eq!(cpu.sp, 0xFFFE);

  // キャリーフラグが1（リターンしない場合）
  let mut cpu = CPU::new();
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x34);
  cpu.bus.write_byte(0xFFFD, 0x12);
  cpu.registers.f.carry = true;
  cpu.pc = 0x200;
  cpu.bus.write_byte(0x200, 0xD0); // RET NC

  cpu.step();

  // PCは次命令へ
  assert_eq!(cpu.pc, 0x201);
  // SPは変化しない
  assert_eq!(cpu.sp, 0xFFFC);
}

#[test]
fn pop_de() {
    let mut cpu = CPU::new();
    cpu.sp = 0xFFFC;
    cpu.bus.write_byte(0xFFFC, 0x78); // LSB
    cpu.bus.write_byte(0xFFFD, 0x56); // MSB
    // POP DE 命令 (0xD1)
    cpu.bus.write_byte(0x00, 0xD1);
    cpu.step();
    assert_eq!(cpu.registers.get_de(), 0x5678);
    assert_eq!(cpu.sp, 0xFFFE);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn ret_c() {
  let mut cpu = CPU::new();

  // スタックにリターンアドレスを積む
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x78); // LSB
  cpu.bus.write_byte(0xFFFD, 0x56); // MSB

  // キャリーフラグが1（リターンする場合）
  cpu.registers.f.carry = true;
  cpu.pc = 0x100;
  cpu.bus.write_byte(0x100, 0xD8); // RET C

  cpu.step();

  // スタックからアドレスを読みPCにセット
  assert_eq!(cpu.pc, 0x5678);
  // SPが2増加
  assert_eq!(cpu.sp, 0xFFFE);

  // キャリーフラグが0（リターンしない場合）
  let mut cpu = CPU::new();
  cpu.sp = 0xFFFC;
  cpu.bus.write_byte(0xFFFC, 0x78);
  cpu.bus.write_byte(0xFFFD, 0x56);
  cpu.registers.f.carry = false;
  cpu.pc = 0x200;
  cpu.bus.write_byte(0x200, 0xD8); // RET C

  cpu.step();

  // PCは次命令へ
  assert_eq!(cpu.pc, 0x201);
  // SPは変化しない
  assert_eq!(cpu.sp, 0xFFFC);
}

#[test]
fn pop_hl() {
    let mut cpu = CPU::new();
    cpu.sp = 0xFFFC;
    cpu.bus.write_byte(0xFFFC, 0xBC); // LSB
    cpu.bus.write_byte(0xFFFD, 0x9A); // MSB
    // POP HL 命令 (0xE1)
    cpu.bus.write_byte(0x00, 0xE1);
    cpu.step();
    assert_eq!(cpu.registers.get_hl(), 0x9ABC);
    assert_eq!(cpu.sp, 0xFFFE);
    assert_eq!(cpu.pc, 0x01);
}

#[test]
fn pop_af() {
    let mut cpu = CPU::new();
    cpu.sp = 0xFFFC;
    cpu.bus.write_byte(0xFFFC, 0xF0); // LSB (Flags)
    cpu.bus.write_byte(0xFFFD, 0x0D); // MSB (A)
    // POP AF 命令 (0xF1)
    cpu.bus.write_byte(0x00, 0xF1);
    cpu.step();
    assert_eq!(cpu.registers.get_af(), 0x0DF0);
    assert_eq!(cpu.sp, 0xFFFE);
    assert_eq!(cpu.pc, 0x01);
}