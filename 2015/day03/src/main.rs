// Advent of code 2015, day 03, part 1 and 2
// 
// Learning exercise by github.com/cttsrk
//
// Part 1: Find out how many houses Santa has delivered at least one present to.
// The houses are in an infinite square grid and santa moves according to
// directions in the input file, one step in a cardinal direction for every
// instruction.
//
// Part 2: Find out how many houses are visited with the same input, only this
// time there are two couriers taking turns reading the directions.
//
// This solution supports an arbitrary number of couriers.

use std::collections::HashSet;

fn houses(directions: &str, couriers: usize) -> usize {
    let mut pos    = vec![[0, 0]; couriers];
    let mut curr   = 0;
    let mut houses = HashSet::new();

    houses.insert(pos[curr]);

    for c in directions.chars() {
        match c {
            '>' => pos[curr][0] += 1,
            '^' => pos[curr][1] += 1,
            '<' => pos[curr][0] -= 1,
            'v' => pos[curr][1] -= 1,
            _   => println!("Invalid input '{}', skipping", c),
        }

        houses.insert(pos[curr]);

        curr = (curr + 1) % pos.len(); 
    }

    houses.len()
}

fn main() {
    let input = include_str!("../directions.txt");

    println!("Houses visited with 1 couriers: {}", houses(&input, 1));
    println!("Houses visited with 2 couriers: {}", houses(&input, 2));
}
