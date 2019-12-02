use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() {
    let stdin = io::stdin();
    let f = BufReader::new(stdin);
    let mut agg: u64 = 0;
    for line in f.lines() {
        line.unwrap().parse().map(|v:u64| {
            agg += (v / 3) -2;()
        });
    }
    println!("{}", agg);
}
