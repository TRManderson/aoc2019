use std::collections::HashMap;
use std::io::{BufReader, stdin, BufRead};
use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Copy, Clone,Debug)]
struct Pair{
    x: i32,
    y: i32
}


enum Dir {
    L, R, U, D
}

struct Instr {
    dir: Dir,
    len: u16,
}


fn parse_line(data: String) -> Vec<Instr> {
    data.split(",").map(|col| {
        let dir = match &col[0..1] {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => panic!("Bad parse")
        };
        Instr {
            dir,
            len: col[1..].parse().unwrap_or_else(|_x| panic!("Bad int parse"))
        }
    }).collect()
}

fn main() {
    // this algo sucks, don't hate
    let mut reader = BufReader::new(stdin()).lines();
    let line1 = parse_line(reader.next().unwrap().unwrap());
    let line2 = parse_line(reader.next().unwrap().unwrap());


    let mut coords = Pair {x: 0, y: 0};
    let mut steps = 0;
    let mut refs1 = HashMap::new();
    let mut refs2 = HashMap::new();
    let mv1: fn(&Instr) -> (fn(Pair) -> Pair) = |instr| match instr.dir {
        Dir::U => (|c| Pair {x: c.x, y: c.y+1}),
        Dir::D => (|c| Pair {x: c.x, y: c.y-1}),
        Dir::R => (|c| Pair {x: c.x+1, y: c.y}),
        Dir::L => (|c| Pair {x: c.x-1, y: c.y}),
    };
    for instr in line1 {
        let do_mv = mv1(&instr);
        for _i in 1..(instr.len + 1) {
            coords = do_mv(coords);
            steps += 1;
            if !refs1.contains_key(&coords) {refs1.insert(coords.clone(), steps);}
        }
    }

    coords = Pair {x: 0, y: 0};
    steps = 0;
    for instr in line2 {
        let do_mv = mv1(&instr);
        for _i in 1..(instr.len + 1) {
            coords = do_mv(coords);
            steps += 1;
            if !refs2.contains_key(&coords) {refs2.insert(coords.clone(), steps);}
        }
    }
    refs1.remove(&Pair {x: 0, y: 0});
    refs2.remove(&Pair {x: 0, y: 0});

    println!("{}", refs1.iter()
        .filter_map(|(c, v1)| refs2.get(c).map(|v2| v1 + v2))
        .min()
        .unwrap_or(10000000)
    )

}
