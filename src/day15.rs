use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "NNNNN   MMMMM"
    // captures 1, 2 are the two integer inputs, N and M
    static ref LINE_RE: Regex = Regex::new("(\\d+)\\s+(\\d+)").unwrap();
}

enum Dir {
    N,E,S,W,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MapState {
    Empty,
    Box,
    Wall,
    Player,
}

struct Board {
    board: Vec<Vec<MapState>>,
    pos: (isize, isize),
}

impl Board {
    fn new(input: &Input) -> Board {
        let mut board = Vec::new();

        for y in 0..input.size.1 {
            let mut row = Vec::new();
            for x in 0..input.size.0 {
                row.push(input.map[&(x, y)]);
            }
            board.push(row);
        }

        Board { board, pos: input.start_pos }
    }

    fn can_move(&self, dir: &Dir) -> bool {
        let dxdy = match dir {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        };

        // Look in the indicated direction for an open space
        // If one is found before a wall, we can move
        let mut spot = self.pos;
        while self.board[spot.1 as usize][spot.0 as usize] != MapState::Wall {
            if self.board[spot.1 as usize][spot.0 as usize] == MapState::Empty {
                // We can move
                return true;
            }

            spot = (spot.0+dxdy.0, spot.1+dxdy.1)
        }

        // We didn't find that open spot
        false
    }

    fn do_move(&mut self, dir: &Dir) {
        if self.can_move(&dir) {
            let dxdy = match dir {
                Dir::N => (0, -1),
                Dir::E => (1, 0),
                Dir::S => (0, 1),
                Dir::W => (-1, 0),
            };
    
            // Look in the indicated direction for an open space
            // If one is found before a wall, we can move
            let spot = self.pos;
            let next_spot = (spot.0+dxdy.0, spot.1+dxdy.1);

            if self.board[next_spot.1 as usize][next_spot.0 as usize] == MapState::Box {
                // We're pushing boxes.
                let mut end_line = next_spot;

                while self.board[end_line.1 as usize][end_line.0 as usize] != MapState::Empty {
                    end_line = (end_line.0+dxdy.0, end_line.1+dxdy.1)
                }
                self.board[end_line.1 as usize][end_line.0 as usize] = MapState::Box;
            }

            self.board[next_spot.1 as usize][next_spot.0 as usize] = MapState::Player;
            self.board[spot.1 as usize][spot.0 as usize] = MapState::Empty;

            self.pos = next_spot;
        }
        
    }

    fn gps(&self) -> usize {
        let mut gps = 0;

        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                if self.board[y][x] == MapState::Box {
                    gps += 100*y + x;
                }
            }
        }

        gps
    }

    fn show(&self) {
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                match self.board[y][x] {
                    MapState::Wall => { print!("#"); }
                    MapState::Empty => { print!("."); }
                    MapState::Box => { print!("O"); }
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
        let mut board = Board::new(&input);

        for m in input.moves {
            board.do_move(&m);
        }

        Answer::Numeric(board.gps())
    }

    fn part2(&self, text: &str) -> Answer {
        let _input = Input::read(text);

        Answer::None
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

        // TODO-DW : Verify that inputs were read successfully.
        // assert_eq!(input.left.len(), 6);
    }

    #[test]
    fn test_board() {
        let input = Input::read(EXAMPLE1);
        let board = Board::new(&input);

        assert_eq!(board.board.len(), 10);
        assert_eq!(board.board[0].len(), 10);
        assert_eq!(board.pos, (4, 4));
    }

    #[test]
    fn test_moves() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input);

        assert_eq!(board.pos, (4, 4));
        assert_eq!(board.can_move(&Dir::W), true);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (3, 4));
        assert_eq!(board.can_move(&Dir::W), true);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (2, 4));
        assert_eq!(board.can_move(&Dir::W), false);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (2, 4));
    }

    #[test]
    fn test_moves2() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input);

        assert_eq!(board.pos, (4, 4));
        assert_eq!(board.can_move(&Dir::N), true);
        board.do_move(&Dir::N);
        assert_eq!(board.pos, (4, 3));
        assert_eq!(board.can_move(&Dir::W), true);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (3, 3));
        assert_eq!(board.can_move(&Dir::W), false);
        board.do_move(&Dir::W);
        assert_eq!(board.pos, (3, 3));
    }

    #[test]
    fn test_ex1() {
        let input = Input::read(EXAMPLE1);
        let mut board = Board::new(&input);

        for m in input.moves {
            board.do_move(&m);

        }

        assert_eq!(board.gps(), 10092);
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
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}