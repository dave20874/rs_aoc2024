use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref BUTTON_A_RE: Regex = Regex::new("Button A: X([+-]\\d+), Y([+-]\\d+)").unwrap();
    static ref BUTTON_B_RE: Regex = Regex::new("Button B: X([+-]\\d+), Y([+-]\\d+)").unwrap();
    static ref PRIZE_RE: Regex = Regex::new("Prize: X=(\\d+), Y=(\\d+)").unwrap();
}

struct Game {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}

impl Game {
    fn soln(&self) -> Option<(isize, isize)> {
        let det = (self.a.0 * self.b.1) - (self.b.0 * self.a.1);
        // dbg!(det);

        if det != 0 {
            let det_x = (self.prize.0 * self.b.1) - (self.b.0 * self.prize.1);
            let det_y = (self.a.0 * self.prize.1) - (self.prize.0 * self.a.1);
            // dbg!(det_x);
            // dbg!(det_y);

            if (det_x % det == 0) & (det_y % det == 0) {
                Some( (det_x/det, det_y/det) )
            }
            else {
                // No integer solution
                None
            }
            
        }
        else {
            // No solution
            None
        }
    }
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    games: Vec<Game>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut games = Vec::new();
        let mut a = (0, 0);
        let mut b = (0, 0);
        let mut prize;
        for line in text.lines() {
            if let Some(cap) = BUTTON_A_RE.captures(line) {
                a = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
            }
            if let Some(cap) = BUTTON_B_RE.captures(line) {
                b = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
            }
            if let Some(cap) = PRIZE_RE.captures(line) {
                prize = (cap[1].parse().unwrap(), cap[2].parse().unwrap());
                games.push( Game { a, b, prize });
            }
        }
        Input { games }
    }

    fn tokens(&self) -> usize {
        let sum: isize = self.games.iter()
            .map(|g| { 
                g.soln() 
            } )
            .map(|s| { 
                match s {
                    Some( (a, b) ) => 3*a + b,
                    None => 0,
                }
            } )
            .sum();

        sum as usize
    }
}

pub struct Day13 {
}

// Day13
impl Day13 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day13 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.tokens())
    }

    fn part2(&self, text: &str) -> Answer {
        let _input = Input::read(text);

        Answer::None
    }
}

#[cfg(test)]

mod test {

    use crate::day13::{Day13, Input};
    use crate::day::{Day, Answer};
    
    // TODO Place example inputs here.
    const EXAMPLE1: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.games.len(), 4);
        assert_eq!(input.games[0].a, (94, 34));
        assert_eq!(input.games[0].b, (22, 67));
        assert_eq!(input.games[0].prize, (8400, 5400));
    }

    #[test]
    fn test_soln() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.games[0].soln(), Some( (80, 40) ));
        assert_eq!(input.games[1].soln(), None);
        assert_eq!(input.games[2].soln(), Some( (38, 86) ));
        assert_eq!(input.games[3].soln(), None);    
    }
    
    #[test]
    fn test_tokens() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.tokens(), 480);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day13::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(480));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day13::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}