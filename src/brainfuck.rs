use std::io::{stdin, Read};
use std::cmp::PartialEq;

pub fn allowed(program: &Vec<u8>) -> Option<()> {
    for operator in program {
        match *operator {
            b'<' | b'>' | b'+' | b'-' | b'.' | b',' | b'[' | b']' => {}
            _ => {
                return None;
            }
        }
    }
    Some(())
}

fn balanced<T: PartialEq>(arr: &[T], open: T, close: T) -> Option<usize> {
    let mut i: usize = 0;
    let mut balance = 0;
    while i < arr.len() {
        if arr[i] == open {
            balance += 1;
        } else if arr[i] == close {
            balance -= 1;
        }
        if balance == 0 {
            return Some(i);
        } else if balance < 0 {
            println!("first");
            return None;
        }
        i += 1;
    }
    println!("last");
    return None;
}

fn rev_balanced<T: PartialEq>(arr: &[T], open: T, close: T) -> Option<usize> {
    let mut i: usize = 0;
    let len = arr.len();
    let mut balance = 0;
    while i < len {
        let j = len - 1 - i;
        if arr[j] == open {
            balance += 1;
        } else if arr[j] == close {
            balance -= 1;
        }
        if balance == 0 {
            return Some(j);
        } else if balance < 0 {
            return None;
        }
        i += 1;
    }
    return None;
}

pub fn interprete(program: &mut Vec<u8>) -> Option<()> {
    let mut registers = [0u8; 32768];
    let mut reg_pos: usize = 0;
    let mut app_pos: usize = 0;
    let mut characher = [0];

    while app_pos < program.len() {
        //print!("{}", program[app_pos] as char);
        match program[app_pos] {
            b'>' => { reg_pos += 1; }
            b'<' => { reg_pos -= 1; }
            b'+' => { registers[reg_pos] += 1 }
            b'-' => { registers[reg_pos] -= 1 }
            b'.' => {
                //print!("{}", registers[reg_pos]);
                print!("{}", registers[reg_pos] as char)
            }
            b',' => {
                if let Err(e) = stdin().read(&mut characher) {
                    eprintln!("can't read a character: {}", e);
                }
            }
            b'[' => {
                if registers[reg_pos] == 0 {
                    app_pos = balanced(&program.as_slice()[app_pos..program.len()], b'[', b']')?;
                }
            }
            b']' => {
                if registers[reg_pos] != 0 {
                    app_pos = rev_balanced(&program.as_slice()[..app_pos+1], b']', b'[')?;
                }
            }
            _ => {}
        }
        app_pos += 1;
    }
    Some(())
}