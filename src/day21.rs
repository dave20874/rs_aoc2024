use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    static ref LINE_RE: Regex = Regex::new("(\\d{3})A").unwrap();
}

struct Code {
    buttons: String,
    numeric: usize,
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    codes: Vec<Code>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let codes = text.lines()
            .filter_map(|l| { 
                if let Some(caps) = LINE_RE.captures(l) {
                    let num = caps[1].parse::<usize>().unwrap();
                    Some( Code { buttons: caps[0].to_string(), numeric: num } )
                }
                else {
                    None
                }
            })
            .collect();

        Input { codes }
    }
}

struct ArrowKeypad<'a> {
    subpad: Option<&'a mut ArrowKeypad<'a>>,
    cache: HashMap<(char, char), usize>,
}

lazy_static! {
    // All the ways to move from first char to second on arrow keypad.
    static ref ARROWPAD_SEQUENCE: HashMap<(char, char), Vec<&'static str>> = HashMap::from([
        ( ('A', 'A'), vec![""] ),
        ( ('A', '^'), vec!["<"] ),
        ( ('A', '<'), vec!["<v<", "v<<"] ),
        ( ('A', 'v'), vec!["<v", "v<"] ),
        ( ('A', '>'), vec!["v"] ),

        ( ('^', 'A'), vec![">"] ),
        ( ('^', '^'), vec![""] ),
        ( ('^', '<'), vec!["v<"] ),
        ( ('^', 'v'), vec!["v"] ),
        ( ('^', '>'), vec!["v>", ">v"] ),

        ( ('<', 'A'), vec![">>^", ">^>"] ),
        ( ('<', '^'), vec![">^"] ),
        ( ('<', '<'), vec![""] ),
        ( ('<', 'v'), vec![">"] ),
        ( ('<', '>'), vec![">>"] ),
        
        ( ('v', 'A'), vec![">^", "^>"] ),
        ( ('v', '^'), vec!["^"] ),
        ( ('v', '<'), vec!["<"] ),
        ( ('v', 'v'), vec![""] ),
        ( ('v', '>'), vec![">"] ),

        ( ('>', 'A'), vec!["^"] ),
        ( ('>', '^'), vec!["<^", "^<"] ),
        ( ('>', '<'), vec!["<<"] ),
        ( ('>', 'v'), vec!["<"] ),
        ( ('>', '>'), vec![""] ),
    ]);
}

impl<'a> ArrowKeypad<'a> {

    
    fn new(indirect: Option<&'a mut ArrowKeypad<'a>>) -> ArrowKeypad<'a> {
        ArrowKeypad { subpad: indirect, cache: HashMap::new() }
    }

    fn strokes(&mut self, start: char, end: char) -> usize {
        // check cache
        if let Some(value) = self.cache.get(&(start, end)) {
            return *value;
        }

        let strokes = match &mut self.subpad {
            Some(subpad) => {
                let seq_vec = ARROWPAD_SEQUENCE.get(&(start, end)).unwrap();
                let min_strokes = seq_vec.iter()
                    .map(|seq| { 
                        let mut last = 'A';
                        let mut strokes = 0;
                        for c in seq.chars() {
                            strokes += subpad.strokes(last, c);
                            last = c;
                        }
                        strokes += subpad.strokes( last, 'A');

                        strokes
                     })
                    .min().unwrap();

                min_strokes
            }
            None => {
                1
            }
        };

        // Cache result
        self.cache.insert((start, end), strokes);

        strokes
    }
}

