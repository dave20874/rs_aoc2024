use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref LINE_RE: Regex = Regex::new("(\\d+)\\s+(\\d+)").unwrap();
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    walls: HashSet<(usize, usize)>,
    open: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut walls = HashSet::new();
        let mut open = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in text.lines().enumerate() {
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        // Wall
                        walls.insert((x, y));
                    }
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
                    _ => { panic!("Bad input.") }
                }
                width = x+1;
            }
            height = y+1;
        }

        Input { walls, open, width, height, start, end }
    }
}

struct Cheat {
    _from: (usize, usize),
    _to: (usize, usize),
    new_dist: usize,
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

    // Test whether moving through wall at coord works as a cheat
    // To work, there has to be a way from an open cell on one side
    // to the other.  If the wall geometry doesn't work, we return 
    // None.  If it works, we return Some(cheat).

    // If there are two possible cheats for a block, (because it stands alone),
    // Only the N-S cheat is evaluated and returned.
    fn eval_cheat(input: &Input, 
                  from_start: &HashMap<(usize, usize), usize>,
                  from_end: &HashMap<(usize, usize), usize>,
                  coord: &(usize, usize),
                  orig_dist: usize) -> Option<Cheat> {
        // Determine start and end points of the cheat in open area.
        let n = (coord.0, coord.1-1);
        let s = (coord.0, coord.1+1);
        let w = (coord.0-1, coord.1);
        let e = (coord.0+1, coord.1);

        let (start, end) = 
            if input.open.contains(&n) & input.open.contains(&s) {
                // A N-S cheat is possible, which direction would it go?
                if from_end.get(&n) < from_end.get(&s) {
                    // South to North
                    (s, n)
                }
                else {
                    // North to South
                    (n, s)
                }
            }
            else if input.open.contains(&w) & input.open.contains(&e) {
                // An E-W cheat is possible, which direction would it go?
                if from_end.get(&w) < from_end.get(&e) {
                    // East to West
                    (e, w)
                }
                else {
                    // West to East
                    (w, e)
                }
            }
            else {
                // Neither direction is open so no cheat here
                return None
            };

        // Evaluate total distance with the cheat
        let new_dist = from_start.get(&start).unwrap() + from_end.get(&end).unwrap() + 2;

        // Return this cheat
        Some(Cheat { _from: start, _to: end, new_dist, savings: orig_dist as isize - new_dist as isize })
    }

    fn find_cheats(input: &Input) -> Vec<Cheat> {

        // Evaluate distances from start end end for every open space
        let from_start = Day20::dist_from(input, &input.start);
        let from_end = Day20::dist_from(input, &input.end);

        let orig_dist = from_start.get(&input.end).unwrap();

        // For every wall point, not on the border, evaluate whether
        // it can be used as a cheat
        let cheats = input.walls.iter()
            // Eliminate edges from consideration
            .filter(|coord| {   
                (coord.0 > 0) & (coord.1 > 0) & (coord.0 < input.width-1) & (coord.1 < input.height-1) 
            } )

            // Evaluate this wall for cheating
            .filter_map(|coord| {
                Day20::eval_cheat(input, &from_start, &from_end, coord, *orig_dist)
            })

            // Take only the cheats that shorten the path
            .filter(|cheat| {
                cheat.new_dist < *orig_dist
            })

            // Collect into a vector
            .collect();

        cheats
    }

    fn num_valid_cheats(input: &Input, threshold: isize) -> usize {
        let cheats = Day20::find_cheats(&input);

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

        let n = Day20::num_valid_cheats(&input, 100);
        Answer::Numeric(n)
    }

    fn part2(&self, text: &str) -> Answer {
        let _input = Input::read(text);

        Answer::None
    }
}

#[cfg(test)]

mod test {

    use crate::day20::{Day20, Input};
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
        assert_eq!(input.walls.len(), 140);
        assert_eq!(input.open.len(), 85);
        assert_eq!(input.width, 15);
        assert_eq!(input.height, 15);
        assert_eq!(input.start, (1, 3));
        assert_eq!(input.end, (5, 7));
    }

    #[test]
    fn test_cheats() {
        let input = Input::read(EXAMPLE1);
        let cheats = Day20::find_cheats(&input);

        assert_eq!(cheats.len(), 14+14+2+4+2+3+1+1+1+1+1);
    }

    #[test]
    fn test_num_valid_cheats() {
        let input = Input::read(EXAMPLE1);
        let n = Day20::num_valid_cheats(&input, 20);

        assert_eq!(n, 5);
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
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}