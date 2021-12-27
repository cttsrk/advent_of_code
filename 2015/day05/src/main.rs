// Advent of code 2015, day 05, part 1 and 2
// 
// Learning exercise by github.com/cttsrk
//
// Do some arbitrary pattern matching to determine which strings qualify.
//
// GOTCHA: The vowel 'y' is not included in the vowel criterion. Go figure...

use fancy_regex::Regex;     // Non-fancy regex doesn't support backreferences

fn main() {
    let input  = include_str!("../strings.txt");        // Compile time include
    let vowel  = Regex::new(r"[aeiou]")    .unwrap();
    let double = Regex::new(r"(.)\1{1}")   .unwrap();
    let flag   = Regex::new(r"ab|cd|pq|xy").unwrap();
    let ppair  = Regex::new(r"(..).*\1")   .unwrap();
    let mirror = Regex::new(r"(.).\1")     .unwrap();

    let (mut part1, mut part2) = (0, 0);

    for line in input.lines() {
        let vowels   = vowel .find_iter(line).count();
        let doubles  = double.is_match(line) .unwrap();
        let flags    = flag  .is_match(line) .unwrap();
        let pairpair = ppair .is_match(line) .unwrap();
        let mirrored = mirror.is_match(line) .unwrap();

        if ! flags  && doubles && vowels >= 3 { part1 += 1; }
        if pairpair && mirrored               { part2 += 1; }
    }
    println!("{} nice lines according to the Part 1 criteria.", part1);
    println!("{} nice lines according to the Part 2 criteria.", part2);
}
