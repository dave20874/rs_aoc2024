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

pub struct Day2<'a> {
    input: &'a str,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut ascending = false;
        let mut descending = false;
        let mut equal = false;
        let mut max_diff = 0;


        for n in 0..self.values.len()-1 {
            let mut diff = 0;
            if self.values[n] < self.values[n+1] {
                ascending = true;
                diff = self.values[n+1] - self.values[n];
            } 
            else if self.values[n] > self.values[n+1] {
                descending = true;
                diff = self.values[n] - self.values[n+1];
            }
            else {
                equal = true;
            }

            if diff > max_diff {
                max_diff = diff;
            }
        }

        return !equal & !(ascending & descending) & (max_diff <= 3);
    }

    fn is_damped_safe(&self) -> bool {
        let mut found_safe = false;
        let mut shortened_values: Vec<usize> = Vec::new();

        for skip in 0..self.values.len() {
            // Construct a set of values without the skipped one.
            shortened_values.clear();

            for n in 0..self.values.len() {
                if n != skip {
                    shortened_values.push(self.values[n]);
                }
            }
            let shortened_report = Report { values: shortened_values.clone() };

            if shortened_report.is_safe() {
                // println!("Found safe with sample {skip} removed: {shortened_values:#?}");
                found_safe = true;
                break;
            }
        }

        found_safe
    }
}

// Day2
impl<'a> Day2<'a> {
    pub const fn new(input: &'a str) -> Self {
        Self { input: input }
    }

    fn read_input(input: &'a str) -> Input
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

impl<'a> Day for Day2<'a> {

    // Compute Part 1 solution
    fn part1(&self) -> Answer {
        let input = Self::read_input(self.input);

        let num_safe = input.reports.iter().filter(|r| r.is_safe()).count();

        Answer::Numeric(num_safe)
    }

    fn part2(&self) -> Answer {
        let input = Self::read_input(self.input);

        let num_safe = input.reports.iter().filter(|r| r.is_damped_safe()).count();

        Answer::Numeric(num_safe)
    }
}

#[cfg(test)]

mod test {

    use crate::day2::Day2;
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
                
        assert_eq!(input.reports[0].is_safe(), true);
        assert_eq!(input.reports[1].is_safe(), false);
        assert_eq!(input.reports[2].is_safe(), false);
        assert_eq!(input.reports[3].is_safe(), false);
        assert_eq!(input.reports[4].is_safe(), false);
        assert_eq!(input.reports[5].is_safe(), true);
    }

    #[test]
    // Read and confirm inputs
    fn test_damped_safety() {
        let input = Day2::read_input(EXAMPLE1);
                
        assert_eq!(input.reports[0].is_damped_safe(), true);
        assert_eq!(input.reports[1].is_damped_safe(), false);
        assert_eq!(input.reports[2].is_damped_safe(), false);
        assert_eq!(input.reports[3].is_damped_safe(), true);
        assert_eq!(input.reports[4].is_damped_safe(), true);
        assert_eq!(input.reports[5].is_damped_safe(), true);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day2::new(EXAMPLE1);

        assert_eq!(d.part1(), Answer::Numeric(2));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day2::new(EXAMPLE1);
        assert_eq!(d.part2(), Answer::Numeric(4));
    }
    
}
