// Advent of code 2015, day 07, part 1 and 2
//
// Learning exercise by github.com/cttsrk
//
// This one was far enough outside my experience that the learning curve was
// quite steep. I made detours into parsing/lexing, the borrow checker,
// hashmaps, destructuring, enums, etc.
//
// I think maybe this puzzle was designed for recursive solving so I went for
// that and had a nice aha moment with the placement of the base case.
//
// For part 2, it took some thinking to figure out how to clear the cache
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
    // with multi-type fields in the Instr struct and op match.
    if let Ok(value) = wire.parse::<u16>() { return value; }

    // Check if this wire is already solved. An else branch could be used here
    // to guard against circular dependencies by checking for a seen bool.
    if let Some(result) = map[wire].result { return result; }

    // Recursively solve any inputs that are not `None`
    let v1 = if let Some(v) = map[wire].in1 { solve(v, &mut map) } else { 0 };
    let v2 = if let Some(v) = map[wire].in2 { solve(v, &mut map) } else { 0 };

    // Perform the logic operation that feeds this wire. No operator implies a
    // direct value / forwarding from another wire.
    let result = match map[wire].op {
        Some("AND")    => v1 & v2,
        Some("OR")     => v1 | v2,
        Some("NOT")    => ! v2,
        Some("RSHIFT") => v1 >> v2,
        Some("LSHIFT") => v1 << v2,
        None           => v2,
                     _ => panic!("Invalid input")
    };

    // This wire has been solved for the first time, cache the result for re-use
    // when other paths through the circuit reach this wire.
    if let Some(w) = map.get_mut(wire) { w.result = Some(result); }

    result
}

fn main() {
    // Parse input, solve part 1
    let (list, mut map) = parse(include_str!("../circuit.txt"));
    let (key, pin) = ("a", "b");
    let part1 = solve(key, &mut map);

    // Clear cache, pin wire "b", solve part 2
    for wire in list { if let Some(w) = map.get_mut(wire) { w.result = None; } }
    if let Some(w) = map.get_mut(pin) { w.result = Some(part1); }
    let part2 = solve(key, &mut map);

    println!("Part 1: Wire 'a' outputs {}", part1);
    println!("Part 2: Pinning wire 'b' to {}, 'a' outputs {}.", part1, part2);
}
