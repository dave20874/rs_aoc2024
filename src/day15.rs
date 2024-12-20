use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use crate::day::{Day, Answer};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,E,S,W,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum MapState {
    Empty,
    Box,
    BoxRight,
    BoxLeft,
    Wall,
    Player,
}

struct Board {
    board: Vec<Vec<MapState>>,
    pos: (isize, isize),
}

impl Board {
    fn new(input: &Input, part2: bool) -> Board {
        let mut board = Vec::new();

        for y in 0..input.size.1 {
            let mut row = Vec::new();
            for x in 0..input.size.0 {
                if part2 {
                    // Each input cell, becomes two board cells.
                    match input.map[&(x, y)] {
                        MapState::Box => {
                            row.push(MapState::BoxLeft);
                            row.push(MapState::BoxRight);
                        }
                        MapState::Wall => {
                            row.push(MapState::Wall);
                            row.push(MapState::Wall);
                        }
                        MapState::Empty => {
                            row.push(MapState::Empty);
                            row.push(MapState::Empty);
                        }
                        MapState::Player => {
                            row.push(MapState::Player);
                            row.push(MapState::Empty);
                        }
                        _ => {
                            // Other map states don't occur in input.
                            assert!(false, "Bad map state in input.");
                        }
                    }
                }
                else {
                    // Simply copy map state from the input map to the board
                    row.push(input.map[&(x, y)]);
                }
                
            }
            board.push(row);
        }

        let pos = if part2 {
            // part 2: adjust start position for double-wide processing.
            (input.start_pos.0*2, input.start_pos.1)
        }
        else {
            // part 1: use given start position
            input.start_pos
        };

        Board { board, pos }
    }

    // return coord of adjacent position in the given direction
    fn adjacent_to(a: &(isize, isize), dir: &Dir) -> (isize, isize) {
        match dir {
            Dir::N => { (a.0, a.1-1) }
            Dir::S => { (a.0, a.1+1) }
            Dir::E => { (a.0+1, a.1) }
            Dir::W => { (a.0-1, a.1) }
        }
    }

    fn compare_move(a: &(isize, isize), b: &(isize, isize), dir: &Dir) -> Ordering {
        match dir {
            Dir::N => {
                // When moving North, low-y pieces sort first.
                if a.1 < b.1 { return Ordering::Less; }
                if a.1 > b.1 { return Ordering::Greater; }

                // Same row, sort left to right
                if a.0 < b.0 { return Ordering::Less; }
                if a.0 > b.0 { return Ordering::Greater; }
                return Ordering::Equal;
            }
            Dir::S => {
                // When moving South, high-y pieces sort first.
                if a.1 < b.1 { return Ordering::Greater; }
                if a.1 > b.1 { return Ordering::Less; }

                // Same row, sort left to right
                if a.0 < b.0 { return Ordering::Less; }
                if a.0 > b.0 { return Ordering::Greater; }
                return Ordering::Equal;
            }
            Dir::E => {
                // When moving East, high-x pieces sort first.
                if a.0 < b.0 { return Ordering::Greater; }
                if a.0 > b.0 { return Ordering::Less; }

                // Same col, sort top to bottom
                if a.1 < b.1 { return Ordering::Less; }
                if a.1 > b.1 { return Ordering::Greater; }
                return Ordering::Equal;
            }
            Dir::W => {
                // When moving West, low-x pieces sort first
                if a.0 < b.0 { return Ordering::Less; }
                if a.0 > b.0 { return Ordering::Greater; }

                // Same col, sort top to bottom
                if a.1 < b.1 { return Ordering::Less; }
                if a.1 > b.1 { return Ordering::Greater; }
                return Ordering::Equal;
            }
        }

    }

