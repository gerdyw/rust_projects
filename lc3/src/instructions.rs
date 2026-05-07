use crate::helpers::{bit, bits, sign_extend};

const DEST_REG: u8 = 9;
const SOURCE_REG1: u8 = 6;
const SOURCE_REG2: u8 = 0;

const IMMEDIATE: u8 = 0;
const IMMEDIATE_LEN: u8 = 5;
const IMMEDIATE_MODE: u8 = 5;

const REG_LEN: u8 = 3;
const PC_OFFSET: u8 = 0;
const PC_OFFSET_LEN: u8 = 9;

const BASE_REG: u8 = 6;

const P: u8 = 9;
const Z: u8 = 10;
const N: u8 = 11;

#[derive(Debug)]
pub enum Instruction {
    Add(BinaryOp),
    And(BinaryOp),
    Not {
        dest_reg: usize,
        source_reg: usize,
    },
    Branch {
        branch_neg: bool,
        branch_zero: bool,
        branch_pos: bool,
        pc_offset: u16,
    },
    Load {
        dest_reg: usize,
        pc_offset: u16,
    },
    LoadEffectiveAddress {
        dest_reg: usize,
        pc_offset: u16,
    },
    Store {
        source_reg: usize,
        pc_offset: u16,
    },
    Jump(usize),
    Trap(TrapVec),
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let opcode = value >> 12;
        match opcode {
            0b0001 => Instruction::Add(value.into()),
            0b0101 => Instruction::And(value.into()),
            0b1001 => Instruction::Not {
                dest_reg: extract_dest_reg(value),
                source_reg: extract_source_reg1(value),
            },
            0b0000 => Instruction::Branch {
                branch_neg: bit(value, N),
                branch_zero: bit(value, Z),
                branch_pos: bit(value, P),
                pc_offset: extract_pc_offset(value),
            },
            0b0010 => Instruction::Load {
                dest_reg: extract_dest_reg(value),
                pc_offset: extract_pc_offset(value),
            },
            0b1110 => Instruction::LoadEffectiveAddress {
                dest_reg: extract_dest_reg(value),
                pc_offset: extract_pc_offset(value),
            },
            0b0011 => Instruction::Store {
                source_reg: extract_store_source_reg(value),
                pc_offset: extract_pc_offset(value),
            },
            0b1100 => Instruction::Jump(extract_reg(value, BASE_REG)),
            0b1111 => Instruction::Trap(value.into()),
            _ => panic!("unimplemented opcode: {opcode:x}"),
        }
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Register {
        dest_reg: usize,
        source_reg1: usize,
        source_reg2: usize,
    },
    Immediate {
        dest_reg: usize,
        source_reg1: usize,
        immediate: u8,
    },
}

impl BinaryOp {
    pub fn dest_reg(&self) -> usize {
        match self {
            BinaryOp::Immediate {
                dest_reg,
                source_reg1: _,
                immediate: _,
            } => *dest_reg,
            BinaryOp::Register {
                dest_reg,
                source_reg1: _,
                source_reg2: _,
            } => *dest_reg,
        }
    }

    pub fn source_reg1(&self) -> usize {
        match self {
            BinaryOp::Immediate {
                dest_reg: _,
                source_reg1,
                immediate: _,
            } => *source_reg1,
            BinaryOp::Register {
                dest_reg: _,
                source_reg1,
                source_reg2: _,
            } => *source_reg1,
        }
    }
}

impl From<u16> for BinaryOp {
    fn from(value: u16) -> Self {
        let dest_reg = extract_dest_reg(value);
        let sr1 = extract_source_reg1(value);
        let immediate_mode = bits(value, IMMEDIATE_MODE, 1) != 0;

        if immediate_mode {
            let immediate = bits(value, IMMEDIATE, IMMEDIATE_LEN);
            BinaryOp::Immediate {
                dest_reg,
                source_reg1: sr1,
                immediate: immediate.try_into().unwrap(),
            }
        } else {
            let sr2 = extract_source_reg2(value);
            BinaryOp::Register {
                dest_reg,
                source_reg1: sr1,
                source_reg2: sr2,
            }
        }
    }
}

#[derive(Debug)]
pub enum TrapVec {
    Halt = 0x25,
}

impl From<u16> for TrapVec {
    fn from(instr: u16) -> Self {
        let trap_vec = instr & 0x00FF;

        match trap_vec {
            0x25 => TrapVec::Halt,
            _ => panic!("unimplemented trap: x{trap_vec:02X}"),
        }
    }
}

fn extract_reg(value: u16, reg_start: u8) -> usize {
    bits(value, reg_start, REG_LEN) as usize
}

fn extract_source_reg1(value: u16) -> usize {
    extract_reg(value, SOURCE_REG1)
}

fn extract_source_reg2(value: u16) -> usize {
    extract_reg(value, SOURCE_REG2)
}

fn extract_dest_reg(value: u16) -> usize {
    extract_reg(value, DEST_REG)
}

fn extract_pc_offset(value: u16) -> u16 {
    sign_extend(bits(value, PC_OFFSET, PC_OFFSET_LEN), PC_OFFSET_LEN)
}

fn extract_store_source_reg(value: u16) -> usize {
    extract_reg(value, DEST_REG)
}
