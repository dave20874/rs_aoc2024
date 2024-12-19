use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref REG_A_RE: Regex = Regex::new("Register A: (\\d+)").unwrap();
    static ref REG_B_RE: Regex = Regex::new("Register B: (\\d+)").unwrap();
    static ref REG_C_RE: Regex = Regex::new("Register C: (\\d+)").unwrap();
    static ref PROG_RE: Regex  = Regex::new("Program: (\\d(,\\d)*)").unwrap();
    static ref INT_RE: Regex   = Regex::new("(\\d+)").unwrap();
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut program = Vec::new();
        for line in text.lines() {
            if let Some(caps) = REG_A_RE.captures(line) {
                a = caps[1].parse::<usize>().unwrap();
            }
            if let Some(caps) = REG_B_RE.captures(line) {
                b = caps[1].parse::<usize>().unwrap();
            }
            if let Some(caps) = REG_C_RE.captures(line) {
                c = caps[1].parse::<usize>().unwrap();
            }
            if let Some(caps) = PROG_RE.captures(line) {
                for num_cap in INT_RE.captures_iter(&caps[1]) {
                    let num = num_cap[1].parse::<usize>().unwrap();
                    program.push(num);
                }
            }
        }

        Input { a, b, c, program }
    }
}

struct Machine {
    cycles: usize,
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    program: Vec<usize>,
    output: Vec<usize>,
}

impl Machine {
    fn new(input: &Input) -> Machine {

        // Convert program string to Vec<usize>
        Machine { 
            cycles: 0,
            a: input.a, 
            b: input.b,
            c: input.c,
            ip: 0,
            program: input.program.clone(),
            output: Vec::new()
        }
    }

    // Evaluate combo operand
    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..4 => { operand },
            4 => { self.a },
            5 => { self.b },
            6 => { self.c },
            _ => {
                panic!("Illegal combo operand");
            }
        }
    }

    fn step(&mut self) {
        self.cycles += 1;

        // fetch opcode and operand
        let opcode = self.program[self.ip];
        let operand = self.program[self.ip+1];
        self.ip += 2;

        // decode opcode
        match opcode {
            0 => {  // ADV
                let denom = 1 << self.combo(operand);
                // println!("{} {} ADV: {denom}", self.cycles, self.ip);
                self.a = self.a / denom;
            }
            1 => {  // BXL
                self.b = self.b ^ operand;
            }
            2 => {  // BST
                self.b = self.combo(operand) & 7;
            }
            3 => {  // JNZ
                if self.a != 0 {
                    self.ip = operand;
                }
            }
            4 => {  // BXC
                self.b = self.b ^ self.c;
            }
            5 => {  // OUT
                let output = self.combo(operand) & 7;
                // println!("{} {} Output: {output}", self.cycles, self.ip);
                self.output.push(output);
            }
            6 => {  // BDV
                self.b = self.a / (1 << self.combo(operand))
            }
            7 => {  // CDV
                self.c = self.a / (1 << self.combo(operand))
            }
            _ => {
                panic!("Illegal opcode {opcode}")
            }
        }

    }

    fn run_to_halt(&mut self) -> String {
        let end = self.program.len();
        while self.ip < end {
            self.step();
        }
        
        // convert output to comma separated string
        let mut out = String::new();
        for (n, val) in self.output.iter().enumerate() {
            if n > 0 {
                out += ",";
            }
            out.push_str(&format!("{val}"));
        }

        out
    }
}

pub struct Day17 {
}

// Day17
impl Day17 {
    pub const fn new() -> Self {
        Self { }
    }

    // At a = 1 << (3*(N-1)), we see the first sequence with N digits
    // We should see all the distinct values of the last digit at intervals
    // of 1 << (3*(N-2)).
    // We can work this into a search algorithm that only needs 8*(N digits) checks.
    fn search(input: &Input) -> usize {
        let prog_len = input.program.len();

        // println!("Searching for {:?}", input.program);

        let mut base = 0;

        // Loop breaks when full answer is found
        loop {
            // Run the program with a at our current base
            let mut machine = Machine::new(&input);
            // println!("base: {base}");
            machine.a = base;
            machine.run_to_halt();

            // Figure out highest incorrect digit
            let mut highest_err = None;
            for i in (0..prog_len).rev() {
                if machine.output.len() < i {
                    highest_err = Some(i);
                    break;
                }
                if machine.output[i] != input.program[i] {
                    highest_err = Some(i);
                    break;                    
                }
            }
            match highest_err {
                None => {
                    // We found the full match at base
                    // println!("Found {:?}", machine.output);
                    break;
                }
                Some(place) => {
                    let interval = 1 << 3*(place);
                    // println!("mismatch at place {place} base: {base}+{interval} -> {}", base+interval);
                    base += interval;
                }
            }
        }

        base
    }

}

impl<'a> Day for Day17 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let mut machine = Machine::new(&input);

        Answer::String(machine.run_to_halt())
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(Self::search(&input))
    }
}

#[cfg(test)]

mod test {

    use crate::day17::{Day17, Input, Machine};
    use crate::day::{Day, Answer};
    use data_aoc2024::DAY17_INPUT;
    
    // Example Inputs
    const EXAMPLE1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.a, 729);
        assert_eq!(input.b, 0);
        assert_eq!(input.c, 0);
        assert_eq!(input.program.len(), 6);
        assert_eq!(input.program, vec![0,1,5,4,3,0]);
    }

    #[test]
    // Read and confirm inputs
    fn test_machine() {
        let input = Input::read(EXAMPLE1);

        let mut machine = Machine::new(&input);
        let s = machine.run_to_halt();
        assert_eq!(s, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_search() {
        // length of output as a function of a:
        // 0-7 : 1
        // 8-64 : 2
        // 65-512 : 3
        // N : log8(N)
        // I think we can dial in each digit starting from the last : 
        // Try 8 numbers, separated by 8^(N-1), 
        // Then try numbers separated by 8^(N-2) to get next digit, etc.

        let input = Input::read(DAY17_INPUT);
        assert_eq!(Day17::search(&input), 164540892147389);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day17::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::String("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day17::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(164540892147389));
    }
    
}