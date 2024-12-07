use std::collections::HashSet;

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
    rows: isize,
    cols: isize,
    blocks: HashSet<(isize, isize)>,
    start_pos: (isize, isize),
    start_heading: (isize, isize),
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut blocks: HashSet<(isize, isize)> = HashSet::new();
        let mut start_pos = (0, 0);
        let mut rows = 0;
        let mut cols = 0;

        for (row_no_u, line) in text.lines().enumerate() {
            let row_no = row_no_u as isize;
            for (col_no_u, c) in line.chars().enumerate() {
                let col_no = col_no_u as isize;
                match c {
                    '#' => {
                        // record this block
                        blocks.insert((row_no, col_no));
                    }
                    '^' => {
                        start_pos = (row_no, col_no);
                    }
                    _ => ()
                }
                if col_no >= cols { cols = col_no + 1; }
            }
            if row_no >= rows { rows = row_no + 1; }
        }

        let start_heading = (-1, 0);

        Input { rows, cols, blocks, start_pos, start_heading }
    }

    // do one move from a starting position and heading, returns Some() next position and heading.
    // if moved off the grid, returns None.
    fn step(&self, pos: &(isize, isize), heading: &(isize, isize), extra_blocker: &Option<(isize, isize)>) -> Option<((isize, isize), (isize, isize))> {
        // take a Naive step to next_row, next_col
        let mut next_pos= (pos.0+heading.0, pos.1+heading.1);
        let mut next_heading = *heading;

        // If we went off grid, return None
        if (next_pos.0 < 0) | (next_pos.0 >= self.rows) | (next_pos.1 < 0) | (next_pos.1 >= self.cols) {
            return None;
        }

        // If we are blocked, turn right
        if self.blocks.contains(&next_pos) | (*extra_blocker == Some(next_pos)) {
            // revert to initial position
            next_pos = *pos;

            next_heading = (heading.1, -heading.0);
        }

        Some((next_pos, next_heading))
    }

    // Number of spaces visited by the simple walk.
    fn num_visited(&self) -> usize {
        let mut visited: HashSet::<(isize, isize)> = HashSet::new();

        let mut position = self.start_pos;
        let mut heading = self.start_heading;
        visited.insert(position);

        while let Some((new_pos, new_heading)) = self.step(&position, &heading, &None) {
            position = new_pos;
            heading = new_heading;
            visited.insert(position);
        }

        visited.len()
    }

    // true if it's ok to place a block at position.
    fn valid_block_space(&self, position:&(isize, isize)) -> bool {
        // must be on board
        if (position.0 < 0) | (position.0 >= self.rows) | (position.1 < 0) | (position.1 >= self.cols) {
            return false;
        }

        // can't be on another block
        if self.blocks.contains(position) {
            return false;
        }

        // can't be the starting position
        if self.start_pos == *position {
            return false;
        }

        return true;
    }

    // Test whether a loop is entered if the player at position and heading go forward, assuming an
    // added blocker at <blocker>.
    fn it_loops(
        &self, 
        early_path: &HashSet<((isize, isize), (isize, isize))>, position:&(isize, isize), 
        heading:&(isize, isize), 
        blocker:&(isize, isize)) -> bool {

        // step from position, heading until we either retrace a step or fall off board.
        // return true if we start retracing.

        let mut visited:HashSet<((isize, isize), (isize, isize))> = HashSet::new();
        let mut position = *position;
        let mut heading = *heading;

        while let Some((new_pos, new_heading)) = self.step(&position, &heading, &Some(*blocker)) {
            position = new_pos;
            heading = new_heading;

            if early_path.contains(&(position, heading)) | visited.contains(&(position, heading)) {
                // we are repeating.
                return true;
            }
            visited.insert((position, heading));
        }

        // reached edge
        false
    }

    // Return number of places a single new block could be place, to create a loop.
    fn num_options(&self) -> usize {
        // Here's the plan: Walk the normal path.  At each step hypothesize a block in front (if valid).  
        // Check whether this leads into a loop by continuing on and detecting overlap.
        // Placing a block is valid if the space is on the grid, isn't already a block or start and 
        // has not been visited earlier.

        // Places the path has visited so far
        let mut visited: HashSet::<(isize, isize)> = HashSet::new();
        let mut path: HashSet::<((isize, isize), (isize, isize))> = HashSet::new();

        // Places where a block can go and have been found to loop.
        let mut loopers: HashSet::<(isize, isize)> = HashSet::new();
        let mut already_checked: HashSet::<(isize, isize)> = HashSet::new();

        // Current position and heading
        let mut position = self.start_pos;
        let mut heading = self.start_heading;
        visited.insert(position);
        path.insert((position, heading));
        
        // Test whether we could block in front of the initial position
        let next_spot = (position.0+heading.0, position.1+heading.1);
        if self.valid_block_space(&next_spot) & 
            !visited.contains(&next_spot) &
            !already_checked.contains(&next_spot) {
            // We could put a block on next_spot.
            already_checked.insert(next_spot);
            if self.it_loops(&path, &position, &heading, &next_spot) {
                // Add this to the set of looping block positions
                loopers.insert(next_spot);
            }
        }

        while let Some((new_pos, new_heading)) = self.step(&position, &heading, &None) {
            position = new_pos;
            heading = new_heading;
            visited.insert(position);
            path.insert((position, heading));

            // Test whether we could block in front of this position. 
            // (This duplicates logic above.  This should be refactored.)
            let next_spot = (position.0+heading.0, position.1+heading.1);
            if self.valid_block_space(&next_spot) & 
                !visited.contains(&next_spot) &
                !already_checked.contains(&next_spot) {
                // We could put a block on next_spot.
                already_checked.insert(next_spot);
                if self.it_loops(&path, &position, &heading, &next_spot) {
                    // Add this to the set of looping block positions
                    loopers.insert(next_spot);
                }
            }
        }

        loopers.len()
    }
}

