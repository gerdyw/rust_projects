use lc3::vm::VM;

fn main() -> Result<(), String> {
    println!("Starting LC-3");
    let mut vm = VM::new();

    let _ = vm.load_words(
        0x3000,
        &[
            0x1021, // ADD R0, R0, #1
            0x1021, // ADD R0, R0, #1
            0xF025, // HALT
        ],
    );

    vm.run();

    assert_eq!(vm.registers[0], 2);
    Ok(())
}

