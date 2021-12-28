// Advent of code 2015, day 07, part 1 and 2
//
// Learning exercise by github.com/cttsrk
//
// Part1: Find out the output of wire "a" in the provided circuit.
//
// Part2: Pin wire "b" to the value of the output from part1 (meaning no
// feedback, just copy out the value once) and find out the new output of wire
// "a".
//
// This one was far enough outside my experience that the learning curve was
// quite steep. I made detours into parsing/lexing, the borrow checker,
// hashmaps, destructuring, enums, etc.
//
// I think maybe this puzzle was designed for recursive solving so I went for
// that and had a nice aha moment with the placement of the base case.
//
// Por part 2, it took some thinking to figure out how to clear the cache
// without another pass over the input file. I didn't find a way to iterate over
// the hashmap and modify its entries at the same time, so I ended up saving a
// separate vector of the key names from the first pass.

use std::collections::HashMap;

struct Instr<'a>{
    result: Option<u16>,
    op:  Option<&'a str>,
    in1: Option<&'a str>,
    in2: Option<&'a str>,
}

fn parse(circuit: &str) -> (Vec<&str>, HashMap<&str, Instr>) {
    let mut list = Vec::new();
    let mut map  = HashMap::new();

    // Parse lines of the format "in1 op in2 -> wire". Valid input can omit the
    // first one or two fields, so reverse the order.
    for line in circuit.lines() {
        let mut fields = line.split(' ').rev();

        let (wire, _)      = (fields.next().unwrap(), fields.next());
        let (in2, op, in1) = (fields.next(), fields.next(), fields.next());
                                                                                                                                                                 
        list.push(wire.clone());
        map.insert(wire, Instr { result: None, op: op, in1: in1, in2: in2 });
    }

    (list, map)
}

fn solve(wire: &str, mut map: &mut HashMap<&str, Instr>) -> u16 {
    // Checking for the direct value base case here seems simpler than dealing
    // with multi-type fields in the Instr struct.
    if let Ok(value) = wire.parse::<u16>() { return value; }

    // Check if this wire is already solved. An else branch could be used here
    // to guard against circular dependencies by checking for a seen bool.
    if let Some(result) = map[wire].result { return result; }

    let (mut val1, mut val2) = (0, 0);
    if let Some(val) = map[wire].in1 { val1 = solve(val, &mut map); }
    if let Some(val) = map[wire].in2 { val2 = solve(val, &mut map); }

    let result = match map[wire].op {
        Some("AND")    => val1 & val2,
        Some("OR")     => val1 | val2,
        Some("NOT")    => ! val2,
        Some("RSHIFT") => val1 >> val2,
        Some("LSHIFT") => val1 << val2,
        None           => val2,
                     _ => panic!("Invalid input")
    };

    if let Some(wire) = map.get_mut(wire) { wire.result = Some(result); }

    result
}

fn main() {
    let circuit = include_str!("../circuit.txt");
    let (list, mut map) = parse(circuit);

    let (key, pin) = ("a", "b");
    let part1 = solve(key, &mut map);

    // Clear saved results
    for w in list { if let Some(wire) = map.get_mut(w) { wire.result = None; } }

    if let Some(wire) = map.get_mut(pin) { wire.result = Some(part1); }
    let part2 = solve(key, &mut map);

    println!("Part 1: Wire {} outputs {}", key, part1);
    println!("Part 2: Pinning {} to {}, {} outputs {}", pin, part1, key, part2);
}
