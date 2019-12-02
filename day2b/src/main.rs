use std::io;


fn run(noun: usize, verb: usize, input: &Vec<usize>)  -> usize{
    let mut ops: Vec<usize> = input.clone();
    let mut head: usize = 0;
    ops[1] = noun; ops[2] = verb;

    while ops[head] != 99 {
        let target = ops[head+3].clone();
        match ops[head] {
            1 => ops[target] = ops[ops[head+1]] + ops[ops[head+2]],
            2 => ops[target] = ops[ops[head+1]] * ops[ops[head+2]],
            _ => panic!("unreachable")
        }
        head += 4;
    }
    ops[0]
}


fn main() {
    let mut body = String::new();
    let res = io::stdin().read_line(&mut body);
    let ops: Vec<usize> = body[..body.len()-1].split(',').map(|s| s.parse().unwrap_or(0)).map(|s:u16| usize::from(s)).collect();
    for noun in 0..99 {
        for verb in 0..99 {
            if run(noun, verb, &ops) == 19690720 {
                println!("{}", 100 * noun + verb);
                return
            }
        }
    }
}

