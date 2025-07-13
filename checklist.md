### 通常オペコード 00–FF
#### 00–0F
- [x] 00 NOP
- [x] 01 LD BC,d16
- [x] 02 LD (BC),A
- [x] 03 INC BC
- [x] 04 INC B
- [x] 05 DEC B
- [x] 06 LD B,d8
- [x] 07 RLCA
- [x] 08 LD (a16),SP
- [x] 09 ADD HL,BC
- [x] 0A LD A,(BC)
- [x] 0B DEC BC
- [x] 0C INC C
- [x] 0D DEC C
- [x] 0E LD C,d8
- [x] 0F RRCA

#### 10–1F
- [ ] 10 STOP
- [x] 11 LD DE,d16
- [x] 12 LD (DE),A
- [x] 13 INC DE
- [x] 14 INC D
- [x] 15 DEC D
- [x] 16 LD D,d8
- [x] 17 RLA
- [x] 18 JR r8
- [x] 19 ADD HL,DE
- [x] 1A LD A,(DE)
- [x] 1B DEC DE
- [x] 1C INC E
- [x] 1D DEC E
- [x] 1E LD E,d8
- [x] 1F RRA

#### 20–2F
- [x] 20 JR NZ,r8
- [x] 21 LD HL,d16
- [x] 22 LD (HL+),A
- [x] 23 INC HL
- [x] 24 INC H
- [x] 25 DEC H
- [x] 26 LD H,d8
- [x] 27 DAA
- [x] 28 JR Z,r8
- [x] 29 ADD HL,HL
- [x] 2A LD A,(HL+)
- [x] 2B DEC HL
- [x] 2C INC L
- [x] 2D DEC L
- [x] 2E LD L,d8
- [x] 2F CPL

#### 30–3F
- [x] 30 JR NC,r8
- [x] 31 LD SP,d16
- [x] 32 LD (HL-),A
- [x] 33 INC SP
- [x] 34 INC (HL)
- [x] 35 DEC (HL)
- [x] 36 LD (HL),d8
- [x] 37 SCF
- [x] 38 JR C,r8
- [x] 39 ADD HL,SP
- [x] 3A LD A,(HL-)
- [x] 3B DEC SP
- [x] 3C INC A
- [x] 3D DEC A
- [x] 3E LD A,d8
- [x] 3F CCF

#### 40–4F
- [x] 40 LD B,B
- [x] 41 LD B,C
- [x] 42 LD B,D
- [x] 43 LD B,E
- [x] 44 LD B,H
- [x] 45 LD B,L
- [x] 46 LD B,(HL)
- [x] 47 LD B,A
- [x] 48 LD C,B
- [x] 49 LD C,C
- [x] 4A LD C,D
- [x] 4B LD C,E
- [x] 4C LD C,H
- [x] 4D LD C,L
- [x] 4E LD C,(HL)
- [x] 4F LD C,A

#### 50–5F
- [x] 50 LD D,B
- [x] 51 LD D,C
- [x] 52 LD D,D
- [x] 53 LD D,E
- [x] 54 LD D,H
- [x] 55 LD D,L
- [x] 56 LD D,(HL)
- [x] 57 LD D,A
- [x] 58 LD E,B
- [x] 59 LD E,C
- [x] 5A LD E,D
- [x] 5B LD E,E
- [x] 5C LD E,H
- [x] 5D LD E,L
- [x] 5E LD E,(HL)
- [x] 5F LD E,A

#### 60–6F
- [x] 60 LD H,B
- [x] 61 LD H,C
- [x] 62 LD H,D
- [x] 63 LD H,E
- [x] 64 LD H,H
- [x] 65 LD H,L
- [x] 66 LD H,(HL)
- [x] 67 LD H,A
- [x] 68 LD L,B
- [x] 69 LD L,C
- [x] 6A LD L,D
- [x] 6B LD L,E
- [x] 6C LD L,H
- [x] 6D LD L,L
- [x] 6E LD L,(HL)
- [x] 6F LD L,A

