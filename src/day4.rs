use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};

struct Check {
    // Vec of (row_offset, col_offset, char)
    pub checks: Vec<(usize, usize, char)>,
}

lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref LINE_RE: Regex = Regex::new("(\\d+)\\s+(\\d+)").unwrap();

    static ref MAS_X_CHECKS: Vec<Check> = vec![
        Check { checks: vec![(0, 0, 'M'), (0, 2, 'S'), (1, 1, 'A'), (2, 0, 'M'), (2, 2, 'S')] },// orientation 0
        Check { checks: vec![(0, 0, 'M'), (0, 2, 'M'), (1, 1, 'A'), (2, 0, 'S'), (2, 2, 'S')] }, // orientation 1
        Check { checks: vec![(0, 0, 'S'), (0, 2, 'M'), (1, 1, 'A'), (2, 0, 'S'), (2, 2, 'M')] }, // orientation 2
        Check { checks: vec![(0, 0, 'S'), (0, 2, 'S'), (1, 1, 'A'), (2, 0, 'M'), (2, 2, 'M')] }, // orientation 3
    ];

    static ref XMAS_CHECKS: Vec<Check> = vec![
        Check { checks: vec![(0, 0, 'X'), (0, 1, 'M'), (0, 2, 'A'), (0, 3, 'S')] }, // orientation 0
        Check { checks: vec![(0, 3, 'X'), (0, 2, 'M'), (0, 1, 'A'), (0, 0, 'S')] }, // orientation 0
        Check { checks: vec![(0, 0, 'X'), (1, 0, 'M'), (2, 0, 'A'), (3, 0, 'S')] }, // orientation 0
        Check { checks: vec![(3, 0, 'X'), (2, 0, 'M'), (1, 0, 'A'), (0, 0, 'S')] }, // orientation 0
        Check { checks: vec![(0, 0, 'X'), (1, 1, 'M'), (2, 2, 'A'), (3, 3, 'S')] }, // orientation 0
        Check { checks: vec![(3, 3, 'X'), (2, 2, 'M'), (1, 1, 'A'), (0, 0, 'S')] }, // orientation 0
        Check { checks: vec![(0, 3, 'X'), (1, 2, 'M'), (2, 1, 'A'), (3, 0, 'S')] }, // orientation 0
        Check { checks: vec![(3, 0, 'X'), (2, 1, 'M'), (1, 2, 'A'), (0, 3, 'S')] }, // orientation 0
    ];
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    // puzzle[row][col] -> char
    puzzle: Vec<Vec<char>>,
}

impl Input {


    fn read(text: &str) -> Input
    {
        let mut puzzle: Vec<Vec<char>> = Vec::new();

        for line in text.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            puzzle.push(row);
        }

        Input { puzzle }
    }

    fn checks_out(&self, checks: &Vec<Check>, origin: (usize, usize), orientation: usize) -> bool
    {
        for (row_offs, col_offs, c) in &checks[orientation].checks {
            let row = origin.0+row_offs;
            let col = origin.1+col_offs;
            if row >= self.puzzle.len() { return false; }
            if col >= self.puzzle[0].len() { return false; }

            if self.puzzle[row][col] != *c {
                return false;
            }
        }

        true
    }

    fn total_xmas(&self) -> usize
    {
        let mut count = 0;

        for row in 0..self.puzzle.len() {
            for col in 0..self.puzzle[0].len() {
                let orientations = XMAS_CHECKS.len();

                for orientation in 0..orientations {
                    if self.checks_out(&XMAS_CHECKS, (row, col), orientation) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
    
    fn total_mas_x(&self) -> usize
    {
        let mut count = 0;

        for row in 0..self.puzzle.len() {
            for col in 0..self.puzzle[0].len() {
                let orientations = MAS_X_CHECKS.len();

                for orientation in 0..orientations {
                    if self.checks_out(&MAS_X_CHECKS, (row, col), orientation) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

pub struct Day4 {
}

// Day4
impl Day4 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day4 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.total_xmas())
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.total_mas_x())
    }
}

#[cfg(test)]

mod test {

    use crate::day4::{Day4, Input, XMAS_CHECKS, MAS_X_CHECKS};
    use crate::day::{Day, Answer};
    
    // Example inputs
    const EXAMPLE1: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.puzzle.len(), 10);
        assert_eq!(input.puzzle[0].len(), 10);
        assert_eq!(input.puzzle[0][0], 'M');
        assert_eq!(input.puzzle[2][0], 'A');
        assert_eq!(input.puzzle[0][2], 'M');
    }

    #[test]
    fn test_is_xmas() {
        let input= Input::read(EXAMPLE1);

        // Is there XMAS at 0, 0 in orientation 0? No.
        assert_eq!(input.checks_out(&XMAS_CHECKS, (0, 0), 0), false);

        // Is there XMAS at (0, 5), in orientation 0? Yes.
        assert_eq!(input.checks_out(&XMAS_CHECKS, (0, 5), 0), true);

        // Is there XMAS at (2, 2), in orientation 1? No.
        assert_eq!(input.checks_out(&XMAS_CHECKS, (2, 2), 1), false);
    }

    #[test]
    fn test_total_xmas() {
        let input= Input::read(EXAMPLE1);

        // Count all XMAS.
        assert_eq!(input.total_xmas(), 18);
    }

    #[test]
    fn test_is_mas_x() {
        let input= Input::read(EXAMPLE1);

        // Is there MAS_X centered at 1, 1 in orientation 0? No.
        assert_eq!(input.checks_out(&MAS_X_CHECKS, (0, 0), 0), false);

        // Is there XMAS at (1, 2) in orientation 0? Yes.
        assert_eq!(input.checks_out(&MAS_X_CHECKS, (0, 1), 0), true);

        // Is there XMAS at (2, 6), in orientation 1? Yes.
        assert_eq!(input.checks_out(&MAS_X_CHECKS, (1, 5), 1), true);
    }

    #[test]
    fn test_total_mas_x() {
        let input= Input::read(EXAMPLE1);

        // Count all the MAS X's
        assert_eq!(input.total_mas_x(), 9);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day4::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(18));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day4::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(9));
    }
    
}