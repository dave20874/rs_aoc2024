use std::cmp::Ordering;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN|MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref PAIR_RE: Regex = Regex::new("(\\d+)\\|(\\d+)").unwrap();

    // Produces runs of digits from captures_iter().  capture[1] contains digits.
    static ref INTEGER_RE: Regex = Regex::new("(\\d+)").unwrap();
}

// A representation of the puzzle inputs.
struct Input {
    pairs: Vec<(usize, usize)>,  // the ordering constraings
    updates: Vec<Vec<usize>>,    // updates to check/correct.
}

impl Input {
    // Read input text, constructing Input.
    fn read(text: &str) -> Input
    {
        let mut pairs = Vec::new();
        let mut updates = Vec::new();

        let mut in_pairs = true;

        for line in text.lines() {
            if in_pairs {
                if let Some(caps) = PAIR_RE.captures(line) {
                    // Process pair, caps[1] is first elt, caps[2] is second elt.
                    pairs.push((caps[1].parse().unwrap(), caps[2].parse().unwrap()));
                }
                else {
                    // Input didn't match the pair pattern, it must be the blank line
                    // switch to processing updates
                    in_pairs = false;
                }
            }
            else {
                let update: Vec<usize> = INTEGER_RE.captures_iter(line).map(|caps| caps[1].parse().unwrap()).collect();
                updates.push(update);
            }
        }

        Input { pairs, updates }
    }
}

// Verify that an update has pages in the right order.
// This simply examines each adjacent pair of pages in the update and
// checks to see if there's an ordering specified for these two.
// There's no attempt to check transitive ordering.
// (Fortunately, this works.  Advent of Code made this easy today!)
fn simple_verify(pairs: &Vec<(usize, usize)>, update: &Vec<usize>) -> bool {
    // println!("Verifying {update:?}");

    for window in update.windows(2) {
        let a = window[0];
        let b = window[1];

        if pairs.contains(&(b, a)) { 
            return false; 
        } else if !pairs.contains(&(a, b)) { 
            panic!("Simple checks didn't work!");
        }
    }

    true
}

// A comparison rule to enable sorting updates according to the ordering rules.
// Again, there's no attempt to do transitive comparisons.  And, again, we get away with it.
fn simple_ordering(pairs: &Vec<(usize, usize)>, a: &usize, b: &usize) -> Ordering {
    if a == b { return Ordering::Equal; }
    if pairs.contains(&(*a, *b)) { return Ordering::Less; }
    if pairs.contains(&(*b, *a)) { return Ordering::Greater }

    panic!("simple ordering failed you.");
}

pub struct Day5 {
    // This struct exists to support the Day trait.
}

// Day5
impl Day5 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day5 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let sum = input.updates.iter()
        // updates that verify
            .filter(|upd| simple_verify(&input.pairs, upd)) 

            // take middle element
            .map(|upd| upd[upd.len()/2]) 

            // sum
            .sum(); 

        Answer::Numeric(sum)
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let sum = input.updates.iter()
            // updates that don't verify
            .filter(|upd| !simple_verify(&input.pairs, upd)) 

            // clone and sort the update
            .map(|upd| {                       
                    let mut c = upd.clone(); 
                    c.sort_by(|a, b| simple_ordering(&input.pairs, a, b)); 
                    c 
                })

            // get middle element
            .map(|upd| upd[upd.len()/2])

            // sum those.
            .sum();

        Answer::Numeric(sum)
    }
}

#[cfg(test)]

mod test {

    use crate::day5::{Day5, Input, simple_verify};
    use crate::day::{Day, Answer};
    
    // Example inputs
    const EXAMPLE1: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.pairs.len(), 21);
        assert_eq!(input.updates.len(), 6);
    }

    #[test]
    fn test_simple_verify() {
        let input = Input::read(EXAMPLE1);

        let expected = vec![true, true, true, false, false, false];

        for (n, update) in input.updates.iter().enumerate() {
            assert_eq!(simple_verify(&input.pairs, update), expected[n]);
        }
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day5::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(143));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day5::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(123));
    }
    
}