pub struct Day6 {
}

// Day6
impl Day6 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day6 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.num_visited())
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.num_options())
    }
}

#[cfg(test)]

mod test {

    use crate::day6::{Day6, Input};
    use crate::day::{Day, Answer};
    
    // TODO Place example inputs here.
    const EXAMPLE1: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.rows, 10);
        assert_eq!(input.cols, 10);
        assert_eq!(input.blocks.len(), 8);
        assert_eq!(input.start_pos, (6, 4));
        assert_eq!(input.start_heading, (-1, 0));
    }

    #[test]
    fn test_step() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.step(&(6, 4), &(-1, 0), &None), Some( ((5, 4), (-1, 0)) ) ); // step north
        assert_eq!(input.step(&(2, 4), &(-1, 0), &None), Some( ((1, 4), (-1, 0)) ) ); // step up to block
        assert_eq!(input.step(&(1, 4), &(-1, 0), &None), Some( ((1, 4), (0, 1)) ) );  // hit block, turn east
        assert_eq!(input.step(&(1, 4), &(0, 1), &None), Some( ((1, 5), (0, 1)) ) );   // move east from block
        assert_eq!(input.step(&(1, 8), &(0, 1), &None), Some( ((1, 8), (1, 0)) ) );   // hit block, turn south
        assert_eq!(input.step(&(6, 8), &(1, 0), &None), Some( ((6, 8), (0, -1)) ) );  // hit block, turn west
        assert_eq!(input.step(&(6, 2), &(0, -1), &None), Some( ((6, 2), (-1, 0)) ) ); // hit block, turn north
        assert_eq!(input.step(&(9, 7), &(1, 0), &None), None );                       // walk off the edge
    }

    #[test]
    fn test_visited() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.num_visited(), 41);
    }

    #[test]
    fn test_num_options() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.num_options(), 6); 
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day6::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(41));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day6::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(6));
    }
    
}