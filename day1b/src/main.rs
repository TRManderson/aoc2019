use std::io::{self, BufReader};
use std::io::prelude::*;


fn main() {
    let stdin = io::stdin();
    let f = BufReader::new(stdin);
    let mut agg: u64 = 0;
    for line in f.lines() {
        line.unwrap().parse().map(|v:u64| {
            let mut fuel_load = v;
            while fuel_load > 6 {
                let new_load = (fuel_load / 3) - 2;
                agg += new_load;
                fuel_load = new_load;
            }
        });
    }

    println!("{}", agg);
}
