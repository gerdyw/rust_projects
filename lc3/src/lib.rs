pub mod cond;
pub mod helpers;
pub mod instructions;
pub mod vm;

#[cfg(test)]
mod tests {
    use crate::{cond::Cond, vm::VM};

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

    #[test]
    fn jsr_jumps_to_pc_relative_subroutine_and_sets_r7() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x4802, // JSR +2          ; R7 = x3001, PC = x3003
                0x1021, // ADD R0, R0, #1  ; runs after return
                0xF025, // HALT
                0x1261, // ADD R1, R1, #1  ; subroutine body
                0xC1C0, // JMP R7 / RET    ; return to x3001
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[7], 0x3001);
        assert_eq!(vm.registers[0], 1);
        assert_eq!(vm.registers[1], 1);
    }

    #[test]
    fn jsrr_jumps_to_address_in_base_register_and_sets_r7() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xE402, // LEA R2, +2      ; R2 = x3003
                0x4080, // JSRR R2         ; R7 = x3002, PC = R2
                0xF025, // HALT            ; runs after return
                0x1261, // ADD R1, R1, #1  ; subroutine body
                0xC1C0, // JMP R7 / RET    ; return to x3002
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[2], 0x3003);
        assert_eq!(vm.registers[7], 0x3002);
        assert_eq!(vm.registers[1], 1);
    }

    #[test]
    fn jsr_does_not_update_condition_flags() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1 ; R0 = xFFFF, cond = Negative
                0x4801, // JSR +1          ; R7 = x3002, PC = x3003
                0xF025, // HALT            ; runs after return
                0xC1C0, // JMP R7 / RET    ; return to x3002
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0xFFFF);
        assert_eq!(vm.registers[7], 0x3002);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn jsrr_does_not_update_condition_flags() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1 ; R0 = xFFFF, cond = Negative
                0xE402, // LEA R2, +2      ; R2 = x3004, cond = Positive
                0x103F, // ADD R0, R0, #-1 ; R0 = xFFFE, cond = Negative
                0x4080, // JSRR R2         ; R7 = x3004, PC = R2
                0xF025, // HALT
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0xFFFE);
        assert_eq!(vm.registers[2], 0x3004);
        assert_eq!(vm.registers[7], 0x3004);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn ldr_loads_from_base_register_plus_positive_offset() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xE203, // LEA R1, +3      ; R1 = x3004
                0x6040, // LDR R0, R1, #0  ; R0 = memory[x3004]
                0xF025, // HALT
                0x0000, // padding
                0x1234, // data
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[1], 0x3004);
        assert_eq!(vm.registers[0], 0x1234);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn ldr_loads_from_base_register_plus_nonzero_offset() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xE202, // LEA R1, +2      ; R1 = x3003
                0x6041, // LDR R0, R1, #1  ; R0 = memory[x3004]
                0xF025, // HALT
                0x0000, // padding
                0xABCD, // data
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[1], 0x3003);
        assert_eq!(vm.registers[0], 0xABCD);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn ldr_supports_negative_offset() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xE203, // LEA R1, +3       ; R1 = x3004
                0x607F, // LDR R0, R1, #-1  ; R0 = memory[x3003]
                0xF025, // HALT
                0x1234, // data
                0x0000, // base points here
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[1], 0x3004);
        assert_eq!(vm.registers[0], 0x1234);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn str_stores_to_base_register_plus_offset() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x102A, // ADD R0, R0, #10  ; R0 = 10
                0xE203, // LEA R1, +3       ; R1 = x3005
                0x7040, // STR R0, R1, #0   ; memory[x3005] = R0
                0xF025, // HALT
                0x0000, // padding
                0x0000, // storage slot
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3005], 10);
    }

    #[test]
    fn str_supports_negative_offset() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x102A, // ADD R0, R0, #10  ; R0 = 10
                0xE203, // LEA R1, +3       ; R1 = x3005
                0x707F, // STR R0, R1, #-1  ; memory[x3004] = R0
                0xF025, // HALT
                0x0000, // storage slot
                0x0000, // base points here
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3004], 10);
    }

    #[test]
    fn str_does_not_update_condition_flags() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1  ; R0 = xFFFF, cond = Negative
                0xE203, // LEA R1, +3       ; R1 = x3005, cond = Positive
                0x103F, // ADD R0, R0, #-1  ; R0 = xFFFE, cond = Negative
                0x7040, // STR R0, R1, #0   ; should not update cond
                0xF025, // HALT
                0x0000, // storage slot
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3005], 0xFFFE);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn str_then_ldr_round_trip() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x1027, // ADD R0, R0, #7   ; R0 = 7
                0xE204, // LEA R1, +4       ; R1 = x3006
                0x7040, // STR R0, R1, #0   ; memory[x3006] = 7
                0x5480, // AND R2, R2, #0   ; R2 = 0
                0x6440, // LDR R2, R1, #0   ; R2 = memory[x3006]
                0xF025, // HALT
                0x0000, // storage slot
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3006], 7);
        assert_eq!(vm.registers[2], 7);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn ldi_loads_through_pc_relative_pointer() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xA001, // LDI R0, +1      ; pointer_addr = x3002, final_addr = memory[x3002]
                0xF025, // HALT
                0x3003, // pointer to data
                0x1234, // data
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0x1234);
        assert_eq!(vm.cond, Cond::Positive);
    }

    #[test]
    fn ldi_updates_negative_flag() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xA001, // LDI R0, +1      ; R0 = memory[memory[x3002]]
                0xF025, // HALT
                0x3003, // pointer to data
                0xFFFF, // data = -1
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0xFFFF);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn ldi_updates_zero_flag() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0xA001, // LDI R0, +1      ; R0 = memory[memory[x3002]]
                0xF025, // HALT
                0x3003, // pointer to data
                0x0000, // data = 0
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.registers[0], 0x0000);
        assert_eq!(vm.cond, Cond::Zero);
    }

    #[test]
    fn sti_stores_through_pc_relative_pointer() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x102A, // ADD R0, R0, #10 ; R0 = 10
                0xB001, // STI R0, +1      ; memory[memory[x3003]] = R0
                0xF025, // HALT
                0x3004, // pointer to storage slot
                0x0000, // storage slot
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3004], 10);
    }

    #[test]
    fn sti_does_not_update_condition_flags() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x103F, // ADD R0, R0, #-1 ; R0 = xFFFF, cond = Negative
                0xB001, // STI R0, +1      ; memory[memory[x3003]] = R0
                0xF025, // HALT
                0x3004, // pointer to storage slot
                0x0000, // storage slot
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3004], 0xFFFF);
        assert_eq!(vm.cond, Cond::Negative);
    }

    #[test]
    fn sti_then_ldi_round_trip() {
        let mut vm = VM::new();

        vm.load_words(
            0x3000,
            &[
                0x1027, // ADD R0, R0, #7  ; R0 = 7
                0xB003, // STI R0, +3      ; memory[memory[x3005]] = R0
                0x5480, // AND R2, R2, #0  ; R2 = 0
                0xA201, // LDI R1, +1      ; R1 = memory[memory[x3005]]
                0xF025, // HALT
                0x3006, // pointer to storage slot
                0x0000, // storage slot
            ],
        )
        .unwrap();

        vm.run();

        assert_eq!(vm.memory[0x3006], 7);
        assert_eq!(vm.registers[1], 7);
        assert_eq!(vm.cond, Cond::Positive);
    }
}
