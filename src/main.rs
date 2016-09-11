use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::BTreeMap;

struct Memory {
    list: Vec<u8>,
    pointer: usize,
}

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
            {
                let source_bytes = source.as_bytes();
                let mut program_counter: usize = 0;
                let mut vm = Memory {
                    list: vec![0; 100],
                    pointer: 0,
                };

                while program_counter < source_bytes.len() {
                    match source_bytes[program_counter] as char {
                        '+' => {
                            if vm.list[vm.pointer].checked_add(1).is_none() {
                                // err
                            } else {
                                vm.list[vm.pointer] += 1;
                            }
                        }
                        '-' => {
                            if vm.list[vm.pointer].checked_sub(1).is_none() {
                                // err
                            } else {
                                vm.list[vm.pointer] -= 1;
                            }
                        }
                        '<' => {
                            if vm.pointer.checked_sub(1).is_none() {
                               // err
                            } else {
                              vm.pointer -= 1;
                            }
                        }
                        '>' => {
                            vm.pointer += 1;
                            if vm.pointer >= vm.list.len() {
                                vm.list.push(0);
                            }
                        }
                        '[' => {
                            if vm.list[vm.pointer] == 0 {
                                if let Some(&v) = jump_positions.get(&program_counter) {
                                    program_counter = v;
                                }
                            }
                        }
                        ']' => {
                            if vm.list[vm.pointer] != 0 {
                                if let Some(&v) = jump_positions.get(&program_counter) {
                                    program_counter = v;
                                }
                            }
                        }
                        '.' => {
                            print!("{}", vm.list[vm.pointer] as char);
                        }
                        ',' => {
                            let mut buf = [0];
                            let mut handle = std::io::stdin().take(1);

                            if handle.read(&mut buf).is_ok() {
                                vm.list[vm.pointer] = buf[0];
                            }
                        }
                        _ => (),
                    }
                    program_counter += 1;
                }
            }
        } else {
            if let Err(e) = writeln!(&mut std::io::stderr(), "File '{}' don't exist", arg) {
                println!("\nWriting Error: {}", e);
            }
        }
    }
}
