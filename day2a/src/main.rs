use std::io;


fn main() {
    let mut body = String::new();
    let res = io::stdin().read_line(&mut body);
    let mut ops: Vec<usize> = body[..body.len()-1].split(',').map(|s| s.parse().unwrap_or(0)).map(|s:u16| usize::from(s)).collect();
    let mut head = 0;
    ops[1] = 12; ops[2] = 2;

    while ops[head] != 99 {
        let target = ops[head+3].clone();
        match ops[head] {
            1 => ops[target] = ops[ops[head+1]] + ops[ops[head+2]],
            2 => ops[target] = ops[ops[head+1]] * ops[ops[head+2]],
            _ => panic!("unreachable")
        }
        head += 4;
    }

    println!("{}", ops[0]);
}