lazy_static! {
    static ref NUMPAD_SEQUENCE: HashMap<(char, char), Vec<&'static str>> = HashMap::from([
        // All the ways to move from first char to second on numeric keypad.
        ( ('A', 'A'), vec![""] ),
        ( ('A', '0'), vec!["<"] ),
        ( ('A', '1'), vec!["^<<", "<^<"] ),
        ( ('A', '2'), vec!["^<"] ),
        ( ('A', '3'), vec!["^"] ),
        ( ('A', '4'), vec!["^^<<", "^<^<", "^<<^", "<^^<", "<^<^"] ),
        ( ('A', '5'), vec!["^^<", "^<^", "<^^"] ),
        ( ('A', '6'), vec!["^^"] ),
        ( ('A', '7'), vec!["^^^<<", "^^<^<", "^^<<^", "^<^^<", "^<^<^", "^<<^^", "<^^^<", "<^<^^"] ),
        ( ('A', '8'), vec!["^^^<", "^^<^", "^<^^", "<^^^"] ),
        ( ('A', '9'), vec!["^^^"] ),

        ( ('0', 'A'), vec![">"] ),
        ( ('0', '0'), vec![""] ),
        ( ('0', '1'), vec!["^<"] ),
        ( ('0', '2'), vec!["^"] ),
        ( ('0', '3'), vec!["^>", ">^"] ),
        ( ('0', '4'), vec!["^^<", "^<^"] ),
        ( ('0', '5'), vec!["^^"] ),
        ( ('0', '6'), vec!["^^>", "^>^", ">^^"] ),
        ( ('0', '7'), vec!["^^^<", "^^<^", "^<^^"] ),
        ( ('0', '8'), vec!["^^^"] ),
        ( ('0', '9'), vec!["^^^>", "^^>^", "^>^^", ">^^^"] ),

        ( ('1', 'A'), vec![">>>v", ">>v>", ">v>>"] ),
        ( ('1', '0'), vec![">v", "v>"] ),
        ( ('1', '1'), vec![""] ),
        ( ('1', '2'), vec![">"] ),
        ( ('1', '3'), vec![">>"] ),
        ( ('1', '4'), vec!["^"] ),
        ( ('1', '5'), vec!["^>", ">^"] ),
        ( ('1', '6'), vec!["^>>", ">^>", ">>^"] ),
        ( ('1', '7'), vec!["^^"] ),
        ( ('1', '8'), vec!["^^>", "^>^", ">^^"] ),
        ( ('1', '9'), vec!["^^>>", "^>^>", "^>>^", ">^>^", ">>^^"] ),

        ( ('2', 'A'), vec![">v", "v>"] ),
        ( ('2', '0'), vec!["v"] ),
        ( ('2', '1'), vec!["<"] ),
        ( ('2', '2'), vec![""] ),
        ( ('2', '3'), vec![">"] ),
        ( ('2', '4'), vec!["^<", "<^"] ),
        ( ('2', '5'), vec!["^"] ),
        ( ('2', '6'), vec!["^>", ">^"] ),
        ( ('2', '7'), vec!["^^<", "^<^", "<^^"] ),
        ( ('2', '8'), vec!["^^"] ),
        ( ('2', '9'), vec!["^^>", "^>^", ">^^"] ),

        ( ('3', 'A'), vec!["v"] ),
        ( ('3', '0'), vec!["v<", "<v"] ),
        ( ('3', '1'), vec!["<<"] ),
        ( ('3', '2'), vec!["<"] ),
        ( ('3', '3'), vec![""] ),
        ( ('3', '4'), vec!["^<<", "<^<", "<<^"] ),
        ( ('3', '5'), vec!["^<", "<^"] ),
        ( ('3', '6'), vec!["^"] ),
        ( ('3', '7'), vec!["<<^^", "<^<^", "<^^<", "^<<^", "^<^<", "^^<<"] ),
        ( ('3', '8'), vec!["^^<", "^<^", "<^^"] ),
        ( ('3', '9'), vec!["^^"] ),

        ( ('4', 'A'), vec![">>vv", ">v>v", ">vv>", "v>>v", "v>v>"] ),
        ( ('4', '0'), vec![">vv", "v>v"] ),
        ( ('4', '1'), vec!["v"] ),
        ( ('4', '2'), vec![">v"] ),
        ( ('4', '3'), vec![">>v", ">v>", "v>>"] ),
        ( ('4', '4'), vec![""] ),
        ( ('4', '5'), vec![">"] ),
        ( ('4', '6'), vec![">>"] ),
        ( ('4', '7'), vec!["^"] ),
        ( ('4', '8'), vec!["^>", ">^"] ),
        ( ('4', '9'), vec!["^>>", ">^>", ">>^"] ),

        ( ('5', 'A'), vec!["vv>", "v>v", ">vv"] ),
        ( ('5', '0'), vec!["vv"] ),
        ( ('5', '1'), vec!["v<", "<v"] ),
        ( ('5', '2'), vec!["v"] ),
        ( ('5', '3'), vec!["v>", ">v"] ),
        ( ('5', '4'), vec!["<"] ),
        ( ('5', '5'), vec![""] ),
        ( ('5', '6'), vec![">"] ),
        ( ('5', '7'), vec!["^<", "<^"] ),
        ( ('5', '8'), vec!["^"] ),
        ( ('5', '9'), vec!["^>", ">^"] ),

        ( ('6', 'A'), vec!["vv"] ),
        ( ('6', '0'), vec!["vv<", "v<v", "<vv"] ),
        ( ('6', '1'), vec!["v<<", "<v<", "<<v"] ),
        ( ('6', '2'), vec!["v<", "<v"] ),
        ( ('6', '3'), vec!["v"] ),
        ( ('6', '4'), vec!["<<"] ),
        ( ('6', '5'), vec!["<"] ),
        ( ('6', '6'), vec![""] ),
        ( ('6', '7'), vec!["^<<", "<^<", "<<^"] ),
        ( ('6', '8'), vec!["^<", "^<"] ),
        ( ('6', '9'), vec!["^"] ),

        ( ('7', 'A'), vec![">>vvv", ">v>vv", ">vv>v", ">vvv>", "v>>vv", "v>v>v", "v>vv>", "vv>>v", "vv>v>"] ),
        ( ('7', '0'), vec![">vvv", "v>vv", "vv>v"] ),
        ( ('7', '1'), vec!["vv"] ),
        ( ('7', '2'), vec!["vv>", "v>v", ">vv"] ),
        ( ('7', '3'), vec!["vv>>", "v>v>", "v>>v", ">vv>", ">v>v", ">>vv"] ),
        ( ('7', '4'), vec!["v"] ),
        ( ('7', '5'), vec!["v>", ">v"] ),
        ( ('7', '6'), vec!["v>>", ">v>", ">>v"] ),
        ( ('7', '7'), vec![""] ),
        ( ('7', '8'), vec![">"] ),
        ( ('7', '9'), vec![">>"] ),

        ( ('8', 'A'), vec![">vvv", "v>vv", "vv>v", "vvv>"] ),
        ( ('8', '0'), vec!["vvv"] ),
        ( ('8', '1'), vec!["vv<", "v<v", "<vv"] ),
        ( ('8', '2'), vec!["vv"] ),
        ( ('8', '3'), vec!["vv>", "v>v", ">vv"] ),
        ( ('8', '4'), vec!["v<", "<v"] ),
        ( ('8', '5'), vec!["v"] ),
        ( ('8', '6'), vec!["v>", ">v"] ),
        ( ('8', '7'), vec!["<"] ),
        ( ('8', '8'), vec![""] ),
        ( ('8', '9'), vec![">"] ),

        ( ('9', 'A'), vec!["vvv"] ),
        ( ('9', '0'), vec!["vvv<", "vv<v", "v<vv", "<vvv"] ),
        ( ('9', '1'), vec!["vv<<", "v<v<", "v<<v", "<v<v", "<<vv"] ),
        ( ('9', '2'), vec!["vv<", "v<v", "<vv"] ),
        ( ('9', '3'), vec!["vv"] ),
        ( ('9', '4'), vec!["v<<", "<v<", "<<v"] ),
        ( ('9', '5'), vec!["v<", "<v"] ),
        ( ('9', '6'), vec!["v"] ),
        ( ('9', '7'), vec!["<<"] ),
        ( ('9', '8'), vec!["<"] ),
        ( ('9', '9'), vec![""] ),
    ]);
}

