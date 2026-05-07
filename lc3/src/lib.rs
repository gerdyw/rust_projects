pub mod cond;
pub mod helpers;
pub mod instructions;
pub mod vm;

#[cfg(test)]
mod tests {
    use crate::{cond::Cond, vm::VM, *};

    #[test]
    fn add_immediate_positive() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x1021, // ADD R0, R0, #1
                0x1021, // ADD R0, R0, #1
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 2);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn add_immediate_zero() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x1021, // ADD R0, R0, #1
                0x103F, // ADD R0, R0, #-1
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0);
        assert_eq!(vm.cond, Cond::Zero);
    }

    #[test]
    fn add_immediate_negative() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0xFFFF);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn and_immediate_clears_register() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x102F, // ADD R0, R0, #15   ; R0 = 15
                0x5020, // AND R0, R0, #0    ; R0 = 0
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0);
        assert_eq!(vm.cond, Cond::Zero);
    }

    #[test]
    fn and_register_mode() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x102F, // ADD R0, R0, #15   ; R0 = 0b1111
                0x126A, // ADD R1, R1, #10   ; R1 = 0b1010
                0x5401, // AND R2, R0, R1    ; R2 = 0b1010
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 15);
        assert_eq!(vm.registers[1], 10);
        assert_eq!(vm.registers[2], 10);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn and_immediate_negative() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1   ; R0 = 0xFFFF
                0x503F, // AND R0, R0, #-1   ; R0 = 0xFFFF
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0xFFFF);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn and_immediate_masks_low_bits() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x102F, // ADD R0, R0, #15   ; R0 = 0b1111
                0x5025, // AND R0, R0, #5    ; R0 = 0b0101
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 5);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn not_zero_becomes_all_ones() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x903F, // NOT R0, R0
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0xFFFF);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn not_all_ones_becomes_zero() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1 ; R0 = 0xFFFF
                0x903F, // NOT R0, R0      ; R0 = 0
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0);
        assert_eq!(vm.cond, Cond::Zero);
    }

    #[test]
    fn br_not_taken_when_condition_does_not_match() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x1021, // ADD R0, R0, #1 ; cond = Positive
                0x0401, // BRz +1         ; should not branch
                0x1261, // ADD R1, R1, #1 ; should execute
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[1], 1);
    }

    #[test]
    fn br_taken_when_condition_matches() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x1021, // ADD R0, R0, #1 ; cond = Positive
                0x0201, // BRp +1         ; skip next instruction
                0x1261, // ADD R1, R1, #1 ; skipped
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[1], 0);
    }

    #[test]
    fn ld_loads_pc_relative_value() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x2001, // LD R0, +1     ; R0 = memory[x3002]
                0xF025, // HALT
                0x1234, // data
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0x1234);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn ld_updates_negative_flag() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x2001, // LD R0, +1     ; R0 = memory[x3002]
                0xF025, // HALT
                0xFFFF, // data = -1
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0xFFFF);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn st_stores_pc_relative_value() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x102A, // ADD R0, R0, #10 ; R0 = 10
                0x3001, // ST R0, +1       ; memory[x3003] = R0
                0xF025, // HALT
                0x0000, // storage slot
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3003], 10);
    }

    #[test]
    fn lea_loads_address_not_value() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xE001, // LEA R0, +1    ; R0 = x3002
                0xF025, // HALT
                0xABCD, // data, should not be loaded
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0x3002);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn jmp_sets_pc_to_address_in_base_register() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xE401, // LEA R2, +1      ; R2 = x3002
                0xC080, // JMP R2          ; PC = R2
                0x1021, // ADD R0, R0, #1  ; runs after jump
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[2], 0x3002);
        assert_eq!(vm.registers[0], 1);
    }

    #[test]
    fn jmp_can_skip_over_instruction() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xE402, // LEA R2, +2      ; R2 = x3003
                0xC080, // JMP R2          ; jump to HALT
                0x1021, // ADD R0, R0, #1  ; skipped
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[2], 0x3003);
        assert_eq!(vm.registers[0], 0);
    }

    #[test]
    fn ret_is_just_jmp_r7() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xEE01, // LEA R7, +1      ; R7 = x3002
                0xC1C0, // JMP R7 / RET    ; PC = R7
                0x1021, // ADD R0, R0, #1  ; runs after return
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[7], 0x3002);
        assert_eq!(vm.registers[0], 1);
    }

    #[test]
    fn jmp_does_not_update_condition_flags() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1 ; R0 = xFFFF, cond = Negative
                0xE402, // LEA R2, +1      ; R2 = x3003, cond = Positive
                0x103F, // ADD R0, R0, #-1 ; R0 = xFFFE, cond = Negative
                0xC080, // JMP R2          ; PC = R2, cond should remain Negative
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.cond, Cond::Negative);
    }
}
