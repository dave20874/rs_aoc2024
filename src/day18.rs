use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};

const START_TIME: usize = 1024;
const PROBLEM_SIZE: usize = 71;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new("(\\d+),(\\d+)").unwrap();
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    size: usize,
    coords: Vec<(usize, usize)>,
}

impl Input {
    fn read(text: &str, size: usize) -> Input
    {
        let mut coords = Vec::new();

        for line in text.lines() {
            if let Some(caps) = LINE_RE.captures(line) {
                let x = caps[1].parse().unwrap();
                let y = caps[2].parse().unwrap();
                coords.push( (x, y) );
            }
        }

        Input { size, coords }
    }
}

pub struct Day18 {
    start_t: usize,
    prob_size: usize,
}

// Day18
impl Day18 {
    pub const fn new() -> Self {
        Self { start_t: START_TIME, prob_size: PROBLEM_SIZE }
    }

    fn solve(input: &Input, t: usize) -> Option<usize> {

        // Generate the set of blocked cells at time t
        let mut blocked: HashSet<(usize, usize)> = HashSet::new();
        let mut n = 0;
        for block in input.coords.iter() {
            blocked.insert(*block);
            n += 1;
            if n == t { break; }
        }

        // Set out to explore the map, starting at (0, 0) at t=1
        // Record the time, in steps, to reach each cell.
        // (quit when end is reached.)
        let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
        let mut distance_to: HashMap<(usize, usize), usize> = HashMap::new();

        distance_to.insert((0, 0), 0);
        frontier.push_back((0, 0));

        'outer: while let Some((x, y)) = frontier.pop_front() {
            // println!("Exploring from: {}, {}", x, y);
            // explore from x, y
            let t = *distance_to.get(&(x, y)).unwrap();

            // Assemble neighbors
            let mut neighbors = Vec::new();
            if y > 0            { neighbors.push( (x, y-1) ); }  // North neighbor
            if y < input.size-1 { neighbors.push( (x, y+1) ); }  // South neighbor
            if x > 0            { neighbors.push( (x-1, y) ); }  // West neighbor
            if x < input.size-1 { neighbors.push( (x+1, y) ); }  // East neighbor

            for next in &neighbors {
                // println!(" Trying neighbor: {}, {}", next.0, next.1);
                if !blocked.contains(next) {
                    if !distance_to.contains_key(next) {
                        distance_to.insert(*next, t+1);
                        frontier.push_back(*next);
                        // println!("    Reached {},{} in {} steps", next.0, next.1, t+1);

                        // Did we reach the end?
                        if *next == (input.size-1, input.size-1) {
                            break 'outer;
                        }
                    }
                    else {
                        // println!("    Been there.")
                    }
                }
                else {
                    // println!("    Blocked.");
                }
            }
        }

        distance_to.get( &(input.size-1, input.size-1) ).copied()
    }

    fn find_cutoff(input: &Input, start_t: usize) -> Option<(usize, usize)> {
        let mut low: usize = start_t;
        let mut high: usize = start_t;

        // grow high until it's high enough to cut off the path
        let mut soln = Self::solve(input, high);
        while let Some(_dist) = soln {
            high *= 2;
            soln = Self::solve(input, high);
        }

        // now binary search between low and high

        while high-low > 1 {
            let mid = (low+high)/2;
            soln = Self::solve(input, mid);
            match soln {
                Some(_dist) => {
                    // Solved, raise low to mid
                    low = mid;
                }
                None => {
                    // Blocked, lower high to mid
                    high = mid;
                }
            }
        }

        // high is the first setting where it's blocked.
        let cutoff = Some(input.coords[high-1]);

        cutoff
    }
}

impl<'a> Day for Day18 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text, self.prob_size);

        match Self::solve(&input, self.start_t) {
            Some(n) => {
                Answer::Numeric(n)
            }
            None => {
                Answer::None
            }
        }
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text, self.prob_size);

        let cutoff = Day18::find_cutoff(&input, self.start_t);

        match cutoff {
            Some((x, y)) => {
                let mut out = String::new();
                out.push_str(&format!("{},{}", x, y).to_string());
                Answer::String(out)
            }
            None => Answer::None
        }
    }
}

#[cfg(test)]

mod test {

    use crate::day18::{Day18, Input};
    use crate::day::{Day, Answer};
    
    // Example Inputs
    const EXAMPLE1: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    const EXAMPLE1_SIZE: usize =7;

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1, EXAMPLE1_SIZE);

        assert_eq!(input.size, EXAMPLE1_SIZE);
        assert_eq!(input.coords.len(), 25);
    }

    #[test]
    fn test_steps() {
        let input = Input::read(EXAMPLE1, EXAMPLE1_SIZE);

        let steps = Day18::solve(&input, 12);
        assert_eq!(steps, Some(22));
    }

    #[test]
    fn test_cutoff() {
        let input = Input::read(EXAMPLE1, EXAMPLE1_SIZE);

        let cutoff = Day18::find_cutoff(&input, 12);
        assert_eq!(cutoff, Some((6,1)));
    }

    
    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 2.
        let mut d = Day18::new();
        d.prob_size = 7;
        d.start_t = 12; // override for test
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(22));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let mut d = Day18::new();
        d.prob_size = 7;
        d.start_t = 12; // override for test
        assert_eq!(d.part2(EXAMPLE1), Answer::String("6,1".to_string()));
    }
    
}