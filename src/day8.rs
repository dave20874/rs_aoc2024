use std::collections::{HashMap, HashSet};
use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    width: isize,
    height: isize,
    antennas: HashMap<char, Vec<(isize, isize)>>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut width  = 0;
        let mut height = 0;
        let mut antennas = HashMap::new();

        for (row_no, line) in text.lines().enumerate() {
            for (col_no, c) in line.chars().enumerate() {
                if col_no >= width { width = col_no+1; }
                if row_no >= height { height = row_no+1; }
                if c.is_alphanumeric() {
                    // We found an antenna
                    // Get reference, in antennas, to this antenna's location.
                    // (create a new antenna if this is the first.)
                    let v = match antennas.get_mut(&c) {
                        Some(vect) => vect,
                        None => {
                            antennas.insert(c, Vec::new());
                            antennas.get_mut(&c).unwrap()
                        }
                    };

                    // Add this antenna's location
                    v.push( (col_no as isize, row_no as isize) );
                }
            }
        }

        Input { width: width as isize, height: height as isize, antennas }
    }

    fn count_antinodes(&self) -> usize {
        let mut antinode_map: HashSet<(isize, isize)> = HashSet::new();
        for antenna_type in self.antennas.keys() {
            let antennas = &self.antennas[&antenna_type];
            if antennas.len() > 1 {
                // Nested for loops over every pair of antennas, i, j
                for i in 0..antennas.len()-1 {
                    for j in i+1..antennas.len() {
                        // Compute vector from i to j, compute locations of antinodes
                        let (dx, dy) = (antennas[i].0-antennas[j].0, antennas[i].1-antennas[j].1);
                        let antinode1 = (antennas[i].0+dx, antennas[i].1+dy);
                        let antinode2 = (antennas[j].0-dx, antennas[j].1-dy);

                        // Insert antinodes (if they are on the map)
                        if (antinode1.0 >= 0) & (antinode1.0 < self.width) 
                           & (antinode1.1 >= 0) & (antinode1.1 < self.height) {
                            // valid antinode 1
                            antinode_map.insert(antinode1);
                        }
                        
                        if (antinode2.0 >= 0) & (antinode2.0 < self.width) 
                           & (antinode2.1 >= 0) & (antinode2.1 < self.height) {
                            // valid antinode 2
                            antinode_map.insert(antinode2);
                        }
                    }
                }
            }
        }

        antinode_map.len()
    }

    fn count_antinodes_updated(&self) -> usize {
        let mut antinode_map: HashSet<(isize, isize)> = HashSet::new();
        for antenna_type in self.antennas.keys() {
            let antennas = &self.antennas[&antenna_type];
            if antennas.len() > 1 {
                // Nested for loops over every pair of antennas, i, j
                for i in 0..antennas.len()-1 {
                    for j in i+1..antennas.len() {

                        // Get vector between these antennas
                        let (dx, dy) = (antennas[i].0-antennas[j].0, antennas[i].1-antennas[j].1);

                        // Project Antinodes from antenna[i], including the antenna
                        let mut p = (antennas[i].0, antennas[i].1);
                        while (p.0 >= 0) & (p.0 < self.width) & (p.1 >= 0) & (p.1 < self.height) {
                            antinode_map.insert(p);
                            p = (p.0+dx, p.1+dy);
                        }

                        // Project Antinodes from antenna[j], including the antenna
                        let mut p = (antennas[j].0, antennas[j].1);
                        while (p.0 >= 0) & (p.0 < self.width) & (p.1 >= 0) & (p.1 < self.height) {
                            antinode_map.insert(p);
                            p = (p.0-dx, p.1-dy);
                        }
                    }
                }
            }
        }

        antinode_map.len()
    }
}

pub struct Day8 {
}

// Day8
impl Day8 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day8 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.count_antinodes())
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.count_antinodes_updated())
    }
}

#[cfg(test)]

mod test {

    use crate::day8::{Day8, Input};
    use crate::day::{Day, Answer};
    
    // TODO Place example inputs here.
    const EXAMPLE1: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.width, 12);
        assert_eq!(input.height, 12);
        assert_eq!(input.antennas.len(), 2);
        assert_eq!(input.antennas[&'0'].len(), 4);
        assert_eq!(input.antennas[&'0'][0], (8, 1));
        assert_eq!(input.antennas[&'A'].len(), 3);
    }

    #[test]
    // Read and confirm inputs
    fn test_num_antinodes() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.count_antinodes(), 14);
    }

    
    #[test]
    // Read and confirm inputs
    fn test_num_antinodes_updated() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.count_antinodes_updated(), 34);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day8::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(14));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day8::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(34));
    }
    
}