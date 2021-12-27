// Advent of code 2015, day 04, part 1 and 2
// 
// Learning exercise by github.com/cttsrk
//
// Mine some md5 hashes like so: The key and an increasing number are
// concatenated as text and the result is hashed.
//
// Part 1: Find the first such hash that starts with five zeros.
//
// Part 2: Find the first such hash that starts with six zeros.
//
// There must be a cleaner way to do all this number to string casting, this
// looks messy.

fn first(key: &str, zeros: &str) -> usize {
    let mut number = 1;

    loop {
        let digest = md5::compute(key.to_string() + &number.to_string());
        if format!("{:x}", digest)[0..zeros.len()] == *zeros { break; }
        number += 1;
    }

    number
}

fn main() {
    let key = "yzbqklnj";

    println!("First hash with 5 leading zeroes: {}", first(key, "00000"));
    println!("First hash with 6 leading zeroes: {}", first(key, "000000"));
}
