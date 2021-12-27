// Advent of code 2015, day 02, part 1 and 2
//
// Learning exercise by github.com/cttsrk
//
// Part 1: Find out how much paper is needed to wrap a bunch of presents. All
// presents are cuboids and need wrapping paper equivalent to the area of their
// surfaces, plus extra paper equivalent to the smallest side.
//
// Part 2: Find out how much ribbon is needed to wrap said presents. The ribbon
// is wrapped the shortest way possible around a present, which is equivalent to
// the shortest perimeter of any side, plus extra ribbon of a length equivalent
// to the volume of the cuboid.

fn main() {

    let mut paper  = 0;
    let mut ribbon = 0;

    for line in include_str!("../presents.txt").lines() {
        let d: Vec<usize> = line.split("x")
                                .map(|s| s.parse().unwrap())
                                .collect();
        let mut sides  = [d[0] * d[1],
                          d[0] * d[2],
                          d[1] * d[2]];
        let mut perims = [(d[0] + d[1]) * 2,
                          (d[0] + d[2]) * 2,
                          (d[1] + d[2]) * 2];
        sides.sort();
        perims.sort();

        paper  += sides[0] + 2 * sides.iter().sum::<usize>();
        ribbon += perims[0] + d.iter().product::<usize>();

        println!("{:?}  sides:{:?}  paper:{}  ribbon:{}",
                d, sides, paper, ribbon);
    }
}