struct NumKeypad<'a> {
    subpad: Option<&'a mut ArrowKeypad<'a>>,
}

impl<'a> NumKeypad<'a> {
    fn new(pad: Option<&'a mut ArrowKeypad<'a>>) -> NumKeypad<'a> {
        NumKeypad { subpad: pad }
    }

    fn strokes(&mut self, start: char, end: char) -> usize {
        let strokes = match &mut self.subpad {
            Some(subpad) => {
                // Generate arrow pad sequence from start to end and evaluate the
                // strokes needed by the subpad
                // let seq = NUMPAD_SEQUENCE.get(&(start, end)).unwrap();

                let seq_vec = NUMPAD_SEQUENCE.get(&(start, end)).unwrap();
                let min_strokes = seq_vec.iter()
                    .map(|seq| { 
                        let mut last = 'A';
                        let mut strokes = 0;
                        for c in seq.chars() {
                            strokes += subpad.strokes(last, c);
                            last = c;
                        }
                        strokes += subpad.strokes(last, 'A');

                        strokes
                     })
                    .min().unwrap();

                min_strokes
            }
            None => {
                1
            }
        };

        strokes
    }

    fn seq_len(&mut self, seq: &str) -> usize {

        let mut last = 'A';
        let mut strokes = 0;

        for c in seq.chars() {
            strokes += self.strokes(last, c);
            last = c;
        }

        strokes
    }
}

pub struct Day21 {
}

// Day21
impl Day21 {
    pub const fn new() -> Self {
        Self { }
    }

    fn complexity(input: &Input) -> usize {

        // create numeric keypad with no indirection
        let mut pad3 = ArrowKeypad::new(None);
        let mut pad2 = ArrowKeypad::new(Some(&mut pad3));
        let mut pad1 = ArrowKeypad::new(Some(&mut pad2));
        let mut pad0 = NumKeypad::new(Some(&mut pad1));

        let mut sum = 0;
        for code in &input.codes {
            let len = pad0.seq_len(&code.buttons);

            sum += len * code.numeric;
        }

        sum
    }

