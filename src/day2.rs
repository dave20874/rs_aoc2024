use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref LINE_RE: Regex = Regex::new("(\\d+)").unwrap();
}

struct Report {
    values: Vec<usize>,
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    reports: Vec<Report>,
}

pub struct Day2 {
}

fn is_safe(report: &Vec<usize>) -> bool {
    let mut ascending = false;
    let mut descending = false;
    let mut equal = false;
    let mut max_diff = 0;

    for window in report.windows(2) {
        let a = window[0];
        let b = window[1];
        let diff = 
            if a < b {
                ascending = true;
                b - a
            } 
            else if a > b {
                descending = true;
                a - b
            }
            else {
                equal = true;
                0
            };

        if diff > max_diff {
            max_diff = diff;
        }
    }

    return !equal & !(ascending & descending) & (max_diff <= 3);
}

fn is_damped_safe(report: &Vec<usize>) -> bool {
    let mut found_safe = false;

    for skip in 0..report.len() {
        let shortened_values = report.iter()
            .enumerate()
            .filter(|(n, _value)| *n != skip)
            .map(|(_n, value)| *value)
            .collect();

        if is_safe(&shortened_values) {
            // println!("Found safe with sample {skip} removed: {shortened_values:#?}");
            found_safe = true;
            break;
        }
    }

    found_safe
}

// Day2
impl Day2 {
    pub const fn new() -> Self {
        Self { }
    }

    fn read_input(input: & str) -> Input
    {
        let mut reports: Vec<Report> = Vec::new();
        for line in input.lines() {
            let values: Vec<usize> = LINE_RE.find_iter(line)
                .map(|m| m.as_str()
                    .parse::<usize>().unwrap())
                .collect();

            reports.push(Report { values });
        }

        Input { reports }
    }
}

impl Day for Day2 {

    // Compute Part 1 solution
    fn part1(&self, input: &str) -> Answer {
        let input = Self::read_input(input);

        let num_safe = input.reports.iter().filter(|r| is_safe(&r.values)).count();

        Answer::Numeric(num_safe)
    }

    fn part2(&self, input: &str) -> Answer {
        let input = Self::read_input(input);

        let num_safe = input.reports.iter().filter(|r| is_damped_safe(&r.values)).count();

        Answer::Numeric(num_safe)
    }
}

#[cfg(test)]

mod test {

    use crate::day2::{Day2, is_safe, is_damped_safe};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str =
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Day2::read_input(EXAMPLE1);
                
        assert_eq!(input.reports.len(), 6);
        assert_eq!(input.reports[0].values.len(), 5);
    }

    #[test]
    // Read and confirm inputs
    fn test_safety() {
        let input = Day2::read_input(EXAMPLE1);

        let expected = [true, false, false, false, false, true];

        for (n, safe) in expected.iter().enumerate() {
            assert_eq!(is_safe(&input.reports[n].values), *safe);
        }
    }

    #[test]
    // Read and confirm inputs
    fn test_damped_safety() {
        let input = Day2::read_input(EXAMPLE1);

        let expected = [true, false, false, true, true, true];

        for (n, safe) in expected.iter().enumerate() {
            assert_eq!(is_damped_safe(&input.reports[n].values), *safe);
        }
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day2::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(2));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day2::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(4));
    }
    
}
