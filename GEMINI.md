# Game Boy (DMG-01) エミュレータについて

このディレクトリは、任天堂の初代ゲームボーイ（DMG-01）をソフトウェア上で再現するエミュレータの開発用です。

## 概要

- **目的**: Game Boyのハードウェア挙動をPC上で再現し、ROMイメージを動作させること
- **参考**: [The Ultimate Game Boy Talk](https://rylev.github.io/DMG-01/public/book/introduction.html) の内容をベースに実装

## 主な特徴

- Z80系CPU（Sharp LR35902）の命令セットエミュレーション
- メモリマッピング、バンク切り替えの再現
- グラフィック（LCDコントローラ）、サウンド、入力（ボタン）など主要I/Oの再現
- デバッグやトレース機能の追加も検討

## 参考リンク

- [The Ultimate Game Boy Talk](https://rylev.github.io/DMG-01/public/book/introduction.html)
- 実機資料や技術ドキュメント

---
このプロジェクトは、Game Boyの仕組みを学びつつ、エミュレータ開発の技術習得を目的としています。私の質問に対する返答で一般的で再現性のあるものは、コーディングに関するものもそうでないものも随時docsディレクトリにmdファイルでまとめていってください。