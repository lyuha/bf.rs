mod bf;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::BTreeMap;

use self::bf::BFMemory;

fn main() {
    for arg in env::args().skip(1) {
        if let Ok(mut f) = File::open(&arg) {
            println!("\nFile '{}' output", arg);
            let mut buffer = String::new();
            if let Err(e) = f.read_to_string(&mut buffer) {
                // err
            }

            let source = buffer.chars()
                .filter(|&x| {
                    match x {
                        '+' | '-' | '<' | '>' | '[' | ']' | ',' | '.' => true,
                        _ => false,
                    }
                })
                .collect::<String>();

            let mut jump_positions = BTreeMap::new();
            {
                let mut jump_stack = Vec::new();
                for (i, value) in source.chars().enumerate() {
                    match value {
                        '[' => {
                            jump_stack.push(i);
                        }
                        ']' => {
                            if jump_stack.is_empty() {
                                // Err
                            }
                            if let Some(v) = jump_stack.pop() {
                                jump_positions.insert(i, v);
                                jump_positions.insert(v, i);
                            }
                        }
                        _ => (),
                    }
                }
            }

            let source_bytes = source.as_bytes();
            let mut program_counter: usize = 0;
            let mut vm = BFMemory::new();

            while program_counter < source_bytes.len() {
                match source_bytes[program_counter] as char {
                    '+' => vm.increase(),
                    '-' => vm.decrease(),
                    '<' => vm.move_left(),
                    '>' => vm.move_right(),
                    '[' => {
                        if vm.get_value() == 0 {
                            if let Some(&v) = jump_positions.get(&program_counter) {
                                program_counter = v;
                            }
                        }
                    }
                    ']' => {
                        if vm.get_value() != 0 {
                            if let Some(&v) = jump_positions.get(&program_counter) {
                                program_counter = v;
                            }
                        }
                    }
                    '.' => {
                        print!("{}", vm.get_value() as char);
                    }
                    ',' => {
                        let mut buf = [0];
                        let mut handle = std::io::stdin().take(1);

                        if handle.read(&mut buf).is_ok() {
                            vm.set_value(buf[0])
                        }
                    }
                    _ => (),
                }
                program_counter += 1;
            }

        } else if let Err(e) = writeln!(&mut std::io::stderr(), "File '{}' don't exist", arg) {
            println!("\nWriting Error: {}", e);

        }

    }
}
