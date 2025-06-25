# Enumの定義

Rustでは、`enum`のヴァリアント（取りうる値の種類）は、使用する前にすべて定義されている必要があります。

今回のプロジェクトでは、`Instruction` enumを定義しましたが、当初はすべての命令を網羅していませんでした。そのため、`match`式で未定義のヴァリアント（例: `Instruction::ADD`）を使用しようとした際に、コンパイルエラーが発生しました。

```rust
// src/CPU.rs

// エラーが発生したコード
enum Instruction {
  JP(JumpTest),
  LD(LoadType),
}

// ...

// Instruction::ADD は未定義のためエラー
match instruction {
  Instruction::ADD(target) => { ... }
  // ...
}
```

このエラーを解決するには、`Instruction` enumの定義に、使用するすべてのヴァリアントを追加する必要があります。

```rust
// src/CPU.rs

// 修正後のコード
enum Instruction {
  ADD(ArithmeticTarget),
  PUSH(StackTarget),
  POP(StackTarget),
  CALL(JumpTest),
  RET(JumpTest),
  RLC(PrefixTarget),
  INC(IncDecTarget),
  DEC(IncDecTarget), // DECを追加
  JP(JumpTest),
  LD(LoadType),
}
```

このように、プログラムで使用する可能性のあるすべての値を`enum`の定義に含めることが重要です。
