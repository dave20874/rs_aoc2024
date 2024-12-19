use std::collections::{HashMap, HashSet};

use crate::day::{Day, Answer};
use priority_queue::PriorityQueue;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct SolnState {
    pos: (usize, usize),
    dir: Dir,
}


impl SolnState {
    fn new(pos: (usize, usize)) -> SolnState {
        SolnState {pos, dir: Dir::E }
    }

    fn forward(&self) -> SolnState {
        let newpos = match self.dir {
            Dir::N => { (self.pos.0, self.pos.1-1) },
            Dir::S => { (self.pos.0, self.pos.1+1) },
            Dir::E => { (self.pos.0+1, self.pos.1) },
            Dir::W => { (self.pos.0-1, self.pos.1) },
        };

        SolnState {pos: newpos, dir: self.dir}
    }

    fn right(&self) -> SolnState {
        let new_dir = match self.dir {
            Dir::N => { Dir::E },
            Dir::S => { Dir::W },
            Dir::E => { Dir::S },
            Dir::W => { Dir::N },
        };

        SolnState {pos: self.pos, dir: new_dir}
    }

    fn left(&self) -> SolnState {
        let new_dir = match self.dir {
            Dir::N => { Dir::W },
            Dir::S => { Dir::E },
            Dir::E => { Dir::N },
            Dir::W => { Dir::S },
        };

        SolnState {pos: self.pos, dir: new_dir}
    }
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    maze: HashSet<(usize, usize)>, // A coord in maze represents open space.
    start: (usize, usize),         // Start location
    end: (usize, usize),           // End location
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut maze = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in text.lines().enumerate() {
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        // Wall, we don't need to do anything
                    }
                    '.' => {
                        // Open space, add this to the maze HashSet
                        maze.insert((x, y));
                    }
                    'S' => {
                        // Start, add this to the maze HashSet and record start
                        maze.insert((x, y));
                        start = (x, y);
                    }
                    'E' => {
                        // End, add to maze and record end
                        maze.insert((x, y));
                        end = (x, y);
                    }
                    _ => {
                        // Invalid input
                        panic!("Invalid input character.");
                    }
                }
            }
        }

        Input { maze, start, end }
    }

    fn solve(&self) -> Option<(usize, usize)> {
        let mut to_explore: PriorityQueue<(SolnState, Vec<(usize, usize)>), isize> = PriorityQueue::new();
        let mut explored: HashMap<SolnState, usize> = HashMap::new();
        let start_state = SolnState::new(self.start);
        to_explore.push((start_state, Vec::new()), 0);
        let mut best_cost = None;
        let mut best_tiles: HashSet<(usize, usize)> = HashSet::new();

        // start and end positions are always in best tiles.
        best_tiles.insert(self.start);
        best_tiles.insert(self.end);
        // let mut iterations = 0;

        while !to_explore.is_empty() {
            // iterations += 1;
            // if iterations > 1000 {
            //     return None
            // }

            // Pop the best current state from the queue
            let ((current, path), neg_cost) = to_explore.pop().unwrap();
            let cost = -neg_cost as usize;

            if let Some(old_cost) = explored.get(&current) {
                if *old_cost < cost {
                    // We already explored this state at a lower or equal cost.
                    continue;
                }
            }

            explored.insert(current, cost);

            // If this is at the end, we have solved it.
            if current.pos == self.end {
                match best_cost {
                    Some(prior_cost) => {
                        if cost < prior_cost {
                            panic!("This can't be happening")
                        }
                        else if cost > prior_cost {
                            // We won't find any more best solutions
                            break;
                        }
                        else {
                            // An equally good solution was found.
                            // register all locations on its path.
                            for pos in &path {
                                best_tiles.insert(*pos);
                            }
                        }
                    }
                    None => {
                        // We have our best, lowest cost
                        best_cost = Some(cost);

                        // register all locations on its path.
                        for pos in &path {
                            best_tiles.insert(*pos);
                        }
                    }
                }
            }

            // Generate all possible next steps from here and their costs
            for (next_state, next_cost) in Self::next_states(&current, &cost) {
                if self.maze.contains(&next_state.pos) {
                    // The space is clear
                    // next_state represents a valid move
                    // extend path
                    let mut extended_path = path.clone();
                    extended_path.push(next_state.pos);
                    to_explore.push((next_state, extended_path), -(next_cost as isize));
                }
            }
        }

        match best_cost {
            Some(cost) => {
                // We have a solution, return cost and num best_tiles.
                Some( (cost, best_tiles.len()) )
            }
            None => { None }
        }
    }

    // Generate next states.
    fn next_states(state: &SolnState, cost: &usize) -> Vec<(SolnState, usize)> {
        vec![(state.forward(), cost+1), 
             (state.left(), cost+1000), 
             (state.right(), cost+1000)]
    }
}

pub struct Day16 {
}

// Day16
impl Day16 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day16 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        match input.solve() {
            Some((cost, _best_tiles)) => Answer::Numeric(cost),
            None => Answer::None,
        }
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);


        match input.solve() {
            Some((_cost, best_tiles)) => Answer::Numeric(best_tiles),
            None => Answer::None,
        }
    }
}

#[cfg(test)]

mod test {

    use crate::day16::{Day16, Input};
    use crate::day::{Day, Answer};
    
    // Example Inputs
    const EXAMPLE1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

const EXAMPLE2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert!(input.maze.contains(&(1, 1)));
        assert!(!input.maze.contains(&(0, 0)));
        assert_eq!(input.start, (1, 13));
        assert_eq!(input.end, (13, 1));
    }

    #[test]
    // Read and confirm inputs
    fn test_read_ex2() {
        let input = Input::read(EXAMPLE2);

        assert!(input.maze.contains(&(1, 1)));
        assert!(!input.maze.contains(&(0, 0)));
        assert_eq!(input.start, (1, 15));
        assert_eq!(input.end, (15, 1));
    }

    #[test]
    // Read and confirm inputs
    fn test_solve() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.solve(), Some((7036, 45)));
    }

    #[test]
    // Read and confirm inputs
    fn test_solve2() {
        let input = Input::read(EXAMPLE2);

        assert_eq!(input.solve(), Some((11048, 64)));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day16::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(7036));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day16::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(45));
    }
    
}