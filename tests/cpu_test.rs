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
    assert_eq!(cpu.pc, 0x01);
    assert!(cpu.registers.f.half_carry);
    assert!(cpu.registers.f.subtract);

    // 3. ゼロフラグ
    cpu.pc = 0;
    cpu.registers.e = 0x01;
    cpu.bus.write_byte(0x00, 0x1D); // DEC E
    cpu.step();
    assert_eq!(cpu.registers.e, 0x00);
    assert_eq!(cpu.pc, 0x01);
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
