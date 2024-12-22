use std::collections::{HashMap, HashSet};

use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    secrets: Vec<usize>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let secrets = text.lines()
            .map(|line| {
                line.parse::<usize>().unwrap()
            })
            .collect();

        Input { secrets }
    }
}

pub struct Day22 {
}

// Day22
impl Day22 {
    pub const fn new() -> Self {
        Self { }
    }

    fn next_value(seed: usize, nth: usize) -> usize {
        let mut val = seed;

        for _n in 0..nth {
            val ^= val << 6; // multiply by 64, mix
            val &= 0xFFFFFF;   // prune
            val ^= val >> 5; // divide by 32, mix
            val &= 0xFFFFFF;   //prune
            val ^= val << 11;// multiply by 2048, mix
            val &= 0xFFFFFF    // prune      
        }

        val
    }

    fn scan_seq(seed: usize, len: usize, map: &mut HashMap<(i8, i8, i8, i8), usize>) {
        let mut diffs: [i8; 4] = [0; 4];

        let mut registered: HashSet<(i8, i8, i8, i8)> = HashSet::new();

        let mut generated = 0;
        let mut secret = seed;
        let mut price = (secret % 10) as i8;

        // Walk through the sequnce of <len> prices
        while generated < len {

            secret = Day22::next_value(secret, 1);
            let new_price = (secret % 10) as i8;
            let diff = new_price - price;
            generated += 1;
            price = new_price;

            // Shift new diff into the sequence.
            diffs[0] = diffs[1];
            diffs[1] = diffs[2];
            diffs[2] = diffs[3];
            diffs[3] = diff;

            // Don't look at diff sequences until after we see the fifth secret
            if generated < 4 { continue };

            let diff_seq = (diffs[0], diffs[1], diffs[2], diffs[3]);
         
            // Check whether this sequence has been registered for this buyer
            if !registered.contains(&(diffs[0], diffs[1], diffs[2], diffs[3])) {
                // This is a new sequence of diffs for this case.
                // Using this sequence would yield <new_price> bananas

                // TODO: There's a much better way to update a HashMap, I'm sure.
                if map.contains_key(&diff_seq) {
                    let mut val = *map.get(&diff_seq).unwrap();
                    val += new_price as usize;
                    map.insert(diff_seq, val);
                }
                else {
                    map.insert(diff_seq, new_price as usize);
                }

                registered.insert(diff_seq);
            }
        }
    }

    fn most_bananas(input: &Input) -> usize {
        let mut bananas_by_seq: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();

        // scan the sequences accumulating bananas for each price change sequence.
        for seed in input.secrets.iter() {
            Day22::scan_seq(*seed, 2000, &mut bananas_by_seq);
        }

        // Pick the maximum
        bananas_by_seq.iter()
            .map(|(_k, v)| { *v })
            .max().unwrap()
    }
}

impl<'a> Day for Day22 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let value: usize = input.secrets.iter()
            .map(|secret| {
                Day22::next_value(*secret, 2000)
            })
            .sum();

        Answer::Numeric(value)
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(Day22::most_bananas(&input))
    }
}

#[cfg(test)]

mod test {

    use crate::day22::{Day22, Input};
    use crate::day::{Day, Answer};
    
    // Example inputs
    const EXAMPLE1: &str = "\
1
10
100
2024
";

    const EXAMPLE2: &str = "\
1
2
3
2024
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.secrets.len(), 4);
        assert_eq!(input.secrets[0], 1);
        assert_eq!(input.secrets[1], 10);
        assert_eq!(input.secrets[2], 100);
        assert_eq!(input.secrets[3], 2024);
    }

    #[test]
    fn test_rng() {
        let sequence = vec![
            123,
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ];

        for n in 0..sequence.len()-1 {
            assert_eq!(Day22::next_value(sequence[n], 1), sequence[n+1]);
        }
    }

    #[test]
    fn test_p1() {
        let input = Input::read(EXAMPLE1);

        let value: usize = input.secrets.iter()
            .map(|secret| {
                Day22::next_value(*secret, 2000)
            })
            .sum();

        assert_eq!(value, 37327623);          
    }

    #[test]
    fn test_most_bananas_ex2() {
        let input = Input::read(EXAMPLE2);

        let most = Day22::most_bananas(&input);
        assert_eq!(most, 23);
    }

    #[test]
    fn test_most_bananas() {
        let input = Input::read(data_aoc2024::DAY22_INPUT);

        let most = Day22::most_bananas(&input);
        assert!(most > 1442);
        assert!(most < 1450);
        assert_eq!(most, 1449);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day22::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(37327623));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2_ex2() {
        // Based on the example in part 2.
        let d = Day22::new();
        assert_eq!(d.part2(EXAMPLE2), Answer::Numeric(23));
    }   
}