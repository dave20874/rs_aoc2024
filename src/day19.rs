use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    static ref PATTERN_RE: Regex = Regex::new("([wubrg]+)").unwrap();
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut patterns = Vec::new();
        let mut designs = Vec::new();

        let mut in_designs = false;
        for line in text.lines() {
            if !in_designs {
                if line.trim().len() == 0 {
                    // Transition to designs
                    in_designs = true;
                }
                else {
                    // Read patterns
                    for caps in PATTERN_RE.captures_iter(line) {
                        patterns.push(caps[1].to_string());
                    }
                }
            }
            else {
                // Read a design
                designs.push(line.trim().to_string());
            }
        }

        Input { patterns, designs }
    }
}

pub struct Day19 {
}

// Day19
impl Day19 {
    pub const fn new() -> Self {
        Self { }
    }

    fn alt_matches(input: &Input) -> usize {
        let mut matches = 0;

        for design in input.designs.iter() {
            if Self::design_matches(input, design) > 0 {
                matches += 1;
            }
        }

        matches
    }

    // This evaluates the number of solutions in terms of the number of solutions on
    // shorter versions (tails) of the input string.  It starts with saying there is
    // 1 way to match "".  This is stored in a cache of sub solutions.  Then it uses
    // induction.  To evaluate a longer tail, t, we look at each of the patterns 
    // form prefixes of t and add up the sub-solutions resulting from removing that 
    // prefix.  (This would have been evaluated earlier.)  The resulting sum is then
    // stored in sub-solutions for t and we go up to the next larger tail.
    //
    // By the time this is done, the sub-solutions hashmap contains the solution for
    // the full input design.
    fn design_matches(input: &Input, design: &str) -> usize {
        let mut sub_solns: HashMap<&str, usize> = HashMap::new();

        // Seed sub_solns.  There is exactly one way to match the empty string.
        sub_solns.insert("", 1);

        // Grow the test string from the back toward the front, evaluating the
        // number of solutions based on how many patterns match and how
        // many solutions are downstream of that.
        for n in (0..design.len()).rev() {
            let sub_str = &design[n..design.len()];
            let mut matches = 0;

            for p in &input.patterns {
                if sub_str.starts_with(p) {
                    let rest = &sub_str[p.len()..];
                    matches += sub_solns.get(rest).unwrap();
                }
            }

            sub_solns.insert(sub_str, matches);
        }

        *sub_solns.get(design).unwrap()
    }

    fn num_matches(input: &Input) -> usize {
        input.designs.iter().map(|d| {Self::design_matches(input, d)}).sum()
    }
}

impl<'a> Day for Day19 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(Self::alt_matches(&input))
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(Self::num_matches(&input))
    }
}

#[cfg(test)]

mod test {

    use crate::day19::{Day19, Input};
    use crate::day::{Day, Answer};
    
    // Example Inputs
    const EXAMPLE1: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.patterns.len(), 8);
        assert_eq!(input.designs.len(), 8);
        assert_eq!(input.patterns[7], "br");
        assert_eq!(input.designs[7], "bbrgwb");
    }
   
    #[test]
    fn test_alt_matches() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(Day19::alt_matches(&input), 6);
    }
    
    #[test]
    fn test_num_matches() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(Day19::num_matches(&input), 16);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day19::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(6));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day19::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(16));
    }
    
}