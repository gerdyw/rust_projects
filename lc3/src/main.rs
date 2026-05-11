use std::{env, fs};

use lc3::vm::VM;

fn main() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let obj_path = args
        .next()
        .ok_or_else(|| "usage: lc3 <object-file>".to_string())?;

    if args.next().is_some() {
        return Err("usage: lc3 <object-file>".to_string());
    }

    println!("Starting LC-3 with {obj_path}");
    let mut vm = VM::new();

    let bytes = fs::read(&obj_path).map_err(|e| format!("failed to read {obj_path}: {e}"))?;
    let origin = parse_origin(&bytes)?;
    vm.load_obj_bytes(origin, &bytes)?;

    vm.run();

    Ok(())
}

fn parse_origin(bytes: &[u8]) -> Result<u16, String> {
    if bytes.len() < 2 {
        return Err("object file must be at least 2 bytes".to_string());
    }

    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

