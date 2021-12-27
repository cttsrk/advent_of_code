// Advent of code 2015, day 07, part 1 and 2
//
// Learning exercise by github.com/cttsrk
//
// Part1: Find out the output of wire "a" in the provided circuit.
//
// Part2: Pin wire "b" to the value of the output from part1 (that is, no
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
// At the end, it took some thinking to figure out how to clear the cache for
// part2 without another pass over the input file. I didn't find a way to
// iterate over the hashmap and modify its entries at the same time, so I ended
// up saving a separate vector of the key names from the first pass. KISSing
// more would be to just rebuild the hashmap after finding the answer to part1.

use std::collections::HashMap;

struct Instr {
    result: Option<u16>,
    expr:   Expr,
}

#[derive(Clone)]
enum Expr {
    And(String, String),
    Or(String, String),
    Not(String),
    RShift(String, String),
    LShift(String, String),
    Value(String),
}

fn parse(line: &str) -> (&str, Instr) {
    let mut args = line.split(' ').rev();

    let wire   = args.next().unwrap_or("_");
    let _      = args.next();   // Skip the "->"
    let input2 = args.next().unwrap_or("_").to_string();
    let op     = args.next().unwrap_or("implicit");
    let input1 = args.next().unwrap_or("_").to_string();

    let expr = match op {
        "AND"      => Expr::And(input1, input2),
        "OR"       => Expr::Or(input1, input2),
        "NOT"      => Expr::Not(input2),
        "RSHIFT"   => Expr::RShift(input1, input2),
        "LSHIFT"   => Expr::LShift(input1, input2),
        "implicit" => Expr::Value(input2),
        _          => panic!("No operator")
    };

    (wire, Instr {
        result: None,
        expr:   expr,
    })
}

fn solve(wire: &str, mut map: &mut HashMap<&str, Instr>) -> u16 {
    // Checking for the direct value base case here is a lot simpler than
    // dealing with multi-type input fields in the Expr enum.
    if let Ok(val) = wire.parse::<u16>() { return val; }

    // Check if this wire is already solved. We could use an else branch here to
    // guard against circular dependencies by checking for a seen bool.
    if let Some(result) = map[wire].result { return result; }

    let result = match map[wire].expr.clone() {
        Expr::And(ref v1, ref v2)    => { solve(v1, &mut map) & solve(v2, &mut map) },
        Expr::Or(ref v1, ref v2)     => { solve(v1, &mut map) | solve(v2, &mut map) },
        Expr::Not(ref v2)            => { ! solve(v2, &mut map) },
        Expr::RShift(ref v1, ref v2) => { solve(v1, &mut map) >> solve(v2, &mut map) },
        Expr::LShift(ref v1, ref v2) => { solve(v1, &mut map) << solve(v2, &mut map) },
        Expr::Value(ref v2)          => { solve(v2, &mut map) },
    };

    if let Some(wire) = map.get_mut(wire) { wire.result = Some(result); }

    result
}

fn main() {
    let circuit = include_str!("../circuit.txt");

    let mut wires = Vec::new();
    let mut map   = HashMap::new();
    
    for line in circuit.lines() {
        let (wire, instr) = parse(line);
        wires.push(wire);
        map.insert(wire, instr);
    }

    let (key, pin) = ("a", "b");
    let part1 = solve(key, &mut map);

    for w in wires {
        if let Some(wire) = map.get_mut(w) { wire.result = None; }
    }

    if let Some(wire) = map.get_mut(pin) { wire.result = Some(part1); }
    let part2 = solve(key, &mut map);

    println!("Part 1: Wire {} outputs {}", key, part1);
    println!("Part 2: Pinning {} to {}, {} outputs {}", pin, part1, key, part2);
}
