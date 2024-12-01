use std::iter::zip;
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
    // Sorted lists, left and right
    left: Vec<isize>,
    right: Vec<isize>,
}

pub struct Day1<'a> {
    input: &'a str,
}

// Day1
impl<'a> Day1<'a> {
    pub const fn new(input: &'a str) -> Self {
        Self { input: input }
    }

    fn read_input(input: &'a str) -> Input
    {
        let mut left: Vec<isize> = Vec::new();
        let mut right: Vec<isize> = Vec::new();

        for line in input.lines() {
            match LINE_RE.captures(line) {                    
                Some(captures) => {
                    let a = captures[1].parse().unwrap();
                    let b= captures[2].parse().unwrap();                    
                    left.push(a);
                    right.push(b);
                }
                None => {
                    // Ignore this line.
                    println!("Ignored input: {line}");
                }
            }
        }

        left.sort();
        right.sort();

        Input { left, right }
    }
}

impl<'a> Day for Day1<'a> {

    // Compute Part 1 solution
    fn part1(&self) -> Answer {
        let input = Self::read_input(self.input);

        let dist_sum: isize = zip(&input.left, &input.right)
            .map(|pair| { (pair.0-pair.1).abs() }) 
            .sum();

        Answer::Numeric(dist_sum as usize)
    }

    fn part2(&self) -> Answer {

        // Read input file into Input struct, then sum the results.
   
        // (The diff between part1 and part2 is the flag passed to read_input.  It
        // interprets numbers embedded in lines differently for each part.)
        let input = Self::read_input(self.input);

        let similarity = input.left.iter().map(|l| {
            let match_count = input.right.iter().filter(|r| {l == *r}).count();
            *l as usize * match_count
        }).sum();

        Answer::Numeric(similarity)
    }
}

#[cfg(test)]

mod test {

    use crate::day1::Day1;
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str =
"3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Day1::read_input(EXAMPLE1);
                
        assert_eq!(input.left.len(), 6);
        assert_eq!(input.right.len(), 6);
        assert_eq!((input.left[0], input.right[0]), (1, 3));
        assert_eq!((input.left[1], input.right[1]), (2, 3));
        assert_eq!((input.left[2], input.right[2]), (3, 3));
        assert_eq!((input.left[3], input.right[3]), (3, 4));
        assert_eq!((input.left[4], input.right[4]), (3, 5));
        assert_eq!((input.left[5], input.right[5]), (4, 9));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d: Day1 = Day1::new(EXAMPLE1);
        assert_eq!(d.part1(), Answer::Numeric(11));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d: Day1 = Day1::new(EXAMPLE1);
        assert_eq!(d.part2(), Answer::Numeric(31));
    }
    
}
