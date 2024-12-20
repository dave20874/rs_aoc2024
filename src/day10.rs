use std::collections::{HashMap, HashSet};

use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    height: isize,
    width: isize,
    alt: Vec<Vec<isize>>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut height = 0;
        let mut width = 0;
        let mut alt = Vec::new();

        for (_x, line) in text.lines().enumerate() {
            let mut row = Vec::new();
            for (y, c) in line.trim().chars().enumerate() {
                row.push(c.to_digit(10).unwrap() as isize);
                if y as isize >= width {
                    width = y as isize +1;
                }
            }
            if row.len() > 0 {
                alt.push(row);
                height += 1;
            }
        }

        Input { height, width, alt }
    }

    fn sum_scores(&self) -> usize {

        // reachable_peaks is a map of coord (y, x) to a set of coords (y, x), (y, x)...
        // that are reachable peaks from the first coord.
        // We'll start with an empty map, then add all 9's as reaching the set of just
        // themselves.
        // Then all the 8's will be assigned the union of reachable peaks of their 
        // four adjacent cells.
        // By the time we do 0, those cells should get sets of all the peaks they can
        // reach.
        let mut reachable_peaks: HashMap<(isize, isize), HashSet<(isize, isize)>>
            = HashMap::new();
    
        for altitude in (0..10).rev() {
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.alt[y as usize][x as usize] == altitude {
                        if altitude == 9 {
                            // This case is special.  A peak is reachable by itself.
                            let mut set = HashSet::new();
                            set.insert( (y, x) );
                            reachable_peaks.insert((y, x), set);
                        }
                        else {
                            let mut set = HashSet::new();
    
                            // Create union of surrounding, reachable peaks reachable from here
                            for (neighbor_y, neighbor_x) in vec![(y+1, x), (y, x+1), (y-1, x), (y, x-1)] {
                                if (neighbor_x >= 0) & (neighbor_x < self.width) & (neighbor_y >= 0) & (neighbor_y < self.height) {
                                    // dbg!((neighbor_y, neighbor_x));
                                    if  self.alt[neighbor_y as usize][neighbor_x as usize] == altitude+1 {
                                        // this neighbor would be part of a trail
                                        let reachable = reachable_peaks.get(&(neighbor_y, neighbor_x)).unwrap();
                                        for peak in reachable.iter() {
                                            set.insert(*peak);
                                        }
                                    }
                                }
                            }
                            reachable_peaks.insert((y, x), set);
                        }
                    }
                }
            }
        }
    
        let mut sum = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.alt[y as usize ][x as usize] == 0 {
                    sum += reachable_peaks.get(&(y, x)).unwrap().len()
                }
            }
        }
    
        sum
        
    }

    fn sum_ratings(&self) -> usize {
        let mut ratings: HashMap<(isize, isize), usize>
            = HashMap::new();
       
        for altitude in (0..10).rev() {
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.alt[y as usize][x as usize] == altitude {
                        if altitude == 9 {
                            // This case is special.  A peak's rating is always 1.
                            ratings.insert((y, x), 1);
                        }
                        else {
                            // Count distinct paths leading up from here.
                            let mut count = 0;
                            for (neighbor_y, neighbor_x) in vec![(y+1, x), (y, x+1), (y-1, x), (y, x-1)] {
                                if (neighbor_x >= 0) & (neighbor_x < self.width) & (neighbor_y >= 0) & (neighbor_y < self.height) {
                                    // dbg!((neighbor_y, neighbor_x));
                                    if  self.alt[neighbor_y as usize][neighbor_x as usize] == altitude+1 {
                                        // this neighbor would be part of a trail
                                        count += ratings.get(&(neighbor_y, neighbor_x)).unwrap();
                                    }
                                }
                            }
                            ratings.insert((y, x), count);
                        }
                    }
                }
            }
        }
    
        let mut sum = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.alt[y as usize ][x as usize] == 0 {
                    sum += ratings.get(&(y, x)).unwrap();
                }
            }
        }
    
        sum
        
    }
}

pub struct Day10 {
}

// Day10
impl Day10 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day10 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.sum_scores())
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.sum_ratings())
    }
}

#[cfg(test)]

mod test {

    use crate::day10::{Day10, Input};
    use crate::day::{Day, Answer};
    
    // Example inputs
    const EXAMPLE1: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.height, 8);
        assert_eq!(input.width, 8);
        assert_eq!(input.alt[1][1], 8);
    }

    #[test]
    // Read and confirm inputs
    fn test_sum_scores() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.sum_scores(), 36);
    }

    #[test]
    fn test_sum_ratings() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.sum_ratings(), 81);        
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day10::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(36));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day10::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(81));
    }
    
}