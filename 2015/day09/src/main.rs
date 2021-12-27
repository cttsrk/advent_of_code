// Advent of code 2015, day 09, part 1 and 2
//
// Learning exercise by github.com/cttsrk
//

fn main() {
    let list = include_str!("../list.txt");


    println!("Part1: {} literals minus {} symbols equals {}.",
             literals, symbols, literals - symbols);
    println!("Part2: {} encoded minus {} literals equals {}.",
             encoded, literals, encoded - literals);
}
