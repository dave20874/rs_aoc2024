use std::collections::{HashMap, HashSet};

use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    // puzzle[row][col] -> char
    map: Vec<Vec<char>>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let mut map: Vec<Vec<char>> = Vec::new();

        for line in text.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            map.push(row);
        }

        Input { map }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Dir {
    N, E, S, W,
}

struct Region {
    area: HashSet<(usize, usize)>,
    perimeter: usize,

    // set of perimeter crossings ((inner_x, inner_y), (outer_x, outer_y))
    perimeter_crossings: HashSet<(usize, usize, Dir)>,
}

impl Region {
    fn generate(input: &Input, start: (usize, usize)) -> Region {
        // Implement flood fill to find connected area
        let key = input.map[start.1][start.0];
        let height = input.map.len();
        let width = input.map[0].len();
        let mut area = HashSet::new();
        let mut perimeter = 0;
        let mut perimeter_crossings = HashSet::new();

        // let mut covered: HashSet<(usize, usize)> = HashSet::new();
        let mut to_check: Vec<(usize, usize)> = Vec::new();
        area.insert(start);
        to_check.push(start);

        // Only push things to to_check after they are verified in the area.
        while let Some(cell) = to_check.pop() {
            for (adj_x, adj_y, dir) in 
                vec![(cell.0 as isize, cell.1 as isize+1, Dir::N), 
                     (cell.0 as isize, cell.1 as isize-1, Dir::S), 
                     (cell.0 as isize+1, cell.1 as isize, Dir::E), 
                     (cell.0 as isize-1, cell.1 as isize, Dir::W)] {
                if (adj_x >= 0) & (adj_x < width as isize) & (adj_y >= 0) & (adj_y < height as isize) {
                    // Adjacent space is in map
                    let u_adjacent = (adj_x as usize, adj_y as usize);
                    if input.map[u_adjacent.1][u_adjacent.0] == key {
                        // The adjacent space is part of the same area
                        if !area.contains(&u_adjacent) {
                            // covered.insert(adjacent);
                            area.insert(u_adjacent);
                            to_check.push(u_adjacent);
                        }
                    }
                    else {
                        // The adjacent space is a different area
                        perimeter_crossings.insert( (cell.0, cell.1, dir) );
                        perimeter += 1;
                    }
                }
                else {
                    // Adjacent space is off map, this is a perimeter
                    perimeter_crossings.insert( (cell.0, cell.1, dir) );
                    perimeter += 1;
                }
            }
        }

        Region { area, perimeter, perimeter_crossings }
    }

    fn area(&self) -> usize {
        self.area.len()
    }

    fn perimeter(&self) -> usize {
        self.perimeter
    }

    fn num_sides(&self) -> usize {
        // in_line_sides[Dir][Coord1] -> Vec of Coord2
        // Where Coord1 is X for E/W dir, Y for N/S dir, 
        let mut in_line_sides: HashMap<Dir, HashMap<usize, Vec<usize>> > = HashMap::new();
        for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
            in_line_sides.insert(d, HashMap::new());
        }

        // Put sides into four vectors based on whether they
        // are NSEW edges of the area.  Then separate by lat/lon
        // (depending on orientation).  Then sort by lon/lat within
        // those.  
        for (x, y, dir) in self.perimeter_crossings.iter() {
            let (coord1, coord2) = match dir {
                Dir::N => (y, x),
                Dir::S => (y, x),
                Dir::E => (x, y),
                Dir::W => (x, y),
            };
            if !in_line_sides[dir].contains_key(coord1) {
                // Create a vector for this coord1, 
                in_line_sides.get_mut(dir).unwrap()
                    .insert(*coord1, Vec::new());
            }
            in_line_sides
                .get_mut(dir).unwrap()
                .get_mut(coord1).unwrap()
                .push(*coord2);
        }

        // Finally, look for runs within those.  These
        // represent sides.
        let mut lines = 0;
        for (_dir, dir_lines) in in_line_sides.iter_mut() {
            for (_coord1, v) in dir_lines.iter_mut() {
                // sort the coord2 values
                v.sort();

                let gaps: usize = v.windows(2)
                    .map(|x| { if x[0]+1 < x[1] { 1 } else { 0 } } )
                    .sum();
                lines += gaps+1;
            }
        }

        lines
    }

    fn price(&self) -> usize {
        self.perimeter() * self.area()
    }

    fn new_price(&self) -> usize {
        self.num_sides() * self.area()
    }
}

struct GardenMap {
    regions: Vec<Region>,
}

impl GardenMap {
    fn new(input: &Input) -> GardenMap {
        let mut regions = Vec::new();
        let mut mapped: HashSet<(usize, usize)> = HashSet::new();

        for y in 0..input.map.len() {
            for x in 0..input.map[0].len() {
                if !mapped.contains(&(x, y)) {
                    // create a new region from area adjacent to (x, y)
                    let new_region = Region::generate(&input, (x, y));

                    // Mark the whole new region mapped
                    for cell in new_region.area.iter() {
                        mapped.insert(*cell);
                    }

                    regions.push(new_region);
                }
            }
        }

        GardenMap { regions }
    }
}

pub struct Day12 {
}

// Day12
impl Day12 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day12 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);
        let map = GardenMap::new(&input);

        let price = map.regions.iter()
            .map(|r| { r.price() })
            .sum::<usize>();

        Answer::Numeric(price)
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);
        let map = GardenMap::new(&input);

        let price = map.regions.iter()
            .map(|r| { r.new_price() })
            .sum::<usize>();

        Answer::Numeric(price)
    }
}

#[cfg(test)]

mod test {

    use crate::day12::{Day12, Input, GardenMap};
    use crate::day::{Day, Answer};
    
    // Example Inputs
    const EXAMPLE1: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.map.len(), 10);
        assert_eq!(input.map[0].len(), 10);
        assert_eq!(input.map[0][0], 'R');
        assert_eq!(input.map[2][0], 'V');
        assert_eq!(input.map[0][2], 'R');
    }

    #[test]
    fn test_map() {
        let input = Input::read(EXAMPLE1);
        let map = GardenMap::new(&input);

        assert_eq!(map.regions.len(), 11);
        assert_eq!(map.regions[0].area(), 12);
        assert_eq!(map.regions[0].perimeter(), 18);
        assert_eq!(map.regions[0].perimeter_crossings.len(), 18);
        assert_eq!(map.regions[0].num_sides(), 10);
        assert_eq!(map.regions[0].price(), 216);
        assert_eq!(map.regions[0].new_price(), 120);
        assert_eq!(map.regions.iter()
                      .map(|r| { r.price() })
                      .sum::<usize>(), 1930);
        assert_eq!(map.regions.iter()
                    .map(|r| { r.new_price() })
                    .sum::<usize>(), 1206);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day12::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(1930));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day12::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(1206));
    }
    
}