use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {


    // Matches "do()", "don't()" or "mul(NNN,NNN)" with NNN's captured in cap[1] and cap[2]
    // Breaking this down, the RE is basically "do() | don't() | mul(NNN, NNN)"
    // The parens require a lot of escaping and there are no spaces to visibly break up the three
    // main components, but it's not as complicated as it looks.
    static ref INSTR_RE: Regex = Regex::new("do\\(\\)|don't\\(\\)|mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
}

// Three types of instructions we're looking for.
enum Instr {
    Mul(usize, usize),
    Do,
    Dont,
}

// A representation of the puzzle inputs: A vector of instructions
struct Input {
    instructions: Vec<Instr>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut instructions: Vec<Instr> = Vec::new();

        // For every capture in all the lines, push the appropriate instruction enum.
        for line in text.lines() {
            for cap in INSTR_RE.captures_iter(line) {
                match &cap[0] {
                    "don't()" => {
                        // println!("Found DONT:: {}", &cap[0]);
                        instructions.push(Instr::Dont);
                    }
                    "do()" => {
                        // println!("Found DO: {}", &cap[0]);
                        instructions.push(Instr::Do);
                    }
                    _ => {
                        // must be MUL
                        // println!("Found MUL: {}", &cap[0]);
                        instructions.push(Instr::Mul(cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap()));
                    }
                }
            }
        }

        Input { instructions }
    }

    // sum all the multiplies, regardless of do/don't instructions.
    fn sum_mul_unconditional(&self) -> usize
    {
        let mut sum: usize = 0;

        for instr in &self.instructions {
            match instr {
                Instr::Mul(a, b) => { sum += a*b; }
                Instr::Do => {}
                Instr::Dont => {}
            }
        }

        sum
    }

    // sum all multiplies with do/don't instructions honored.
    fn sum_mul_conditional(&self) -> usize
    {
        let mut enabled = true;
        let mut sum: usize = 0;

        for instr in &self.instructions {
            match instr {
                Instr::Mul(a, b) => if enabled { sum += a*b; }
                Instr::Do => { enabled = true; }
                Instr::Dont => { enabled = false; }
            }
        }

        sum
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

// Implement the usual interface for daily solver.
impl Day for Day3 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let sum = input.sum_mul_unconditional();

        Answer::Numeric(sum)
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let sum = input.sum_mul_conditional();

        Answer::Numeric(sum)
    }
}

#[cfg(test)]
mod test {
    use crate::day3::{Day3, Input};
    use crate::day::{Day, Answer};
    
    // Part 1 Example
    const EXAMPLE1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))    
";

// Part 2 Example
    const EXAMPLE2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))  
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        // Read example 1
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.instructions.len(), 4);
        assert_eq!(input.sum_mul_unconditional(), 161);
    }

    #[test]
    fn test_read2() {
        // read example 2
        let input = Input::read(EXAMPLE2);

        assert_eq!(input.instructions.len(), 6);
        assert_eq!(input.sum_mul_unconditional(), 161);
        assert_eq!(input.sum_mul_conditional(), 48);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day3::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(161));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day3::new();
        assert_eq!(d.part2(EXAMPLE2), Answer::Numeric(48));
    }
    
}