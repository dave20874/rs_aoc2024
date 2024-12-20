use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref LINE_RE: Regex = Regex::new("(\\d+)\\s+(\\d+)").unwrap();
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    open: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut open = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in text.lines().enumerate() {
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        // Open
                        open.insert((x, y));
                    }
                    'S' => {
                        // Start
                        start = (x, y);
                        open.insert((x, y));
                    }
                    'E' => {
                        // End
                        end = (x, y);
                        open.insert((x, y));
                    }
                    _ => () // Ignore walls and anything extraneous.
                }
            }
        }

        Input { open, start, end }
    }
}

#[derive(Debug)]
struct Cheat {
    // from: (usize, usize),
    // to: (usize, usize),
    // new_dist: usize,
    savings: isize,
}

pub struct Day20 {
}

// Day20
impl Day20 {
    pub const fn new() -> Self {
        Self { }
    }

    // Evaluate the distance from coord to every open spot in the map.
    fn dist_from(input: &Input, coord: &(usize, usize)) -> HashMap<(usize, usize), usize> {
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

        let mut to_check: VecDeque<((usize, usize), usize)> = VecDeque::new();
        to_check.push_back((*coord, 0));

        while !to_check.is_empty() {
            let ((x, y), dist) = to_check.pop_front().unwrap();

            // If there's already a better way to get to (x, y) ignore this
            if let Some(other_dist) = distances.get(&(x, y)) {
                if *other_dist <= dist { continue; }
            }

            // record this distance
            distances.insert((x, y), dist);

            // Generate neighbors of (x, y).  If they are open
            let n = (x, y-1);
            let s = (x, y+1);
            let e = (x+1, y);
            let w = (x-1, y);
            for neighbor in [n, s, e, w] {
                if input.open.contains(&neighbor) {
                    to_check.push_back((neighbor, dist+1));
                }
            }
        }

        distances
    }

    // Test whether a cheat from start_coord to end_coord saves time.
    // Return None if not., Some(cheat) if so.
    fn eval_cheat(from_start: &HashMap<(usize, usize), usize>,
                  from_end: &HashMap<(usize, usize), usize>,
                  start_coord: &(usize, usize),
                  end_coord: &(usize, usize),
                  orig_dist: usize) -> Option<Cheat> {

        // // Evaluate total distance with the cheat
        let cheat_dist = 
            ((end_coord.0 as isize - start_coord.0 as isize).abs() + 
            (end_coord.1 as isize - start_coord.1 as isize).abs()) as usize;
        let new_dist = from_start.get(&start_coord).unwrap() + from_end.get(&end_coord).unwrap() + cheat_dist;
        let savings = orig_dist as isize - new_dist as isize;

        // Return this cheat
        if savings > 0 {
            Some(Cheat { /* from: *start_coord, to: *end_coord, new_dist, */ savings})
        }
        else {
            None
        }
    }

    fn find_cheats(input: &Input, allowed_dist: usize) -> Vec<Cheat> {

        // Evaluate distances from start end end for every open space
        let from_start = Day20::dist_from(input, &input.start);
        let from_end = Day20::dist_from(input, &input.end);

        let orig_dist = from_start.get(&input.end).unwrap();

        // Check every pair of open cells as a potential cheat
        let start_iter = input.open.iter();
        let end_iter = input.open.iter();
        let cheats = start_iter.cartesian_product(end_iter)

            // Make sure start to end distance is allowed
            .filter(|(start, end)| {
                let dist = (start.0 as isize - end.0 as isize).abs() + (start.1 as isize - end.1 as isize).abs();
                (dist as usize <= allowed_dist) & (dist as usize >= 2)
            })

            // Evaluate the cheat's savings and construct Cheat
            .filter_map(|(start, end)| {
                Day20::eval_cheat(&from_start, &from_end, start, end, *orig_dist)
            })
            .collect();

        cheats
    }

    fn num_valid_cheats(input: &Input, threshold: isize, allowed_dist: usize) -> usize {
        let cheats = Day20::find_cheats(&input, allowed_dist);

        cheats.iter()
            // Take only the ones with >= 100 picoseconds of savings
            .filter(|cheat| { cheat.savings >= threshold })
            .count()
    }

}

impl<'a> Day for Day20 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let n = Day20::num_valid_cheats(&input, 100, 2);
        Answer::Numeric(n)
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let n = Day20::num_valid_cheats(&input, 100, 20);
        Answer::Numeric(n)
    }
}

#[cfg(test)]

mod test {

    use crate::day20::{Day20, Input, Cheat};
    use crate::day::{Day, Answer};
    
    // TODO Place example inputs here.
    const EXAMPLE1: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);
        assert_eq!(input.open.len(), 85);
        assert_eq!(input.start, (1, 3));
        assert_eq!(input.end, (5, 7));
    }

    #[test]
    fn test_cheats() {
        let input = Input::read(EXAMPLE1);
        let cheats = Day20::find_cheats(&input, 2);

        assert_eq!(cheats.len(), 14+14+2+4+2+3+1+1+1+1+1);
    }

    #[test]
    fn test_num_valid_cheats() {
        let input = Input::read(EXAMPLE1);
        let n = Day20::num_valid_cheats(&input, 20, 2);

        assert_eq!(n, 5);
    }

    #[test]
    fn test_cheats_p2() {
        let input = Input::read(EXAMPLE1);
        let all_cheats = Day20::find_cheats(&input, 20);
        let best_cheats: Vec<&Cheat> = all_cheats.iter()
            .filter(|c| c.savings >= 50)
            .collect();

        assert_eq!(best_cheats.len(), 32+31+29+39+25+23+20+19+12+14+12+22+4+3);
    }

    #[test]
    fn test_num_valid_cheats_p2() {
        let input = Input::read(EXAMPLE1);
        let n = Day20::num_valid_cheats(&input, 74, 20);

        assert_eq!(n, 7);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day20::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(0));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day20::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(0));
    }
    
}