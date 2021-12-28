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

struct Instr<'a> {
    result: Option<u16>,
    op:     Option<&'a str>,
    val1:   Option<&'a str>,
    val2:   Option<&'a str>,
}

fn parse(circuit: &'static str, mut list: Vec<&'static str>, mut map: HashMap<&'static str, Instr<'static>>) -> (Vec<&'static str>, HashMap<&'static str, Instr<'static>>) {
    // Parse a line of the format "in1 op in2 -> out". Valid input can omit
    // the first two fields, so reverse the order first.
    for line in circuit.lines() {
        let mut a = line.split(' ').rev();

        let (wire, _)    = (a.next().unwrap(), a.next());
        let (v2, op, v1) = (a.next(), a.next(), a.next());

        list.push(wire);
        map.insert(wire, Instr { result: None, op: op, val1: v1, val2: v2 });
    }

    (list, map)
}

fn solve(wire: &str, mut map: &mut HashMap<&str, Instr>) -> u16 {
    // Checking for the direct value base case here seems simpler than dealing
    // with multi-type fields in the Instr struct.
    if let Ok(val) = wire.parse::<u16>() { return val; }

    // Check if this wire is already solved. An else branch could be used here
    // to guard against circular dependencies by checking for a seen bool.
    if let Some(result) = map[wire].result { return result; }

    let (mut v1, mut v2) = (0, 0);
    if let Some(val) = map[wire].val1 { v1 = solve(val, &mut map); }
    if let Some(val) = map[wire].val2 { v2 = solve(val, &mut map); }

    let result = match map[wire].op {
        Some("AND")    => v1 & v2,
        Some("OR")     => v1 | v2,
        Some("NOT")    => ! v2,
        Some("RSHIFT") => v1 >> v2,
        Some("LSHIFT") => v1 << v2,
        None           => v2,
                     _ => panic!("Invalid input")
    };

    if let Some(wire) = map.get_mut(wire) { wire.result = Some(result); }

    result
}

fn main() {
    let circuit  = include_str!("../circuit.txt");
    let list = Vec::new();
    let map  = HashMap::new();
    
    let (list, mut map) = parse(circuit, list, map);

    let (key, pin) = ("a", "b");
    let part1 = solve(key, &mut map);

    for w in list {
        if let Some(wire) = map.get_mut(w) { wire.result = None; }
    }

    if let Some(wire) = map.get_mut(pin) { wire.result = Some(part1); }
    let part2 = solve(key, &mut map);

    println!("Part 1: Wire {} outputs {}", key, part1);
    println!("Part 2: Pinning {} to {}, {} outputs {}", pin, part1, key, part2);
}