    fn do_move(&mut self, dir: &Dir) {
        let mut to_move: Vec<(isize, isize)> = Vec::new();
        let mut moving: HashSet<(isize, isize)> = HashSet::new();

        // Start evaluating move from the player's position
        to_move.push(self.pos);

        // Evaluate all the implied moves.  If any required move is
        // blocked, this loop will exit the whole do_move function
        while !to_move.is_empty() {
            let checking = to_move.pop().unwrap();

            // If the thing to check has already been added to moving,
            // we can skip this
            if moving.contains(&checking) { continue; }
            moving.insert(checking);

            let moved_into = Board::adjacent_to(&checking, dir);
            match self.board[moved_into.1 as usize][moved_into.0 as usize] {
                MapState::Wall => {
                    // Blocked!  Give up.
                    return;
                }
                MapState::Box => {
                    // This move can happen if the other box can move.
                    to_move.push(moved_into);
                }
                MapState::BoxLeft => {
                    if (*dir == Dir::N) | (*dir == Dir::S) {
                        // This pushes on both halves of the box.
                        to_move.push(moved_into);
                        to_move.push(Board::adjacent_to(&moved_into, &Dir::E));
                    }
                    else {
                        // Moving E/W
                        to_move.push(moved_into);
                    }
                }
                MapState::BoxRight => {
                    if (*dir == Dir::N) | (*dir == Dir::S) {
                        // This push needs to apply to both halves of the box.
                        to_move.push(moved_into);
                        to_move.push(Board::adjacent_to(&moved_into, &Dir::W));
                    }
                    else {
                        // Moving E/W
                        to_move.push(moved_into);
                    }
                }
                _ => {
                    // Move is to open space, it's fine.
                }  
            }
        }

        // All necessary moves are possible and stored in moving
        let mut move_steps: Vec<&(isize, isize)> = moving.iter().collect();
        move_steps.sort_by(|a, b| { Board::compare_move(*a, *b, dir) });

        // Execute the moves
        for m in move_steps {
            let move_to = Board::adjacent_to(m, dir);
            assert_eq!(self.board[move_to.1 as usize][move_to.0 as usize], MapState::Empty);
            self.board[move_to.1 as usize][move_to.0 as usize] = self.board[m.1 as usize][m.0 as usize];
            self.board[m.1 as usize][m.0 as usize] = MapState::Empty;
        }

        // Leave an open space where we started
        self.board[self.pos.1 as usize][self.pos.0 as usize] = MapState::Empty;

        // Update the player position
        self.pos = Board::adjacent_to(&self.pos, dir);

    }

    fn gps(&self) -> usize {
        let mut gps = 0;

        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                if (self.board[y][x] == MapState::Box) | (self.board[y][x] == MapState::BoxLeft) {
                    gps += 100*y + x;
                }
            }
        }

        gps
    }

    #[allow(dead_code)]
    fn show(&self) {
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                match self.board[y][x] {
                    MapState::Wall => { print!("#"); }
                    MapState::Empty => { print!("."); }
                    MapState::Box => { print!("O"); }
                    MapState::BoxLeft => { print!("["); }
                    MapState::BoxRight => { print!("]"); }
                    MapState::Player => { print!("@"); }
                }
            }
            println!();
        }
        println!();
    }
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    map: HashMap<(isize, isize), MapState>,
    size: (isize, isize),
    moves: Vec<Dir>,
    start_pos: (isize, isize),
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut map = HashMap::new();
        let mut size = (0, 0);
        let mut moves = Vec::new();
        let mut in_map = true;
        let mut y = 0;
        let mut x = 0;
        let mut start_pos = (0, 0);

        for line in text.lines() {
            if in_map {
                if line.trim().len() == 0 {
                    // Switch to processing moves
                    in_map = false;
                }
                else {
                    // Process a map line
                    for c in line.chars() {
                        match c {
                            '.' => { map.insert((x, y), MapState::Empty); }
                            '#' => { map.insert((x, y), MapState::Wall); }
                            'O' => { map.insert((x, y), MapState::Box); }
                            '@' => { 
                                map.insert((x, y), MapState::Player); 
                                start_pos = (x, y);
                            }
                            _ => ()
                        }
                        x += 1;
                        if x > size.0 { size.0 = x; }
                    }

                    x = 0;
                    y += 1; 
                    if y > size.1 { size.1 = y; }
                }
            }
            else {
                // Process a line of moves
                for c in line.chars() {
                    match c {
                        '^' => { moves.push(Dir::N); }
                        '>' => { moves.push(Dir::E); }
                        'v' => { moves.push(Dir::S); }
                        '<' => { moves.push(Dir::W); }                        
                        _ => ()
                    }
                }
            }
        }


        Input { map, size, moves, start_pos }
    }
}

