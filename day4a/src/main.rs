fn main() {
    let res= (254032..789860)
        .map(|v| v.to_string())
        .filter(|b| {
            let v: Vec<u8> = b.bytes().collect();
            if v.len() < 6 {
                println!("{}", b);
            }
            let mut has_one = false;
            for i in 1..6 {
                if v[i-1] > v[i] {return false; }
                if v[i as usize] == v[(i-1) as usize] {has_one = true;}

            }
            return has_one;
        }).count();
    println!("{}", res)
}
