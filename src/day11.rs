use std::{collections::HashMap, mem};

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // Use STONE_RE.captures_iter(haystack).  Each cap[1] can be parsed as integer NNNNN
    static ref STONE_RE: Regex = Regex::new("(\\d+)").unwrap();
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    stones: Vec<usize>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let stones = STONE_RE.captures_iter(text)
            .map(|cap| { 
                cap[0].parse::<usize>().unwrap() 
            })
            .collect();

        Input { stones }
    }

    fn split_stone(stone: usize) -> Option<(usize, usize)> {
        let mut floor: usize = 10;
        let mut ceiling: usize = 100;
        let mut splitter: usize = 10;

        // Get the floor/ceiling values into the right power of 100.
        while stone >= ceiling {
            floor *= 100;
            ceiling *= 100;
            splitter *= 10;
        }
        if stone < floor {
            // Not an even number of digits
            None
        }
        else {
            // Even number of digits, return the split
            Some( (stone / splitter, stone % splitter) )
        }
    }

    fn blink(&self, count: usize) -> usize {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        let before = &mut v1;
        let after = &mut v2;

        for stone in &self.stones {
            before.push(*stone);
        }

        for _ in 0..count {
            // operate on all stones in before, placing results in after
            for stone in before.iter() {
                if *stone == 0 {
                    // 0 becomes 1
                    after.push(1 as usize);
                }
                else if let Some((a, b)) = Input::split_stone(*stone) {
                    after.push(a);
                    after.push(b);
                }
                else {
                    after.push(*stone * 2024);
                }
            }

            // swap before and after
            mem::swap(before, after);

            // clear after
            after.clear();
        }

        // result is the number of stones in before list
        before.len()
    }

    
    fn blink2(&self, count: usize) -> usize {
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

        self.stones.iter()
            .map(|s| {
                Input::blink_dfs(&mut cache, *s, count)
            })
            .sum()
    }

    // Stone counting by depth first search with a result cache.
    fn blink_dfs(cache: &mut HashMap::<(usize, usize), usize>, stone: usize, count: usize) -> usize {
        if count == 0 {
            // There is one stone.  Here, let me count it.
            1
        }
        else if let Some(result) = cache.get( &(stone, count) ) {
            // We've seen this situation before, get the solution we already have
            // println!("Cache hit on {}, {} -> {}", stone, count, *result);
            *result
        }
        else {
            // println!("Cache miss");
            // blink on this stone and check results
            let answer = if stone == 0 {
                // By rule, 0 -> 1
                Input::blink_dfs(cache, 1, count-1)
            }
            else if let Some((a, b)) = Input::split_stone(stone) {
                // By rule, even-length numbers are split
                Input::blink_dfs(cache, a, count-1) + Input::blink_dfs(cache, b, count-1)
            }
            else {
                // Default rule: multiply by 2024
                Input::blink_dfs(cache, stone*2024, count-1)
            };

            // Add this result to the cache
            cache.insert((stone, count), answer);

            answer
        }
    }
}

pub struct Day11 {
}

// Day11
impl Day11 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl<'a> Day for Day11 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.blink(25))
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(input.blink2(75))
    }
}

#[cfg(test)]

mod test {

    use crate::day11::{Day11, Input};
    use crate::day::{Day, Answer};
    
    // Example inputs
    const EXAMPLE1: &str = "\
125 17
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.stones.len(), 2);
        assert_eq!(input.stones[0], 125);
        assert_eq!(input.stones[1], 17);
    }

    #[test]
    fn test_blink() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.blink(1), 3);
        assert_eq!(input.blink(2), 4);
        assert_eq!(input.blink(3), 5);
        assert_eq!(input.blink(4), 9);
        assert_eq!(input.blink(5), 13);
        assert_eq!(input.blink(6), 22);
    }

    #[test]
    fn test_blink2() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.blink2(1), 3);
        assert_eq!(input.blink2(2), 4);
        assert_eq!(input.blink2(3), 5);
        assert_eq!(input.blink2(4), 9);
        assert_eq!(input.blink2(5), 13);
        assert_eq!(input.blink2(6), 22);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day11::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(55312));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day11::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(65601038650482));
    }
    
}