#### 70–7F
- [x] 70 LD (HL),B
- [x] 71 LD (HL),C
- [x] 72 LD (HL),D
- [x] 73 LD (HL),E
- [x] 74 LD (HL),H
- [x] 75 LD (HL),L
- [ ] 76 HALT
- [x] 77 LD (HL),A
- [x] 78 LD A,B
- [x] 79 LD A,C
- [x] 7A LD A,D
- [x] 7B LD A,E
- [x] 7C LD A,H
- [x] 7D LD A,L
- [x] 7E LD A,(HL)
- [x] 7F LD A,A

#### 80–8F
- [x] 80 ADD A,B
- [x] 81 ADD A,C
- [x] 82 ADD A,D
- [x] 83 ADD A,E
- [x] 84 ADD A,H
- [x] 85 ADD A,L
- [x] 86 ADD A,(HL)
- [x] 87 ADD A,A
- [x] 88 ADC A,B
- [x] 89 ADC A,C
- [x] 8A ADC A,D
- [x] 8B ADC A,E
- [x] 8C ADC A,H
- [x] 8D ADC A,L
- [x] 8E ADC A,(HL)
- [x] 8F ADC A,A

#### 90–9F
- [x] 90 SUB A,B
- [x] 91 SUB A,C
- [x] 92 SUB A,D
- [x] 93 SUB A,E
- [x] 94 SUB A,H
- [x] 95 SUB A,L
- [x] 96 SUB A,(HL)
- [x] 97 SUB A,A
- [x] 98 SBC A,B
- [x] 99 SBC A,C
- [x] 9A SBC A,D
- [x] 9B SBC A,E
- [x] 9C SBC A,H
- [x] 9D SBC A,L
- [x] 9E SBC A,(HL)
- [x] 9F SBC A,A

#### A0–AF
- [x] A0 AND A,B
- [x] A1 AND A,C
- [x] A2 AND A,D
- [x] A3 AND A,E
- [x] A4 AND A,H
- [x] A5 AND A,L
- [x] A6 AND A,(HL)
- [x] A7 AND A,A
- [x] A8 XOR A,B
- [x] A9 XOR A,C
- [x] AA XOR A,D
- [x] AB XOR A,E
- [x] AC XOR A,H
- [x] AD XOR A,L
- [x] AE XOR A,(HL)
- [x] AF XOR A,A

#### B0–BF
- [x] B0 OR A,B
- [x] B1 OR A,C
- [x] B2 OR A,D
- [x] B3 OR A,E
- [x] B4 OR A,H
- [x] B5 OR A,L
- [x] B6 OR A,(HL)
- [x] B7 OR A,A
- [x] B8 CP A,B
- [x] B9 CP A,C
- [x] BA CP A,D
- [x] BB CP A,E
- [x] BC CP A,H
- [x] BD CP A,L
- [x] BE CP A,(HL)
- [x] BF CP A,A

#### C0–CF
- [ ] C0 RET NZ
- [ ] C1 POP BC
- [ ] C2 JP NZ,a16
- [ ] C3 JP a16
- [ ] C4 CALL NZ,a16
- [ ] C5 PUSH BC
- [ ] C6 ADD A,d8
- [ ] C7 RST 00H
- [ ] C8 RET Z
- [ ] C9 RET
- [ ] CA JP Z,a16
- [ ] CB PREFIX CB
- [ ] CC CALL Z,a16
- [ ] CD CALL a16
- [ ] CE ADC A,d8
- [ ] CF RST 08H

#### D0–DF
- [ ] D0 RET NC
- [ ] D1 POP DE
- [ ] D2 JP NC,a16
- [ ] D3 — (undefined)
- [ ] D4 CALL NC,a16
- [ ] D5 PUSH DE
- [ ] D6 SUB d8
- [ ] D7 RST 10H
- [ ] D8 RET C
- [ ] D9 RETI
- [ ] DA JP C,a16
- [ ] DB — (undefined)
- [ ] DC CALL C,a16
- [ ] DD — (undefined)
- [ ] DE SBC A,d8
- [ ] DF RST 18H

