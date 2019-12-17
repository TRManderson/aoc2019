
fn pred(data: &String) -> bool {
    let mut chars: Vec<(u8, u8)> = Vec::with_capacity(6);
    for byte in data.bytes() {
        match chars.last_mut() {
            Some((prev, count)) =>
                if prev.clone() > byte {return false;}
                else if prev.clone() == byte { *count += 1; }
                else { chars.push((byte, 1))}
            None => chars.push((byte, 1))
        }
    }
    for (_char, count) in chars {
        if count == 2 {
            return true
        }
    }
    false
}



fn main() {
    let res= (254032..789860)
        .map(|v| v.to_string())
        .filter(pred).count();
    println!("{}", res)
}
