use crate::helpers::{bit, bits, bits_extended};

const DEST_REG: u8 = 9;
const SOURCE_REG1: u8 = 6;
const SOURCE_REG2: u8 = 0;

const IMMEDIATE: u8 = 0;
const IMMEDIATE_LEN: u8 = 5;
const IMMEDIATE_MODE: u8 = 5;

const REG_LEN: u8 = 3;
const BASE_REG: u8 = 6;

const P: u8 = 9;
const Z: u8 = 10;
const N: u8 = 11;

#[derive(Debug)]
pub enum Instruction {
    Add(BinaryOp),
    And(BinaryOp),
    Not(NotArgs),
    Branch(BranchArgs),
    Load(LoadOp),
    LoadEffectiveAddress(LeaArgs),
    Store(StoreOp),
    Jump(usize),
    JumpSubroutine(JumpSubroutine),
    Trap(TrapVec),
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        let opcode = value >> 12;
        match opcode {
            0b0001 => Self::Add(value.into()),
            0b0101 => Self::And(value.into()),
            0b1001 => Self::Not(value.into()),
            0b0000 => Self::Branch(value.into()),
            0b0010 => Self::Load(LoadOp::PcRelative(value.into())),
            0b0110 => Self::Load(LoadOp::BaseRelative(value.into())),
            0b1010 => Self::Load(LoadOp::Indirect(value.into())),
            0b1110 => Self::LoadEffectiveAddress(value.into()),
            0b0011 => Self::Store(StoreOp::PcRelative(value.into())),
            0b0111 => Self::Store(StoreOp::BaseRelative(value.into())),
            0b1011 => Self::Store(StoreOp::Indirect(value.into())),
            0b1100 => Self::Jump(extract_reg(value, BASE_REG)),
            0b0100 => Self::JumpSubroutine(value.into()),
            0b1111 => Self::Trap(value.into()),
            _ => panic!("unimplemented opcode: {opcode:x}"),
        }
    }
}

#[derive(Debug)]
pub enum LoadOp {
    PcRelative(LoadArgs),
    BaseRelative(LoadRelativeArgs),
    Indirect(LoadIndirectArgs),
}


impl LoadOp {
    pub fn dest_reg(&self) -> usize {
        match self {
            Self::PcRelative(args) => args.dest_reg,
            Self::BaseRelative(args) => args.dest_reg,
            Self::Indirect(args) => args.dest_reg,
        }
    }
}

#[derive(Debug)]
pub enum StoreOp {
    PcRelative(StoreArgs),
    BaseRelative(StoreRelativeArgs),
    Indirect(StoreIndirectArgs),
}

impl StoreOp {
    pub fn source_reg(&self) -> usize {
        match self {
            Self::BaseRelative(args) => args.source_reg,
            Self::PcRelative(args) => args.source_reg,
            Self::Indirect(args) => args.source_reg,
        }
    }
}

#[derive(Debug)]
pub struct NotArgs {
    pub dest_reg: usize,
    pub source_reg: usize,
}

impl From<u16> for NotArgs {
    fn from(value: u16) -> Self {
        Self {
            dest_reg: extract_dest_reg(value),
            source_reg: extract_source_reg1(value),
        }
    }
}

#[derive(Debug)]
pub struct BranchArgs {
    pub branch_neg: bool,
    pub branch_zero: bool,
    pub branch_pos: bool,
    pub pc_offset: u16,
}

impl From<u16> for BranchArgs {
    fn from(value: u16) -> Self {
        Self {
            branch_neg: bit(value, N),
            branch_zero: bit(value, Z),
            branch_pos: bit(value, P),
            pc_offset: bits_extended(value, 0, 9),
        }
    }
}

#[derive(Debug)]
pub struct LoadArgs {
    pub dest_reg: usize,
    pub pc_offset: u16,
}


impl From<u16> for LoadArgs {
    fn from(value: u16) -> Self {
        Self {
            dest_reg: extract_dest_reg(value),
            pc_offset: bits_extended(value, 0, 9),
        }
    }
}

#[derive(Debug)]
pub struct LeaArgs {
    pub dest_reg: usize,
    pub pc_offset: u16,
}

impl From<u16> for LeaArgs {
    fn from(value: u16) -> Self {
        Self {
            dest_reg: extract_dest_reg(value),
            pc_offset: bits_extended(value, 0, 9),
        }
    }
}

#[derive(Debug)]
pub struct LoadRelativeArgs {
    pub dest_reg: usize,
    pub base_reg: usize,
    pub offset: u16,
}

impl From<u16> for LoadRelativeArgs {
    fn from(value: u16) -> Self {
        Self {
            dest_reg: extract_dest_reg(value),
            base_reg: extract_base_reg(value),
            offset: bits_extended(value, 0, 6),
        }
    }
}

#[derive(Debug)]
pub struct LoadIndirectArgs {
    pub dest_reg: usize,
    pub pc_offset: u16,
}

impl From<u16> for LoadIndirectArgs {
    fn from(value: u16) -> Self {
        Self {
            dest_reg: extract_dest_reg(value),
            pc_offset: bits_extended(value, 0, 9),
        }
    }
}

#[derive(Debug)]
pub struct StoreArgs {
    pub source_reg: usize,
    pub pc_offset: u16,
}

impl From<u16> for StoreArgs {
    fn from(value: u16) -> Self {
        Self {
            source_reg: extract_base_reg(value),
            pc_offset: bits_extended(value, 0, 9),
        }
    }
}

#[derive(Debug)]
pub struct StoreRelativeArgs {
    pub source_reg: usize,
    pub base_reg: usize,
    pub offset: u16,
}

impl From<u16> for StoreRelativeArgs {
    fn from(value: u16) -> Self {
        Self {
            source_reg: extract_dest_reg(value),
            base_reg: extract_base_reg(value),
            offset: bits_extended(value, 0, 6),
        }
    }
}

#[derive(Debug)]
pub struct StoreIndirectArgs {
    pub source_reg: usize,
    pub pc_offset: u16,
}

impl From<u16> for StoreIndirectArgs {
    fn from(value: u16) -> Self {
        Self {
            source_reg: extract_base_reg(value),
            pc_offset: bits_extended(value, 0, 9)
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
pub enum JumpSubroutine {
    Relative(u16),
    Register(usize),
}

impl From<u16> for JumpSubroutine {
    fn from(value: u16) -> Self {
        match bit(value, 11) {
            true => Self::Relative(bits_extended(value, 0, 11)),
            false => Self::Register(extract_base_reg(value)),
        }
    }
}

#[derive(Debug)]
pub enum TrapVec {
    GetC = 0x20,
    Out = 0x21,
    PutS = 0x22,
    In = 0x23,
    PutSP = 0x24,
    Halt = 0x25,
}

impl From<u16> for TrapVec {
    fn from(instr: u16) -> Self {
        let trap_vec = instr & 0x00FF;

        match trap_vec {
            0x20 => TrapVec::GetC,
            0x21 => TrapVec::Out,
            0x22 => TrapVec::PutS,
            0x23 => TrapVec::In,
            0x24 => TrapVec::PutSP,
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

fn extract_base_reg(value: u16) -> usize {
    extract_reg(value, BASE_REG)
}
