use std::io::{BufReader, stdin, BufRead};
use crate::Mode::{Position, Immediate};
use std::borrow::Borrow;
use std::ops::IndexMut;
use std::num::ParseIntError;
use std::fmt::{Display, Debug, Formatter};
use std::fmt;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Position,
    Immediate
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instr {
    opcode: u8,
    modes: Vec<Mode>
}

fn mk_instr(val:i32) -> Option<Instr> {
    if val < 0 {
        return None
    }
    let opcode: u8 = (val % 100) as u8;
    if val / 100 == 0 {
        return Some(Instr { opcode: opcode, modes: Vec::new()})
    }
    let mut modes = Vec::new();
    let mut rest = val / 100;
    while rest > 0 {
        match rest % 10 {
            0 => modes.push(Position),
            1 => modes.push(Immediate),
            _ => return None
        }
        rest /= 10;
    }
    Some(Instr {opcode: opcode, modes: modes})
}

fn get_value(ops: &Vec<i32>, mode: Mode, num: i32) -> i32 {
    match mode {
        Immediate => ops[num as usize],
        Position => ops.get(ops[num as usize] as usize).unwrap_or(0.borrow()).clone()
    }
}

fn mode_for(instr: &Instr, idx: usize) -> Mode {
    instr.modes.get(idx).map_or(Position, |m| m.clone())
}

fn cell_ref(ops: &mut Vec<i32>, idx: i32) -> &mut i32 {
    if idx >= (ops.len() as i32) {
        let extra = 1 +(idx as usize) - ops.len();
        ops.reserve(extra);
        ops.extend((0..extra).map(|_| 0))

    }
    ops.index_mut(ops[(idx as usize)] as usize)
}

fn main() {
    let mut line_reader  = BufReader::new(stdin()).lines();
    let body = line_reader.next().unwrap().unwrap();
    let mut ops: Vec<i32> = body.split(',').map(|s| s.parse().unwrap()).collect();
    let mut head: i32 = 0;
    let mut cur: Instr;

    loop {
        cur = mk_instr(ops[head as usize]).unwrap_or_else(||
            panic!("Failed to parse at {}: {}", head, ops[head as usize])
        );
        let get = |ops, x: i32| get_value(ops, mode_for(&cur, x as usize), head+x+1);
        let set = |ops, c: i32| cell_ref(ops, head+c+1);
        match cur.opcode {
            1 => {
                *set(&mut ops, 2) = get(&ops, 0) + get(&ops, 1);
                head += 4
            }
            2 => {
                *set(&mut ops, 2) = get(&ops, 0) * get(&ops, 1);
                head += 4
            }
            3 => {
                *set(&mut ops, 0) = line_reader.next()
                    .unwrap_or( Ok(String::from("Failed")))
                    .unwrap_or(String::from("Failed"))
                    .parse().unwrap_or_else(|err: ParseIntError| panic!(err.to_string()));
                head += 2
            }
            4 => {
                println!("{}: {}", head, get(&ops, 0));
                head += 2
            }
            5 => {
                if get(&ops, 0) != 0 {
                    head = get(&ops, 1);
                } else {
                    head += 3;
                }
            }
            6 => if get(&ops, 0) == 0 {
                head = get(&ops, 1)
            } else { head += 3 }
            7 => {
                if get(&ops, 0) < get(&ops, 1) {*set(&mut ops, 2) = 1} else {*set(&mut ops, 2) =  0};
                head += 4;
            },
            8 => {
                if get(&ops, 0) == get(&ops, 1) {*set(&mut ops, 2) = 1} else {*set(&mut ops, 2) =  0};
                head += 4;
            },
            99 => {
                println!("return: {}", ops[0]);
                return;
            },
            _ => {
                println!("It's fucked: ({}, {:?})", head, cur);
                return;
            },
        }
    }
}