#### E0–EF
- [ ] E0 LDH (a8),A
- [ ] E1 POP HL
- [ ] E2 LD (C),A
- [ ] E3 — (undefined)
- [ ] E4 — (undefined)
- [ ] E5 PUSH HL
- [ ] E6 AND d8
- [ ] E7 RST 20H
- [ ] E8 ADD SP,r8
- [ ] E9 JP (HL)
- [ ] EA LD (a16),A
- [ ] EB — (undefined)
- [ ] EC — (undefined)
- [ ] ED — (undefined)
- [ ] EE XOR d8
- [ ] EF RST 28H

#### F0–FF
- [ ] F0 LDH A,(a8)
- [ ] F1 POP AF
- [ ] F2 LD A,(C)
- [ ] F3 DI
- [ ] F4 — (undefined)
- [ ] F5 PUSH AF
- [ ] F6 OR d8
- [ ] F7 RST 30H
- [ ] F8 LD HL,SP+r8
- [ ] F9 LD SP,HL
- [ ] FA LD A,(a16)
- [ ] FB EI
- [ ] FC — (undefined)
- [ ] FD — (undefined)
- [ ] FE CP d8
- [ ] FF RST 38H

---

### CBオペコード CB00–CBFF
#### CB00–CB0F
- [ ] CB00 RLC B
- [ ] CB01 RLC C
- [ ] CB02 RLC D
- [ ] CB03 RLC E
- [ ] CB04 RLC H
- [ ] CB05 RLC L
- [ ] CB06 RLC (HL)
- [ ] CB07 RLC A
- [ ] CB08 RRC B
- [ ] CB09 RRC C
- [ ] CB0A RRC D
- [ ] CB0B RRC E
- [ ] CB0C RRC H
- [ ] CB0D RRC L
- [ ] CB0E RRC (HL)
- [ ] CB0F RRC A

#### CB10–CB1F
- [ ] CB10 RL B
- [ ] CB11 RL C
- [ ] CB12 RL D
- [ ] CB13 RL E
- [ ] CB14 RL H
- [ ] CB15 RL L
- [ ] CB16 RL (HL)
- [ ] CB17 RL A
- [ ] CB18 RR B
- [ ] CB19 RR C
- [ ] CB1A RR D
- [ ] CB1B RR E
- [ ] CB1C RR H
- [ ] CB1D RR L
- [ ] CB1E RR (HL)
- [ ] CB1F RR A

#### CB20–CB2F
- [ ] CB20 SLA B
- [ ] CB21 SLA C
- [ ] CB22 SLA D
- [ ] CB23 SLA E
- [ ] CB24 SLA H
- [ ] CB25 SLA L
- [ ] CB26 SLA (HL)
- [ ] CB27 SLA A
- [ ] CB28 SRA B
- [ ] CB29 SRA C
- [ ] CB2A SRA D
- [ ] CB2B SRA E
- [ ] CB2C SRA H
- [ ] CB2D SRA L
- [ ] CB2E SRA (HL)
- [ ] CB2F SRA A

#### CB30–CB3F
- [ ] CB30 SWAP B
- [ ] CB31 SWAP C
- [ ] CB32 SWAP D
- [ ] CB33 SWAP E
- [ ] CB34 SWAP H
- [ ] CB35 SWAP L
- [ ] CB36 SWAP (HL)
- [ ] CB37 SWAP A
- [ ] CB38 SRL B
- [ ] CB39 SRL C
- [ ] CB3A SRL D
- [ ] CB3B SRL E
- [ ] CB3C SRL H
- [ ] CB3D SRL L
- [ ] CB3E SRL (HL)
- [ ] CB3F SRL A

#### CB40–CB4F (BIT 0–7, register)
- [ ] CB40 BIT 0,B
- [ ] CB41 BIT 0,C
- [ ] CB42 BIT 0,D
- [ ] CB43 BIT 0,E
- [ ] CB44 BIT 0,H
- [ ] CB45 BIT 0,L
- [ ] CB46 BIT 0,(HL)
- [ ] CB47 BIT 0,A
- [ ] CB48 BIT 1,B
- [ ] CB49 BIT 1,C
- [ ] CB4A BIT 1,D
- [ ] CB4B BIT 1,E
- [ ] CB4C BIT 1,H
- [ ] CB4D BIT 1,L
- [ ] CB4E BIT 1,(HL)
- [ ] CB4F BIT 1,A

