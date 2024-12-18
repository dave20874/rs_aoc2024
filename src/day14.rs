use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref LINE_RE: Regex = Regex::new("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)").unwrap();
}

struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn project(&self, field: (isize, isize), time: isize) -> (isize, isize) {
        let px = (self.pos.0 + self.vel.0*time).rem_euclid(field.0);
        let py = (self.pos.1 + self.vel.1*time).rem_euclid(field.1);

        (px, py)
    }
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    robots: Vec<Robot>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let robots = text.lines()
            .filter_map(|s| {
                if let Some(caps) = LINE_RE.captures(s) {
                    // println!("Captured: {}, {}, {}, {}", &caps[1], &caps[2], &caps[3], &caps[4]);
                    let pos = ( caps[1].parse().unwrap(), caps[2].parse().unwrap() );
                    let vel = ( caps[3].parse().unwrap(), caps[4].parse().unwrap() );
                    Some( Robot { pos, vel })
                }
                else {
                    None
                }
            })
            .collect();

        Input { robots }
    }

    fn safety_factor(&self, field: (isize, isize), time: isize) -> usize {
        let mid = (field.0/2, field.1/2);
        let components: (isize, isize, isize, isize) = self.robots.iter()
            .map(|r| { 
                let dest = r.project(field, time);
                if (dest.0 < mid.0) & (dest.1 < mid.1) {
                    // Upper left quadrant
                    (1, 0, 0, 0)
                }
                else if (dest.0 < mid.0) & (dest.1 > mid.1) {
                    // Lower left quadrant
                    (0, 1, 0, 0)
                }
                else if (dest.0 > mid.0) & (dest.1 < mid.1) {
                    // Upper right quadrant
                    (0, 0, 1, 0)
                }
                else if (dest.0 > mid.0) & (dest.1 > mid.1) {
                    // Lower right quadrant
                    (0, 0, 0, 1)
                }
                else {
                    // On center lines
                    (0, 0, 0, 0)
                }
             })
             .fold((0, 0, 0, 0), |accum, new| {
                (accum.0+new.0, accum.1+new.1, accum.2+new.2, accum.3+new.3)
             });

        let factor = components.0 * components.1 * components.2 * components.3;

        factor as usize
    }

    fn tree_search(&self, field: (isize, isize), time: isize) -> usize {
        let mut found = false;
        let mut steps = 0;

        while !found & (steps <= time) {
            steps += 1;
            // println!("Checking {} steps", steps); 

            found = self.is_tree(field, steps);
        }

        steps as usize
    }

    // See if a tree appears at this time, on this size field.
    // We detect a tree by finding 30 occupied horizontal cells.
    // The tree pattern has this, no other state does.
    fn is_tree(&self, field: (isize, isize), time: isize) -> bool {
        // Look for a horizontal line segment 30 elements long
        let mut map: HashSet<(isize, isize)> = HashSet::new();

        for r in self.robots.iter() {
            let dest = r.project(field, time);
            map.insert(dest);
        }

        for line_no in 0..field.1 {
            'line: for col_no in 0..field.0-30 {
                for n in 0..30 {
                    if !map.contains(&(col_no+n, line_no)) {
                        continue 'line;
                    }
                }

                // Found 30 occupied cells in a row.
                return true;
            }
        }

        // Never did find that tree
        false

    }

    // fn show_tree(&self, field: (isize, isize), time: isize) {
    //     let mut map: HashSet<(isize, isize)> = HashSet::new();

    //     for r in self.robots.iter() {
    //         let dest = r.project(field, time);
    //         map.insert(dest);
    //     }

    //     for line_no in 0..field.1 {
    //         for col_no in 0..field.0 {
    //             if map.contains(&(col_no, line_no)) {
    //                 print!("#");
    //             }
    //             else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }
}

pub struct Day14 {
}

// Day14
impl Day14 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day14 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.safety_factor((101, 103), 100))
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.tree_search((101, 103), 101*103))
    }
}

#[cfg(test)]

mod test {

    use crate::day14::{Day14, Input};
    use crate::day::{Day, Answer};
    use data_aoc2024::DAY14_INPUT;
    
    // Example Inputs
    const EXAMPLE1: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.robots.len(), 12);
        assert_eq!(input.robots[0].pos, (0, 4));
        assert_eq!(input.robots[0].vel, (3, -3));
    }

    #[test]
    // Read and confirm inputs
    fn test_projection() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.robots[10].project((11, 7), 1), (4, 1));
        assert_eq!(input.robots[10].project((11, 7), 2), (6, 5));
        assert_eq!(input.robots[10].project((11, 7), 3), (8, 2));
        assert_eq!(input.robots[10].project((11, 7), 4), (10, 6));
        assert_eq!(input.robots[10].project((11, 7), 5), (1, 3));
    }

    
    #[test]
    // Read and confirm inputs
    fn test_safety_factor() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.safety_factor((11, 7), 100), 12);
    }

    #[test]
    fn test_tree_time() {
        let input = Input::read(DAY14_INPUT);

        assert_eq!(input.tree_search((101, 103), 10403), 6620);

        // input.show_tree((101, 103), 2697);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day14::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(21));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day14::new();
        assert_eq!(d.part2(data_aoc2024::DAY14_INPUT), Answer::Numeric(6620)); // < 10402
    }
    
}