use std::io::{stdin, Read};
use std::cmp::PartialEq;
use std::iter::Iterator;
use std::fmt::Debug;

pub fn allowed(program: &Vec<u8>) -> Result<(), String> {
    let mut brackets = 0;
    for operator in program {
        match *operator {
            b'<' | b'>' | b'+' | b'-' | b'.' | b',' => {},
            b'[' => brackets += 1,
            b']' => brackets -= 1,
            _ => return Err("the program has not allowed symbols".to_string())
        }
    }
    if brackets != 0 {
        return Err("amount of square brackets is not equal".to_string());
    }
    Ok(())
}

fn balance_point<T: PartialEq + Debug>(iter: &mut Iterator<Item=&T>, open: T, close: T) -> Option<usize> {
    let mut balance = 0;
    for (i, item) in iter.enumerate() {
        if *item == open {
            balance += 1;
        } else if *item == close {
            balance -= 1;
        }
        if balance == 0 {
            return Some(i);
        } else if balance < 0 {
            return None;
        }
    }
    return None;
}

pub fn interprete(program: &Vec<u8>) -> Result<(), String> {
    let mut registers = [0u8; 32768];
    let mut reg_pos: usize = 0;
    let mut app_pos: usize = 0;
    let mut character = [0];
    let program_len = program.len();

    while app_pos < program.len() {
        match program[app_pos] {
            b'>' => reg_pos += 1,
            b'<' => reg_pos -= 1,
            b'+' => registers[reg_pos] += 1,
            b'-' => registers[reg_pos] -= 1,
            b'.' => print!("{}", registers[reg_pos] as char),
            b',' => {stdin().read(&mut character).map_err(|e| format!("can't read a character: {}", e))?;}
            b'[' => {
                if registers[reg_pos] == 0 {
                    let mut iter = program.iter().skip(app_pos - 1);
                    if let Some(found) = balance_point(&mut iter, b'[', b']') {
                        app_pos = found + app_pos;
                    } else {
                        return Err("can't find closing symbol ] of a loop".to_string());
                    }
                }
            }
            b']' => {
                if registers[reg_pos] != 0 {
                    let mut iter = program.iter().rev().skip(program_len - app_pos - 1);
                    if let Some(found) = balance_point(&mut iter, b']', b'[') {
                        app_pos -= found;
                    } else {
                        return Err("can't find opening symbol [ of a loop".to_string());
                    }
                }
            }
            _ => {}
        }
        app_pos += 1;
    }
    Ok(())
}