    fn complexity2(input: &Input) -> usize {

        // create numeric keypad with no indirection
        let mut pad26 = ArrowKeypad::new(None);

        let mut pad25 = ArrowKeypad::new(Some(&mut pad26));
        let mut pad24 = ArrowKeypad::new(Some(&mut pad25));
        let mut pad23 = ArrowKeypad::new(Some(&mut pad24));
        let mut pad22 = ArrowKeypad::new(Some(&mut pad23));
        let mut pad21 = ArrowKeypad::new(Some(&mut pad22));
        let mut pad20 = ArrowKeypad::new(Some(&mut pad21));
        let mut pad19 = ArrowKeypad::new(Some(&mut pad20));
        let mut pad18 = ArrowKeypad::new(Some(&mut pad19));
        let mut pad17 = ArrowKeypad::new(Some(&mut pad18));
        let mut pad16 = ArrowKeypad::new(Some(&mut pad17));
        let mut pad15 = ArrowKeypad::new(Some(&mut pad16));
        let mut pad14 = ArrowKeypad::new(Some(&mut pad15));
        let mut pad13 = ArrowKeypad::new(Some(&mut pad14));
        let mut pad12 = ArrowKeypad::new(Some(&mut pad13));
        let mut pad11 = ArrowKeypad::new(Some(&mut pad12));
        let mut pad10 = ArrowKeypad::new(Some(&mut pad11));
        let mut pad9 = ArrowKeypad::new(Some(&mut pad10));
        let mut pad8 = ArrowKeypad::new(Some(&mut pad9));
        let mut pad7 = ArrowKeypad::new(Some(&mut pad8));
        let mut pad6 = ArrowKeypad::new(Some(&mut pad7));
        let mut pad5 = ArrowKeypad::new(Some(&mut pad6));
        let mut pad4 = ArrowKeypad::new(Some(&mut pad5));
        let mut pad3 = ArrowKeypad::new(Some(&mut pad4));
        let mut pad2 = ArrowKeypad::new(Some(&mut pad3));
        let mut pad1 = ArrowKeypad::new(Some(&mut pad2));

        let mut pad0 = NumKeypad::new(Some(&mut pad1));

        let mut sum = 0;
        for code in &input.codes {
            let len = pad0.seq_len(&code.buttons);

            sum += len * code.numeric;
        }

        sum
    }
}

impl<'a> Day for Day21 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(Self::complexity(&input))
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(Self::complexity2(&input))
    }
}

#[cfg(test)]

mod test {

    use crate::day21::{Day21, Input, NumKeypad, ArrowKeypad};
    use crate::day::{Day, Answer};
    
    // Example inputs
    const EXAMPLE1: &str = "\
029A
980A
179A
456A
379A
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.codes.len(), 5);
        assert_eq!(input.codes[0].buttons, "029A");
        assert_eq!(input.codes[0].numeric, 29);
    }

    #[test]
    fn test_num_keypad() {
        // create numeric keypad with no indirection
        let mut pad0 = NumKeypad::new(None);

        assert_eq!(pad0.seq_len("029A"), 4);
    }

    
    #[test]
    fn test_indirect_num_keypad() {
        // create numeric keypad with no indirection
        let mut pad1 = ArrowKeypad::new(None);
        let mut pad0 = NumKeypad::new(Some(&mut pad1));

        assert_eq!(pad0.seq_len("029A"), 12);
    }

        
    #[test]
    fn test_most_indirect_num_keypad() {
        // create numeric keypad with no indirection
        let mut pad3 = ArrowKeypad::new(None);
        let mut pad2 = ArrowKeypad::new(Some(&mut pad3));
        let mut pad1 = ArrowKeypad::new(Some(&mut pad2));
        let mut pad0 = NumKeypad::new(Some(&mut pad1));

        // assert_eq!(
        //     pad0.seq_len("029A"), 
        //     "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len());
        assert_eq!(
            pad0.seq_len("379A"), 
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len());
    }

    #[test]
    fn test_complexity() {
        let input = Input::read(EXAMPLE1);
        assert_eq!(Day21::complexity(&input), 126384);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1_ex1() {
        // Based on the example in part 1.
        let d= Day21::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(126384));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the actual input
        let d= Day21::new();
        let answer = d.part1(data_aoc2024::DAY21_INPUT);
        match answer {
            Answer::Numeric(val) => {
                assert_eq!(val, 206798);
            }
            _ => {
                assert!(false, "Bad answer type");
            }
        }
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the actual input
        let d= Day21::new();
        let answer = d.part2(data_aoc2024::DAY21_INPUT);
        match answer {
            Answer::Numeric(val) => {
                assert_eq!(val, 251508572750680);
            }
            _ => {
                assert!(false, "Bad answer type");
            }
        }
    }
    
}