#### CB50–CB5F
- [ ] CB50 BIT 2,B
- [ ] CB51 BIT 2,C
- [ ] CB52 BIT 2,D
- [ ] CB53 BIT 2,E
- [ ] CB54 BIT 2,H
- [ ] CB55 BIT 2,L
- [ ] CB56 BIT 2,(HL)
- [ ] CB57 BIT 2,A
- [ ] CB58 BIT 3,B
- [ ] CB59 BIT 3,C
- [ ] CB5A BIT 3,D
- [ ] CB5B BIT 3,E
- [ ] CB5C BIT 3,H
- [ ] CB5D BIT 3,L
- [ ] CB5E BIT 3,(HL)
- [ ] CB5F BIT 3,A

#### CB60–CB6F
- [ ] CB60 BIT 4,B
- [ ] CB61 BIT 4,C
- [ ] CB62 BIT 4,D
- [ ] CB63 BIT 4,E
- [ ] CB64 BIT 4,H
- [ ] CB65 BIT 4,L
- [ ] CB66 BIT 4,(HL)
- [ ] CB67 BIT 4,A
- [ ] CB68 BIT 5,B
- [ ] CB69 BIT 5,C
- [ ] CB6A BIT 5,D
- [ ] CB6B BIT 5,E
- [ ] CB6C BIT 5,H
- [ ] CB6D BIT 5,L
- [ ] CB6E BIT 5,(HL)
- [ ] CB6F BIT 5,A

#### CB70–CB7F
- [ ] CB70 BIT 6,B
- [ ] CB71 BIT 6,C
- [ ] CB72 BIT 6,D
- [ ] CB73 BIT 6,E
- [ ] CB74 BIT 6,H
- [ ] CB75 BIT 6,L
- [ ] CB76 BIT 6,(HL)
- [ ] CB77 BIT 6,A
- [ ] CB78 BIT 7,B
- [ ] CB79 BIT 7,C
- [ ] CB7A BIT 7,D
- [ ] CB7B BIT 7,E
- [ ] CB7C BIT 7,H
- [ ] CB7D BIT 7,L
- [ ] CB7E BIT 7,(HL)
- [ ] CB7F BIT 7,A

#### CB80–CB8F (RES 0–7, register)
- [ ] CB80 RES 0,B
- [ ] CB81 RES 0,C
- [ ] CB82 RES 0,D
- [ ] CB83 RES 0,E
- [ ] CB84 RES 0,H
- [ ] CB85 RES 0,L
- [ ] CB86 RES 0,(HL)
- [ ] CB87 RES 0,A
- [ ] CB88 RES 1,B
- [ ] CB89 RES 1,C
- [ ] CB8A RES 1,D
- [ ] CB8B RES 1,E
- [ ] CB8C RES 1,H
- [ ] CB8D RES 1,L
- [ ] CB8E RES 1,(HL)
- [ ] CB8F RES 1,A

#### CB90–CB9F
- [ ] CB90 RES 2,B
- [ ] CB91 RES 2,C
- [ ] CB92 RES 2,D
- [ ] CB93 RES 2,E
- [ ] CB94 RES 2,H
- [ ] CB95 RES 2,L
- [ ] CB96 RES 2,(HL)
- [ ] CB97 RES 2,A
- [ ] CB98 RES 3,B
- [ ] CB99 RES 3,C
- [ ] CB9A RES 3,D
- [ ] CB9B RES 3,E
- [ ] CB9C RES 3,H
- [ ] CB9D RES 3,L
- [ ] CB9E RES 3,(HL)
- [ ] CB9F RES 3,A