pub struct Day15 {
}

// Day15
impl Day15 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day15 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);
        let mut board = Board::new(&input, false);

        for m in input.moves {
            board.do_move(&m);
        }

        Answer::Numeric(board.gps())
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);
        let mut board = Board::new(&input, true);

        for m in input.moves {
            board.do_move(&m);
        }

        Answer::Numeric(board.gps())
    }
}

#[cfg(test)]

mod test {

    use crate::day15::{Day15, Input, Board, Dir};
    use crate::day::{Day, Answer};
    
    // Example Input
    const EXAMPLE1: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.moves.len(), 70*10);
        assert_eq!(input.start_pos, (4, 4));
    }

    #[test]
    fn test_board() {
        let input = Input::read(EXAMPLE1);
        let board = Board::new(&input, false);

        assert_eq!(board.board.len(), 10);
        assert_eq!(board.board[0].len(), 10);
        assert_eq!(board.pos, (4, 4));
    }

    #[test]
    fn test_board2() {
        let input = Input::read(EXAMPLE1);
        let board = Board::new(&input, true);

        assert_eq!(board.board.len(), 10);
        assert_eq!(board.board[0].len(), 20);
        assert_eq!(board.pos, (8, 4));
    }

    #[test]
    fn test_moves() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, false);

        assert_eq!(board.pos, (4, 4));
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (3, 4));
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (2, 4));

        // Now it's blocked.
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (2, 4));
    }

    #[test]
    fn test_moves_p2() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, true);

        // starting position
        assert_eq!(board.pos, (8, 4));

        // Move once
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (7, 4));

        // Three more moves are OK.
        board.do_move(&Dir::W);
        board.do_move(&Dir::W);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (4, 4));

        // Now it's blocked.
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (4, 4));
    }

    #[test]
    fn test_moves2() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, false);

        assert_eq!(board.pos, (4, 4));
        board.do_move(&Dir::N);
        assert_eq!(board.pos, (4, 3));
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (3, 3));

        // Stuck now
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (3, 3));
    }

    
    #[test]
    fn test_moves2_p2() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, true);

        // Confirm starting position
        assert_eq!(board.pos, (8, 4));

        // Move North
        board.do_move(&Dir::N);
        assert_eq!(board.pos, (8, 3));

        // Two moves west are OK
        board.do_move(&Dir::W);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (6, 3));

        // Now blocked from moving West
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (6, 3));
    }
    
    #[test]
    fn test_moves3_p2() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, true);

        // Confirm starting position
        assert_eq!(board.pos, (8, 4));

        // Move South, West
        board.do_move(&Dir::S);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (7, 5));

        // One move north is OK
        board.do_move(&Dir::N);
        assert_eq!(board.pos, (7, 4));

        // Now blocked from moving North
        board.do_move(&Dir::N);
        assert_eq!(board.pos, (7, 4));
    }
        
    #[test]
    fn test_moves4_p2() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, true);

        // Confirm starting position
        assert_eq!(board.pos, (8, 4));

        // One move West, misaligning blocks
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (7, 4));

        // Move South, West
        board.do_move(&Dir::S);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (6, 5));

        // One move north is OK
        board.do_move(&Dir::N);
        assert_eq!(board.pos, (6, 4));

        // Now blocked from moving North
        board.do_move(&Dir::N);
        assert_eq!(board.pos, (6, 4));
    }

    #[test]
    fn test_ex1() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, false);

        for m in input.moves {
            board.do_move(&m);

        }

        assert_eq!(board.gps(), 10092);
    }

    
    #[test]
    fn test_ex1_p2() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input, true);

        for m in input.moves {
            board.do_move(&m);
        }

        assert_eq!(board.gps(), 9021);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day15::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(10092));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day15::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(9021));
    }
    
}