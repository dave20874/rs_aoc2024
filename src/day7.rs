use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN: AA BBB CC ..."
    // captures 1 is the NNNNN part.  capture 2 is AA BBB CC ...
    static ref LINE_RE: Regex = Regex::new("(\\d+): (.*)").unwrap();

    // captures strings of digits, to be used with captures_iter()
    static ref NUM_RE: Regex = Regex::new("(\\d+)").unwrap();
}

struct Problem {
    result: usize,
    components: Vec<usize>,
}

fn next_pow10(n: usize) -> usize {
    let mut result = 1;

    while n >= result {
        result *= 10;
    }

    result
}

impl Problem {
    fn solvable(&self, with_concat: bool) -> bool {
        // Get a slice of all the components
        let components = &self.components[..];

        // Create a list of possibilities to check, initialize it with this problem.
        let mut to_check: Vec<(usize, &[usize])> = Vec::new();
        to_check.push((self.result, components));

        while let Some((r, c)) = to_check.pop() {
            // Can components c form result r?

            if c.len() == 1 {
                // If we've reduced it to one component and it matches, we found a solution.
                if c[0] == r { return true; }
                continue;
            }

            // From here we know c is len() 2 or more.
            // multiple components, reduce complexity
            let last_component = c[c.len()-1];

            // Is last component too large already? If so, abandon this branch
            if last_component > r {continue;}

            // Could multiplication work?
            if r % last_component == 0 {
                // r is a multiple of last_component, so test multiplication here
                to_check.push((r/last_component, &c[0..c.len()-1]));
            }

            // If concatenation is allowed, would it work?
            if with_concat {
                let next_pow10 = next_pow10(last_component);
                if r % next_pow10 == last_component {
                    // maybe concat would work
                    to_check.push((r/next_pow10, &c[0..c.len()-1]));
                }
            }

            // test addition in last spot
            to_check.push((r-last_component, &c[0..c.len()-1]));
        }

        // We didn't find a solution
        false
    }
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    problems: Vec<Problem>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        // Iterate over input lines, creating a Problem from each one, collect them into a Vec
        let problems = text.lines()
            .map(|l| {
                // split 'result' from components with LINE_RE
                let caps = LINE_RE.captures(l).unwrap();
                let result = caps[1].parse::<usize>().unwrap();

                // Iterate over numeric components, collecting them into a vector.
                let components = NUM_RE.captures_iter(&caps[2])
                    .map(|num_caps| num_caps[1].parse::<usize>().unwrap())
                    .collect();
                Problem {result, components}
            })
            .collect();

        Input { problems }
    }

    fn sum_solvable(&self, with_concat: bool) -> usize {
        self.problems.iter()
            .filter(|p| p.solvable(with_concat) )
            .map(|p| p.result)
            .sum()
    }
}

pub struct Day7 {
}

// Day7
impl Day7 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day7 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.sum_solvable(false))
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.sum_solvable(true))
    }
}

#[cfg(test)]

mod test {

    use crate::day7::{Day7, Input};
    use crate::day::{Day, Answer};
    
    // Example Inputs
    const EXAMPLE1: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        // Check that inputs look ok
        assert_eq!(input.problems.len(), 9);
        assert_eq!(input.problems[0].components.len(), 2);
        assert_eq!(input.problems[0].result, 190);
    }

    #[test]
    fn test_solvable_no_concat() {
        let input = Input::read(EXAMPLE1);
        let expected = [true, true, false, false, false, false, false, false, true];

        // Make sure all the example problems are tested correctly.
        for (n, prob) in input.problems.iter().enumerate() {
            assert_eq!(prob.solvable(false), expected[n]);
        }
    }

    #[test]
    fn test_solvable_with_concat() {
        let input = Input::read(EXAMPLE1);
        let expected = [true, true, false, true, true, false, true, false, true];

        // Make sure all the example problems are tested correctly.
        // (Allowing for concatenation this time.)
        for (n, prob) in input.problems.iter().enumerate() {
            assert_eq!(prob.solvable(true), expected[n]);
        }
    }

    #[test]
    fn test_sum_solvable_no_concat() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.sum_solvable(false), 3749);
    }
    
    #[test]
    fn test_sum_solvable_with_concat() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.sum_solvable(true), 11387);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day7::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(3749));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day7::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(11387));
    }
    
}