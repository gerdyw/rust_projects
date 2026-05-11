use std::{collections::VecDeque, io};

use crate::{
    cond::Cond,
    helpers::{ToChar, sign_extend, words_from_bytes},
    instructions::{
        BinaryOp, BranchArgs,
        Instruction::{self, *},
        JumpSubroutine, LeaArgs, LoadOp, NotArgs, StoreOp, TrapVec,
    },
};

const MEMORY_SIZE: usize = 1 << 16;
const REGISTER_COUNT: usize = 8;

pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: [u16; REGISTER_COUNT],
    pub pc: u16,
    pub cond: Cond,
    pub running: bool,
    pub input: VecDeque<u8>,
    pub output: String,
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            pc: 0,
            cond: Cond::default(),
            running: false,
            input: VecDeque::new(),
            output: "".into(),
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            let instr: Instruction = self.memory[self.pc as usize].into();
            self.pc = self.pc.wrapping_add(1);

            self.execute(instr);
        }
    }

    fn execute(&mut self, instr: Instruction) {
        match instr {
            Add(op) => self.execute_add(op),
            And(op) => self.execute_and(op),
            Not(op) => self.execute_not(op),
            Branch(args) => self.execute_branch(args),
            Load(op) => self.execute_load(op),
            LoadEffectiveAddress(args) => self.load_effective_address(args),
            Store(op) => self.execute_store(op),
            Jump(register) => self.execute_jump(register),
            Trap(trap_vec) => self.execute_trap(trap_vec),
            JumpSubroutine(jsr) => self.execute_jump_subroutine(jsr),
        };
    }

    pub fn load_obj_bytes(&mut self, origin: u16, bytes: &[u8]) -> Result<(), String> {
        let start = origin as usize;
        let words = words_from_bytes(bytes)?;
        for (i, &word) in words.iter().enumerate() {
            self.memory[start + i] = word;
        }

        self.pc = origin;
        Ok(())
    }

    pub fn load_words(&mut self, origin: u16, words: &[u16]) {
        let start = origin as usize;
        for (i, &word) in words.iter().enumerate() {
            self.memory[start + i] = word;
        }

        self.pc = origin;
    }

    fn execute_add(&mut self, add: BinaryOp) {
        let (op1, op2) = self.extract_operands(&add);
        let result = op1.wrapping_add(op2);
        let dest_reg = add.dest_reg();
        self.set_register(result, dest_reg);
    }

    fn execute_and(&mut self, and: BinaryOp) {
        let (op1, op2) = self.extract_operands(&and);
        let result = op1 & op2;
        let dest_reg = and.dest_reg();
        self.set_register(result, dest_reg);
    }

    fn set_register(&mut self, result: u16, dest_reg: usize) {
        self.registers[dest_reg] = result;
        self.update_flags(dest_reg);
    }

    fn update_flags(&mut self, register: usize) {
        let value = self.registers[register];

        self.cond = if value == 0 {
            Cond::Zero
        } else if (value & 0x8000) != 0 {
            Cond::Negative
        } else {
            Cond::Positive
        };
    }

    fn extract_operands(&self, operand: &BinaryOp) -> (u16, u16) {
        let source_reg1 = operand.source_reg1();
        let op1 = self.registers[source_reg1];
        let op2 = match operand {
            BinaryOp::Immediate {
                dest_reg: _,
                source_reg1: _,
                immediate,
            } => sign_extend(*immediate as u16, 5),
            BinaryOp::Register {
                dest_reg: _,
                source_reg1: _,
                source_reg2,
            } => self.registers[*source_reg2],
        };

        (op1, op2)
    }

    fn execute_not(&mut self, not: NotArgs) {
        let value = !self.registers[not.source_reg];
        self.set_register(value, not.dest_reg);
    }

    fn execute_branch(&mut self, branch: BranchArgs) {
        let should_branch = match self.cond {
            Cond::Negative => branch.branch_neg,
            Cond::Zero => branch.branch_zero,
            Cond::Positive => branch.branch_pos,
        };

        if should_branch {
            self.pc = self.pc.wrapping_add(branch.pc_offset);
        }
    }

    fn execute_load(&mut self, load: LoadOp) {
        let dest_reg = load.dest_reg();

        let address = match load {
            LoadOp::PcRelative(args) => self.pc.wrapping_add(args.pc_offset),
            LoadOp::BaseRelative(args) => {
                let base = self.registers[args.base_reg];
                base.wrapping_add(args.offset)
            }
            LoadOp::Indirect(args) => {
                let pointer_address = self.pc.wrapping_add(args.pc_offset) as usize;
                self.memory[pointer_address]
            }
        } as usize;

        let value = self.memory[address];
        self.set_register(value, dest_reg);
    }

    fn execute_store(&mut self, store: StoreOp) {
        let source_reg = store.source_reg();
        let value = self.registers[source_reg];

        let dest_address = match store {
            StoreOp::PcRelative(args) => self.pc.wrapping_add(args.pc_offset),
            StoreOp::BaseRelative(args) => self.registers[args.base_reg].wrapping_add(args.offset),
            StoreOp::Indirect(args) => {
                let pointer_address = self.pc.wrapping_add(args.pc_offset) as usize;
                self.memory[pointer_address]
            }
        } as usize;

        self.memory[dest_address] = value;
    }

    fn execute_jump(&mut self, register: usize) {
        let address = self.registers[register];
        self.pc = address;
    }

    fn execute_jump_subroutine(&mut self, jsr: JumpSubroutine) {
        self.registers[7] = self.pc;
        self.pc = match jsr {
            JumpSubroutine::Relative(offset) => self.pc.wrapping_add(offset),
            JumpSubroutine::Register(register) => self.registers[register],
        }
    }

    fn load_effective_address(&mut self, args: LeaArgs) {
        let address = self.pc.wrapping_add(args.pc_offset);
        self.registers[args.dest_reg] = address;
    }

    fn execute_trap(&mut self, vec: TrapVec) {
        match vec {
            TrapVec::GetC => self.execute_trap_getc(),
            TrapVec::Out => self.execute_trap_out(),
            TrapVec::PutS => self.execute_trap_puts(),
            TrapVec::In => self.execute_trap_in(),
            TrapVec::PutSP => self.execute_trap_putsp(),
            TrapVec::Halt => self.execute_trap_halt(),
        }
    }

    fn execute_trap_getc(&mut self) {
        let c = self.read_char();
        self.set_register(c as u16, 0);
    }

    fn execute_trap_out(&mut self) {
        let ch = self.registers[0] as u8 as char;
        self.out(ch);
    }

    fn execute_trap_puts(&mut self) {
        let mut addr = self.registers[0];
        loop {
            let ch = self.memory[addr as usize] as u8 as char;

            if ch == '\0' {
                break;
            }

            self.out(ch);
            addr = addr.wrapping_add(1);
        }
        println!();
    }

    fn execute_trap_in(&mut self) {
        let c = self.read_char();
        self.set_register(c as u16, 0);
        self.out(c);
    }

    fn execute_trap_putsp(&mut self) {
        let mut addr = self.registers[0];

        loop {
            let word = self.memory[addr as usize];

            if word == 0 {
                break;
            }
            let ch = word.to_char();
            self.out(ch);

            let ch = (word >> 8).to_char();
            if ch != '\0' {
                self.out(ch);
            }

            addr = addr.wrapping_add(1);
        }
    }

    fn execute_trap_halt(&mut self) {
        self.running = false;
    }

    fn out(&mut self, ch: char) {
        print!("{ch}");
        self.output.push(ch);
    }

    fn read_char(&mut self) -> char {
        if let Some(ch) = self.input.pop_front() {
            return ch as char;
        };

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Awaiting a line");
        self.input.extend(line.bytes());

        self.input
            .pop_front()
            .expect("expected at least one byte of input") as char
    }
}
