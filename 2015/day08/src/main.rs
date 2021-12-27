// Advent of code 2015, day 08, part 1 and 2
//
// Learning exercise by github.com/cttsrk
//
// Part1: Calculate the memory footprint and literal size of some quoted
// strings.
//
// Part2: Calculate the size of an extra encoding.
//
// Easy compared to day 7, I guess Saturdays are difficult puzzles and Sundays
// are extra easy?

fn main() {
    let list = include_str!("../list.txt");

    let mut literals: i32 = 0;
    let mut symbols:  i32 = 0;
    let mut encoded:  i32 = 0;

    for line in list.lines() {
        let line = line.trim();
        literals += line.len() as i32;

        // Part 1: Just count inital sequence bytes while consuming bytes.
        let mut bytes = line[1..line.len()-1].bytes();  // Skip surrounding ""
        while let Some(cc) = bytes.next() {         // Default grabs 1 byte
            if cc == b'\\' { 
                if let Some(b'x') = bytes.next() {  // '\'  grabs 1 more byte
                      bytes.next(); bytes.next();   // '\x' grabs 2 more bytes
                }
            }
            symbols += 1;
        }

        // Part 2: Don't build the string, just count how long it would've been.
        encoded += 2;       // Add surrounding ""
        let mut bytes = line.bytes();
        while let Some(cc) = bytes.next() {
            match cc {
                b'"' | b'\\' => encoded += 2,
                           _ => encoded += 1,
            }
        }
    }

    println!("Part1: {} literals minus {} symbols equals {}.",
             literals, symbols, literals - symbols);
    println!("Part2: {} encoded minus {} literals equals {}.",
             encoded, literals, encoded - literals);
}
