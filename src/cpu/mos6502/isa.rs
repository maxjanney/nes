use self::{AddressingMode::*, Opcode::*};
use super::{Psr, Registers};
use crate::mem::Memory;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum AddressingMode {
    Accumulator,
    Immediate,
    Absolute,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    Implied,
    Relative,
    IndirectX,
    IndirectY,
    Indirect,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Opcode {
    ADC,
    SBC,
    AND,
    EOR,
    ORA,
    ASL,
    LSR,
    ROL,
    ROR,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    SEC,
    SED,
    SEI,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    JMP,
    JSR,
    RTI,
    RTS,
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    PHA,
    PHP,
    PLA,
    PLP,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    BRK,
    BIT,
    NOP,
    ALR,
    ANC,
    ARR,
    AXS,
    LAX,
    SAX,
    DCP,
    ISC,
    RLA,
    RRA,
    SLO,
    SRE,
    SKB,
    IGN,
}

/// Argument, address, extra cycle
struct Operand(u8, u16, u32);

/// Addressing mode, opcode, cycles
struct Instruction(AddressingMode, Opcode, u32);

impl From<u8> for Instruction {
    fn from(op: u8) -> Self {
        match op {
            0x69 => Instruction(Immediate, ADC, 2),
            0x65 => Instruction(ZeroPage, ADC, 3),
            0x75 => Instruction(ZeroPageX, ADC, 4),
            0x6d => Instruction(Absolute, ADC, 4),
            0x7d => Instruction(AbsoluteX, ADC, 4),
            0x79 => Instruction(AbsoluteY, ADC, 4),
            0x61 => Instruction(IndirectX, ADC, 6),
            0x71 => Instruction(IndirectY, ADC, 5),

            0x29 => Instruction(Immediate, AND, 2),
            0x25 => Instruction(ZeroPage, AND, 3),
            0x35 => Instruction(ZeroPageX, AND, 4),
            0x2d => Instruction(Absolute, AND, 4),
            0x3d => Instruction(AbsoluteX, AND, 4),
            0x39 => Instruction(AbsoluteY, AND, 4),
            0x21 => Instruction(IndirectX, AND, 6),
            0x31 => Instruction(IndirectY, AND, 5),

            0x0a => Instruction(Accumulator, ASL, 2),
            0x06 => Instruction(ZeroPage, ASL, 5),
            0x16 => Instruction(ZeroPageX, ASL, 6),
            0x0e => Instruction(Absolute, ASL, 6),
            0x1e => Instruction(AbsoluteX, ASL, 7),

            0x24 => Instruction(ZeroPage, BIT, 3),
            0x2c => Instruction(Absolute, BIT, 4),

            0x00 => Instruction(Implied, BRK, 7),

            0x90 => Instruction(Relative, BCC, 2),
            0xb0 => Instruction(Relative, BCS, 2),
            0xf0 => Instruction(Relative, BEQ, 2),
            0x30 => Instruction(Relative, BMI, 2),
            0xd0 => Instruction(Relative, BNE, 2),
            0x10 => Instruction(Relative, BPL, 2),
            0x50 => Instruction(Relative, BVC, 2),
            0x70 => Instruction(Relative, BVS, 2),

            0x18 => Instruction(Implied, CLC, 2),
            0xd8 => Instruction(Implied, CLD, 2),
            0x58 => Instruction(Implied, CLI, 2),
            0xb8 => Instruction(Implied, CLV, 2),

            0xc9 => Instruction(Immediate, CMP, 2),
            0xc5 => Instruction(ZeroPage, CMP, 3),
            0xd5 => Instruction(ZeroPageX, CMP, 4),
            0xcd => Instruction(Absolute, CMP, 4),
            0xdd => Instruction(AbsoluteX, CMP, 4),
            0xd9 => Instruction(AbsoluteY, CMP, 4),
            0xc1 => Instruction(IndirectX, CMP, 6),
            0xd1 => Instruction(IndirectY, CMP, 5),

            0xe0 => Instruction(Immediate, CPX, 2),
            0xe4 => Instruction(ZeroPage, CPX, 3),
            0xec => Instruction(Absolute, CPX, 4),

            0xc0 => Instruction(Immediate, CPY, 2),
            0xc4 => Instruction(ZeroPage, CPY, 3),
            0xcc => Instruction(Absolute, CPY, 4),

            0xc6 => Instruction(ZeroPage, DEC, 5),
            0xd6 => Instruction(ZeroPageX, DEC, 6),
            0xce => Instruction(Absolute, DEC, 6),
            0xde => Instruction(AbsoluteX, DEC, 7),

            0xca => Instruction(Implied, DEX, 2),
            0x88 => Instruction(Implied, DEY, 2),

            0x49 => Instruction(Immediate, EOR, 2),
            0x45 => Instruction(ZeroPage, EOR, 3),
            0x55 => Instruction(ZeroPageX, EOR, 4),
            0x4d => Instruction(Absolute, EOR, 4),
            0x5d => Instruction(AbsoluteX, EOR, 4),
            0x59 => Instruction(AbsoluteY, EOR, 4),
            0x41 => Instruction(IndirectX, EOR, 6),
            0x51 => Instruction(IndirectY, EOR, 5),

            0xe6 => Instruction(ZeroPage, INC, 5),
            0xf6 => Instruction(ZeroPageX, INC, 6),
            0xee => Instruction(Absolute, INC, 6),
            0xfe => Instruction(AbsoluteX, INC, 7),

            0xe8 => Instruction(Implied, INX, 2),
            0xc8 => Instruction(Implied, INY, 2),

            0x4c => Instruction(Absolute, JMP, 3),
            0x6c => Instruction(Indirect, JMP, 5),
            0x20 => Instruction(Absolute, JSR, 6),

            0xa9 => Instruction(Immediate, LDA, 2),
            0xa5 => Instruction(ZeroPage, LDA, 3),
            0xb5 => Instruction(ZeroPageX, LDA, 4),
            0xad => Instruction(Absolute, LDA, 4),
            0xbd => Instruction(AbsoluteX, LDA, 4),
            0xb9 => Instruction(AbsoluteY, LDA, 4),
            0xa1 => Instruction(IndirectX, LDA, 6),
            0xb1 => Instruction(IndirectY, LDA, 5),

            0xa2 => Instruction(Immediate, LDX, 2),
            0xa6 => Instruction(ZeroPage, LDX, 3),
            0xb6 => Instruction(ZeroPageY, LDX, 4),
            0xae => Instruction(Absolute, LDX, 4),
            0xbe => Instruction(AbsoluteY, LDX, 4),

            0xa0 => Instruction(Immediate, LDY, 2),
            0xa4 => Instruction(ZeroPage, LDY, 3),
            0xb4 => Instruction(ZeroPageX, LDY, 4),
            0xac => Instruction(Absolute, LDY, 4),
            0xbc => Instruction(AbsoluteX, LDY, 4),

            0x4a => Instruction(Accumulator, LSR, 2),
            0x46 => Instruction(ZeroPage, LSR, 5),
            0x56 => Instruction(ZeroPageX, LSR, 6),
            0x4e => Instruction(Absolute, LSR, 6),
            0x5e => Instruction(AbsoluteX, LSR, 7),

            0xea => Instruction(Implied, NOP, 2),

            0x09 => Instruction(Immediate, ORA, 2),
            0x05 => Instruction(ZeroPage, ORA, 3),
            0x15 => Instruction(ZeroPageX, ORA, 4),
            0x0d => Instruction(Absolute, ORA, 4),
            0x1d => Instruction(AbsoluteX, ORA, 4),
            0x19 => Instruction(AbsoluteY, ORA, 4),
            0x01 => Instruction(IndirectX, ORA, 6),
            0x11 => Instruction(IndirectY, ORA, 5),

            0x48 => Instruction(Implied, PHA, 3),
            0x08 => Instruction(Implied, PHP, 3),
            0x68 => Instruction(Implied, PLA, 4),
            0x28 => Instruction(Implied, PLP, 4),

            0x2a => Instruction(Accumulator, ROL, 2),
            0x26 => Instruction(ZeroPage, ROL, 5),
            0x36 => Instruction(ZeroPageX, ROL, 6),
            0x2e => Instruction(Absolute, ROL, 6),
            0x3e => Instruction(AbsoluteX, ROL, 7),

            0x6a => Instruction(Accumulator, ROR, 2),
            0x66 => Instruction(ZeroPage, ROR, 5),
            0x76 => Instruction(ZeroPageX, ROR, 6),
            0x6e => Instruction(Absolute, ROR, 6),
            0x7e => Instruction(AbsoluteX, ROR, 7),

            0x40 => Instruction(Implied, RTI, 6),
            0x60 => Instruction(Implied, RTS, 6),

            0xe9 => Instruction(Immediate, SBC, 2),
            0xe5 => Instruction(ZeroPage, SBC, 3),
            0xf5 => Instruction(ZeroPageX, SBC, 4),
            0xed => Instruction(Absolute, SBC, 4),
            0xfd => Instruction(AbsoluteX, SBC, 4),
            0xf9 => Instruction(AbsoluteY, SBC, 4),
            0xe1 => Instruction(IndirectX, SBC, 6),
            0xf1 => Instruction(IndirectY, SBC, 5),

            0x38 => Instruction(Implied, SEC, 2),
            0xf8 => Instruction(Implied, SED, 2),
            0x78 => Instruction(Implied, SEI, 2),

            0x85 => Instruction(ZeroPage, STA, 3),
            0x95 => Instruction(ZeroPageX, STA, 4),
            0x8d => Instruction(Absolute, STA, 4),
            0x9d => Instruction(AbsoluteX, STA, 5),
            0x99 => Instruction(AbsoluteY, STA, 5),
            0x81 => Instruction(IndirectX, STA, 6),
            0x91 => Instruction(IndirectY, STA, 6),

            0x86 => Instruction(ZeroPage, STX, 3),
            0x96 => Instruction(ZeroPageY, STX, 4),
            0x8e => Instruction(Absolute, STX, 4),

            0x84 => Instruction(ZeroPage, STY, 3),
            0x94 => Instruction(ZeroPageX, STY, 4),
            0x8c => Instruction(Absolute, STY, 4),

            0xaa => Instruction(Implied, TAX, 2),
            0xa8 => Instruction(Implied, TAY, 2),
            0xba => Instruction(Implied, TSX, 2),
            0x8a => Instruction(Implied, TXA, 2),
            0x9a => Instruction(Implied, TXS, 2),
            0x98 => Instruction(Implied, TYA, 2),

            0x4b => Instruction(Immediate, ALR, 2),
            0x0b => Instruction(Immediate, ANC, 2),
            0x6b => Instruction(Immediate, ARR, 2),

            0xc7 => Instruction(ZeroPage, DCP, 5),
            0xd7 => Instruction(ZeroPageX, DCP, 6),
            0xcf => Instruction(Absolute, DCP, 6),
            0xdf => Instruction(AbsoluteX, DCP, 7),
            0xdb => Instruction(AbsoluteY, DCP, 7),
            0xc3 => Instruction(IndirectX, DCP, 8),
            0xd3 => Instruction(IndirectY, DCP, 8),

            0xe7 => Instruction(ZeroPage, ISC, 5),
            0xf7 => Instruction(ZeroPageX, ISC, 6),
            0xef => Instruction(Absolute, ISC, 6),
            0xff => Instruction(AbsoluteX, ISC, 7),
            0xfb => Instruction(AbsoluteY, ISC, 7),
            0xe3 => Instruction(IndirectX, ISC, 8),
            0xf3 => Instruction(IndirectY, ISC, 4),

            0xa7 => Instruction(ZeroPage, LAX, 3),
            0xb7 => Instruction(ZeroPageY, LAX, 4),
            0xaf => Instruction(Absolute, LAX, 4),
            0xbf => Instruction(AbsoluteY, LAX, 4),
            0xa3 => Instruction(IndirectX, LAX, 6),
            0xb3 => Instruction(IndirectY, LAX, 5),

            0x27 => Instruction(ZeroPage, RLA, 5),
            0x37 => Instruction(ZeroPageX, RLA, 6),
            0x2f => Instruction(Absolute, RLA, 6),
            0x3f => Instruction(AbsoluteX, RLA, 7),
            0x3b => Instruction(AbsoluteY, RLA, 7),
            0x23 => Instruction(IndirectX, RLA, 8),
            0x33 => Instruction(IndirectY, RLA, 8),

            0x67 => Instruction(ZeroPage, RRA, 5),
            0x77 => Instruction(ZeroPageX, RRA, 6),
            0x6f => Instruction(Absolute, RRA, 6),
            0x7f => Instruction(AbsoluteX, RRA, 7),
            0x7b => Instruction(AbsoluteY, RRA, 7),
            0x63 => Instruction(IndirectX, RRA, 8),
            0x73 => Instruction(IndirectY, RRA, 8),

            0x87 => Instruction(ZeroPage, SAX, 3),
            0x97 => Instruction(ZeroPageY, SAX, 4),
            0x8f => Instruction(Absolute, SAX, 4),
            0x83 => Instruction(IndirectX, SAX, 6),

            0xcb => Instruction(Immediate, AXS, 2),

            0x07 => Instruction(ZeroPage, SLO, 5),
            0x17 => Instruction(ZeroPageX, SLO, 6),
            0x0f => Instruction(Absolute, SLO, 6),
            0x1f => Instruction(AbsoluteX, SLO, 7),
            0x1b => Instruction(AbsoluteY, SLO, 7),
            0x03 => Instruction(IndirectX, SLO, 8),
            0x13 => Instruction(IndirectY, SLO, 8),

            0x47 => Instruction(ZeroPage, SRE, 5),
            0x57 => Instruction(ZeroPageX, SRE, 6),
            0x4f => Instruction(Absolute, SRE, 6),
            0x5f => Instruction(AbsoluteX, SRE, 7),
            0x5b => Instruction(AbsoluteY, SRE, 7),
            0x43 => Instruction(IndirectX, SRE, 8),
            0x53 => Instruction(IndirectY, SRE, 8),

            0x0c => Instruction(Absolute, IGN, 4),
            0x04 | 0x44 | 0x64 => Instruction(ZeroPage, IGN, 3),
            0x80 | 0x82 | 0x89 | 0xc2 | 0xe2 => Instruction(Immediate, SKB, 2),
            0x14 | 0x34 | 0x54 | 0x74 | 0xd4 | 0xf4 => Instruction(ZeroPageX, IGN, 4),
            0x1a | 0x3a | 0x5a | 0x7a | 0xda | 0xfa => Instruction(Implied, NOP, 2),
            0x1c | 0x3c | 0x5c | 0x7c | 0xdc | 0xfc => Instruction(AbsoluteX, IGN, 4),

            _ => panic!("Unknown opcode: {}", op),
        }
    }
}

pub fn exec(op: u8, regs: &mut Registers, mem: &mut Memory) -> u32 {
    let Instruction(mode, opcode, cycles) = Instruction::from(op);
    let Operand(arg, addr, mut extra_cycle) = fetch_operand(regs, mem, mode);

    macro_rules! branch {
        ($cond:expr) => {
            if $cond {
                regs.pc = addr;
                extra_cycle += 1;
            }
        };
    }

    macro_rules! trr {
        ($r1:ident, $r2:ident) => {
            regs.psr.set(Psr::Z, regs.$r1 == 0);
            regs.psr.set(Psr::N, regs.$r1 & 0x80 != 0);
            regs.$r2 = regs.$r1;
        };
    }

    match opcode {
        BCC => branch!(!regs.psr.contains(Psr::C)),
        BCS => branch!(regs.psr.contains(Psr::C)),
        BEQ => branch!(regs.psr.contains(Psr::Z)),
        BMI => branch!(regs.psr.contains(Psr::N)),
        BNE => branch!(!regs.psr.contains(Psr::Z)),
        BPL => branch!(!regs.psr.contains(Psr::N)),
        BVC => branch!(!regs.psr.contains(Psr::V)),
        BVS => branch!(regs.psr.contains(Psr::V)),

        CLC => regs.psr.remove(Psr::C),
        CLD => regs.psr.remove(Psr::D),
        CLI => regs.psr.remove(Psr::I),
        CLV => regs.psr.remove(Psr::V),

        PHA => push(regs, mem, regs.a),
        PHP => push(regs, mem, regs.psr.bits()),
        PLA => { regs.a = pop(regs, mem); }
        PLP => { regs.psr = Psr::from_bits_truncate(pop(regs, mem)); }

        RTI => rti(regs, mem),
        RTS => { regs.pc = pop16(regs, mem).wrapping_add(1); }

        SEC => regs.psr.insert(Psr::C),
        SED => regs.psr.insert(Psr::D),
        SEI => regs.psr.insert(Psr::I),

        STA => mem.write8(addr, regs.a),
        STX => mem.write8(addr, regs.x),
        STY => mem.write8(addr, regs.y),

        TAX => { trr!(a, x); }
        TAY => { trr!(a, y); }
        TXA => { trr!(x, a); }
        TYA => { trr!(y, a); }

        TSX => tsx(regs),
        TXS => { regs.sp = (regs.x as u16) | 0x0100; }

        _ => unimplemented!("{:?}", opcode),
    };

    cycles + extra_cycle
}

fn fetch_operand(regs: &mut Registers, mem: &mut Memory, mode: AddressingMode) -> Operand {
    match mode {
        Implied => Operand(0, 0, 0),
        Accumulator => Operand(regs.a, 0, 0),
        Immediate => Operand(mem.read8(regs.bump()), 0, 0),
        Absolute => {
            let addr = mem.read16(regs.pc);
            regs.pc = regs.pc.wrapping_add(2);
            Operand(mem.read8(addr), addr, 0)
        }
        ZeroPage => {
            let addr = mem.read8(regs.bump()) as u16;
            Operand(mem.read8(addr), addr, 0)
        }
        Indirect => {
            let abs_addr = mem.read16(regs.pc);
            regs.pc = regs.pc.wrapping_add(2);
            Operand(0, mem.read16(abs_addr), 0)
        }
        ZeroPageX => {
            let abs_addr = u16::from(mem.read8(regs.bump()).wrapping_add(regs.x));
            Operand(mem.read8(abs_addr), abs_addr, 0)
        }
        ZeroPageY => {
            let abs_addr = u16::from(mem.read8(regs.bump()).wrapping_add(regs.y));
            Operand(mem.read8(abs_addr), abs_addr, 0)
        }
        _ => unimplemented!(),
    }
}

fn push(regs: &mut Registers, mem: &mut Memory, val: u8) {
    mem.write8(regs.sp, val);
    regs.sp -= 1;
}

fn pop(regs: &mut Registers, mem: &mut Memory) -> u8 {
    regs.sp += 1;
    mem.read8(regs.sp)
}

fn pop16(regs: &mut Registers, mem: &mut Memory) -> u16 {
    let lo = pop(regs, mem);
    let hi = pop(regs, mem);
    ((hi as u16) << 8) | (lo as u16)
}

fn rti(regs: &mut Registers, mem: &mut Memory) {
    regs.psr = Psr::from_bits_truncate(pop(regs, mem));
    regs.pc = pop16(regs, mem);
}

fn tsx(regs: &mut Registers) {
    let sp = regs.sp as u8;
    regs.psr.set(Psr::Z, sp == 0);
    regs.psr.set(Psr::N, sp & 0x80 != 0);
    regs.x = sp;
}
