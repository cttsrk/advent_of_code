// Advent of code 2015, day 06, part 1 and 2
// 
// Learning exercise by github.com/cttsrk
//
// The input is 300 instructions for how to treat a 1000 x 1000 grid of
// christmas lights.
//
// Part 1: Turn lights on or off according to the instructions. How many are on
// after all instructions have been performed?
//
// Part 2: Dim or brighten lights according to the instructions. What is the
// total brightness of all lights after all 300 instructions have been peformed?
//
// Still thinking about the factoring here. For now, do one pass through the
// instructions and apply them to two matrices. It could also be two passes
// applied to one matrix at a time. I've seen solutions that use a chimera
// matrix struct that can be instantiated with `bool` or `int`, but I think KISS
// trumps DRY in this case.

#[derive(Debug)]
struct Instruction {
    action:  usize,
    begin:  [usize; 2],
    size:   [usize; 2],
}

// Parse an input line into arguments.
// TODO: Could try to learn the nom crate for this.
impl Instruction {
    fn new(line: &str) -> Self {
        let line = line.replace("turn on",  "1")
                       .replace("turn off", "0")
                       .replace("toggle",   "2")
                       .replace("through ", "" );
        let args: Vec<_> = line.split([' ', ','].as_ref())
                               .map(|x| x.parse::<usize>().unwrap())
                               .collect();
        Self {
            action: args[0],
            begin:  [args[1], args[2]],
            size:   [args[3] - args[1] + 1, args[4] - args[2] + 1],
        }
    }
}

fn main() {
    const ROOT:   usize = 1000;
    const MATRIX: usize = usize::pow(ROOT, 2);

    // Use 1d arrays as virtual 2d matrices. Stack allocating these as usize
    // causes overflow, so use more compact types. Using u8 for lights2 is
    // unsafe since the theoretical maximum value after 300 instructions is 600,
    // but my lights.txt input doesn't cause any overflow.
    let mut lights1: [u8; MATRIX] = [0; MATRIX];
    let mut lights2: [u8; MATRIX] = [0; MATRIX];

    for line in include_str!("../lights.txt").lines() {
        let instr = Instruction::new(line);
        println!("{:?}", instr);        // Slightly bottlenecking on this print

        let mut offset = instr.begin[0] * ROOT + instr.begin[1]; 
        for _ in 0..instr.size[0] {
            for ii in 0..instr.size[1] {
                let p = offset + ii;

                match instr.action {
                    1 => {
                        lights1[p] = 1;
                        lights2[p] = lights2[p] + 1;
                    },
                    0 => {
                        lights1[p] = 0;
                        lights2[p] = lights2[p].saturating_sub(1);
                    },
                    2 => {
                        lights1[p] = 1 - lights1[p];        // 0/1 toggle
                        lights2[p] = lights2[p] + 2;
                    },
                    _ => println!("Unknown input {}", instr.action),
                }
            }
            offset += ROOT;
        }
    }

    println!("Part 1: {} lit lights",
             lights1.iter().filter(|x| **x == 1).count());
    println!("Part 2: {} total brightness",
             lights2.iter().map(|x| *x as u32).sum::<u32>());
}
