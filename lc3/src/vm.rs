use crate::{
    cond::Cond,
    helpers::{sign_extend, words_from_bytes},
    instructions::{BinaryOp, Instruction::{self, *}, TrapVec},
};

const MEMORY_SIZE: usize = 1 << 16;
const REGISTER_COUNT: usize = 8;

pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: [u16; REGISTER_COUNT],
    pub pc: u16,
    pub cond: Cond,
    pub running: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            pc: 0,
            cond: Cond::default(),
            running: false,
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
            Not { dest_reg, source_reg } => self.execute_not(dest_reg, source_reg),
            Branch { branch_neg: n, branch_zero: z, branch_pos: p, pc_offset } => self.execute_branch(n, z, p, pc_offset),
            Load { dest_reg, pc_offset } => self.execute_load(dest_reg, pc_offset),
            LoadEffectiveAddress { dest_reg, pc_offset } => self.execute_load_effective_address(dest_reg, pc_offset),
            Store { source_reg, pc_offset } => self.execute_store(source_reg, pc_offset),
            Jump(register) => self.execute_jump(register),
            Trap(trap_vec) => self.execute_trap(trap_vec),
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

    pub fn load_words(&mut self, origin: u16, words: &[u16]) -> Result<(), String> {
        let start = origin as usize;
        for (i, &word) in words.iter().enumerate() {
            self.memory[start + i] = word;
        }

        self.pc = origin;
        Ok(())
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
            BinaryOp::Immediate { dest_reg: _, source_reg1: _, immediate } => sign_extend(*immediate as u16, 5),
            BinaryOp::Register { dest_reg: _, source_reg1: _, source_reg2 } => self.registers[*source_reg2],
        };

        (op1, op2)
    }

    fn execute_trap(&mut self, vec: TrapVec) {
        match vec {
            TrapVec::Halt => {
                self.running = false;
            }
        }
    }
    
    fn execute_not(&mut self, dest_reg: usize, source_reg: usize) {
        let value = !self.registers[source_reg];
        self.set_register(value, dest_reg);
    }
    
    fn execute_branch(&mut self, branch_neg: bool, branch_zero: bool, branch_pos: bool, pc_offset: u16) {
        let should_branch = match self.cond {
            Cond::Negative => branch_neg,
            Cond::Zero => branch_zero,
            Cond::Positive => branch_pos,
        };

        if should_branch {
            self.pc = self.pc.wrapping_add(pc_offset);
        }
    }
    
    fn execute_load(&mut self, dest_reg: usize, pc_offset: u16) {
        let address = self.pc.wrapping_add(pc_offset);
        let value = self.memory[address as usize];
        self.set_register(value, dest_reg);
    }
    
    fn execute_load_effective_address(&mut self, dest_reg: usize, pc_offset: u16) {
        let address = self.pc.wrapping_add(pc_offset);
        self.set_register(address, dest_reg);
    }
    
    fn execute_store(&mut self, source_reg: usize, pc_offset: u16) {
        let address = self.pc.wrapping_add(pc_offset) as usize;
        let value = self.registers[source_reg];
        self.memory[address] = value;
    }
    
    fn execute_jump(&mut self, register: usize) {
        let address = self.registers[register];
        self.pc = address;
    }
}
