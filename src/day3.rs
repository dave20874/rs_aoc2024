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

impl Input {
    fn read(_text: &str) -> Input
    {
        Input { }
    }
}

pub struct Day3 {
}

// Day3
impl Day3 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day3 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let _input = Input::read(text);

        Answer::None
    }

    fn part2(&self, text: &str) -> Answer {
        let _input = Input::read(text);

        Answer::None
    }
}

#[cfg(test)]

mod test {

    use crate::day3::{Day3, Input};
    use crate::day::{Day, Answer};
    
    // TODO Place example inputs here.
    const EXAMPLE1: &str = "\
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let _input = Input::read(EXAMPLE1);

        // TODO-DW : Verify that inputs were read successfully.
        // assert_eq!(input.left.len(), 6);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day3::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::None);
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day3::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}