// Advent of code 2015, day 01, part 1 and 2
//
// Learning exercise by github.com/cttsrk
//
// Santa starts delivering presents at the ground floor of a multi-storey
// building and rides the elevator up or down according to instructions in
// `floor_code.txt`. '(' means up one floor and ')' means down one floor.
//
// Part 1: What floor does he end up on at the end?
//
// Part 2: How many elevator rides does it take for Santa to go into the
// basement for the first time?
//
// NOTE: Per Rust RFC 212, untyped integers default to `i32` unless another type
// can be inferred from the code. See details at 
// github.com/rust-lang/rfcs/blob/master/text/0212-restore-int-fallback.md
//
// NOTE: Don't use an unqualified "if ( x, else y", but rather match exactly on
// '(' and ')'. The input file as downloaded from adventofcode.com contains only
// '(' and ')', but if you copypaste in an editor and save, you'll likely have a
// newline character at the end of the file resulting in a wrong count.
//
// NOTE: So happy I found a one-liner for loading an input file. This loading
// happens at compile time, so changes to the input require recompiling. Loading
// at run time I believe requires the usual `use std::fs::File;` and
// `use std::io::BufReader;` etc.

fn main() {
    let mut floor = 0;
    let mut first = true;

    for (ii, c) in include_str!("../floor_code.txt").chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => {}
        }

        if first && floor < 0 {
            println!("First basement visit after {} changes.", ii + 1);
            first = false;
        };
    }

    println!("The final floor is {}.", floor);
}
