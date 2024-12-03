use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref LINE_RE: Regex = Regex::new("(\\d+)\\s+(\\d+)").unwrap();
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {

}

pub struct Day20 {
}

// Day 20
impl Day20 {
    pub const fn new() -> Self {
        Self {  }
    }

    fn read_input(_input: &str) -> Input
    {
        // TODO

        Input { }
    }
}

impl Day for Day20 {

    // Compute Part 1 solution
    fn part1(&self, input: &str) -> Answer {
        let _input = Self::read_input(input);

        Answer::None
    }

    fn part2(&self, input: &str) -> Answer {
        let _input = Self::read_input(input);

        Answer::None
    }
}

#[cfg(test)]

mod test {

    use crate::day20::Day20;
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str =
"// TODO Place example inputs here.
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let _input = Day20::read_input(EXAMPLE1);
                
        // assert_eq!(input.left.len(), 6);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day20::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::None);
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day20::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}