#### CBA0–CB AF
- [ ] CBA0 RES 4,B
- [ ] CBA1 RES 4,C
- [ ] CBA2 RES 4,D
- [ ] CBA3 RES 4,E
- [ ] CBA4 RES 4,H
- [ ] CBA5 RES 4,L
- [ ] CBA6 RES 4,(HL)
- [ ] CBA7 RES 4,A
- [ ] CBA8 RES 5,B
- [ ] CBA9 RES 5,C
- [ ] CBAA RES 5,D
- [ ] CBAB RES 5,E
- [ ] CBAC RES 5,H
- [ ] CBAD RES 5,L
- [ ] CBAE RES 5,(HL)
- [ ] CBAF RES 5,A

#### CBB0–CBBF
- [ ] CBB0 RES 6,B
- [ ] CBB1 RES 6,C
- [ ] CBB2 RES 6,D
- [ ] CBB3 RES 6,E
- [ ] CBB4 RES 6,H
- [ ] CBB5 RES 6,L
- [ ] CBB6 RES 6,(HL)
- [ ] CBB7 RES 6,A
- [ ] CBB8 RES 7,B
- [ ] CBB9 RES 7,C
- [ ] CBBA RES 7,D
- [ ] CBBB RES 7,E
- [ ] CBBC RES 7,H
- [ ] CBBD RES 7,L
- [ ] CBBE RES 7,(HL)
- [ ] CBBF RES 7,A

#### CBC0–CBCF (SET 0–7, register)
- [ ] CBC0 SET 0,B
- [ ] CBC1 SET 0,C
- [ ] CBC2 SET 0,D
- [ ] CBC3 SET 0,E
- [ ] CBC4 SET 0,H
- [ ] CBC5 SET 0,L
- [ ] CBC6 SET 0,(HL)
- [ ] CBC7 SET 0,A
- [ ] CBC8 SET 1,B
- [ ] CBC9 SET 1,C
- [ ] CBCA SET 1,D
- [ ] CBCB SET 1,E
- [ ] CBCC SET 1,H
- [ ] CBCD SET 1,L
- [ ] CBCE SET 1,(HL)
- [ ] CBCF SET 1,A

#### CBD0–CBD F
- [ ] CBD0 SET 2,B
- [ ] CBD1 SET 2,C
- [ ] CBD2 SET 2,D
- [ ] CBD3 SET 2,E
- [ ] CBD4 SET 2,H
- [ ] CBD5 SET 2,L
- [ ] CBD6 SET 2,(HL)
- [ ] CBD7 SET 2,A
- [ ] CBD8 SET 3,B
- [ ] CBD9 SET 3,C
- [ ] CBDA SET 3,D
- [ ] CBDB SET 3,E
- [ ] CBDC SET 3,H
- [ ] CBDD SET 3,L
- [ ] CBDE SET 3,(HL)
- [ ] CBDF SET 3,A

#### CBE0–CBEF
- [ ] CBE0 SET 4,B
- [ ] CBE1 SET 4,C
- [ ] CBE2 SET 4,D
- [ ] CBE3 SET 4,E
- [ ] CBE4 SET 4,H
- [ ] CBE5 SET 4,L
- [ ] CBE6 SET 4,(HL)
- [ ] CBE7 SET 4,A
- [ ] CBE8 SET 5,B
- [ ] CBE9 SET 5,C
- [ ] CBEA SET 5,D
- [ ] CBEB SET 5,E
- [ ] CBEC SET 5,H
- [ ] CBED SET 5,L
- [ ] CBEE SET 5,(HL)
- [ ] CBEF SET 5,A

#### CBF0–CBFF
- [ ] CBF0 SET 6,B
- [ ] CBF1 SET 6,C
- [ ] CBF2 SET 6,D
- [ ] CBF3 SET 6,E
- [ ] CBF4 SET 6,H
- [ ] CBF5 SET 6,L
- [ ] CBF6 SET 6,(HL)
- [ ] CBF7 SET 6,A
- [ ] CBF8 SET 7,B
- [ ] CBF9 SET 7,C
- [ ] CBFA SET 7,D
- [ ] CBFB SET 7,E
- [ ] CBFC SET 7,H
- [ ] CBFD SET 7,L
- [ ] CBFE SET 7,(HL)
- [ ] CBFF SET 7,A
