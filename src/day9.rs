use std::collections::VecDeque;

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
    digits: Vec<u8>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let digits = text
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10).unwrap() as u8 
            })
            .collect();
        Input { digits}
    }
}

type Block = Option<usize>;  // usize value is file id.

struct Disk {
    data_blocks: usize, // count of data blocks
    blocks: Vec<Block>,
}

impl Disk {
    fn new(input: &Input) -> Disk {
        let mut data_blocks = 0;
        let mut blocks = Vec::new();


        let mut in_data = true;
        let mut file_id = 0;

        for digit in &input.digits {
            match in_data {
                true => {
                    // digit represents data blocks
                    for _ in 0..*digit {
                        blocks.push(Some(file_id));
                        data_blocks += 1;
                    }
                    file_id += 1;
                }
                false => {
                    // digit represents space
                    for _ in 0..*digit {
                        blocks.push(None);
                    }
                }
            }

            // switch between data and free space
            in_data = !in_data;
        }

        Disk { data_blocks, blocks }
    }

    fn defrag(&mut self) {
        for n in 0..self.data_blocks {
            match self.blocks[n] {
                Some(_) => (),  // do nothing with data blocks
                None => {
                    // Replace this empty block with a full one
                    self.blocks.swap_remove(n);

                    // If that exposed unused blocks at the end, remove them
                    while self.blocks.last().unwrap() == &None {
                        self.blocks.remove(self.blocks.len()-1);
                    }
                }
            }
        }
    }

    fn scan_blocks(&self) -> ([VecDeque<usize>; 10], Vec<(usize, usize, usize)>) {
        let mut free_blocks: [VecDeque<usize>; 10] = [const { VecDeque::new() }; 10];
        let mut blocks_to_move: Vec<(usize, usize, usize)> = Vec::new(); // start, len, file_id
        let mut in_data = false;
        let mut last_file_id = 0;

        let mut run_len: usize = 0;

        // Initial scan to construct data_blocks and free_blocks
        for n in 0..self.blocks.len() {
            match self.blocks[n] {
                Some(file_id) => {
                    // We are in data, is this a new block
                    if !in_data {
                        // end free block we were in
                        free_blocks[run_len].push_back(n-run_len);  // record that free block's start

                        // start new data block
                        run_len = 1;
                        in_data = true;
                        last_file_id = file_id;
                    }
                    else if file_id != last_file_id {
                        // end data block we were in
                        blocks_to_move.push( (n-run_len, run_len, last_file_id));

                        // start new data block
                        run_len = 1;
                        last_file_id = file_id;
                    }
                    else {
                        // continue the data block we were in
                        run_len += 1;
                    }
                }
                None => {
                    if in_data {
                        // end data block we were in
                        blocks_to_move.push( (n-run_len, run_len, last_file_id));

                        // start new free block
                        run_len = 1;
                        in_data = false;
                    }
                    else {
                        // continue the free block we already in
                        run_len += 1;
                    }
                }
            }
        }

        // close out last run
        let n = self.blocks.len();
        if in_data {
            // end data block we were in
            blocks_to_move.push( (n-run_len, run_len, last_file_id));
        } 
        else {
            // end free block we were in
            free_blocks[run_len].push_back(n-run_len);  // record that free block's start
        }

        (free_blocks, blocks_to_move)
    }

    fn defrag2(&mut self) {
        let (mut free_blocks, blocks_to_move) = self.scan_blocks();

        for n in (0..blocks_to_move.len()).rev() {
            let (start, len, _file_id) = blocks_to_move[n];

            let mut first_match: Option<(usize, usize)> = None;  // len, start of empty block
            // println!("Tyring to move file {}", _file_id);

            // find the first place it will fit
            for use_len in len..10 {
                
                // Use a free block of length use_len. 
                // break if successful, otherwise loop will try larger blocks
                if let Some(start) = free_blocks[use_len].front() {
                    if let Some( (_len, best_start)) = first_match {
                        if start < &best_start {
                            // This is the new best start
                            // println!("  Found a better spot at {}, (len: {})", start, use_len);
                            first_match = Some((use_len, *start));
                        }
                    }
                    else {
                        // This is the first best start
                        // println!("  Found first spot at {}, (len: {})", start, use_len);
                        first_match = Some((use_len, *start));
                    }
                }            
            }

            // move the stuff, if we found a place for it
            match first_match {
                Some((use_len, _free_start)) => {
                    match free_blocks[use_len].pop_front() {
                        Some(free_start) => {
                            if free_start < start {
                                // println!("  Moving file {} from {} to {}", _file_id, start, free_start);
                                // Copy len blocks from start to free_start
                                for i in 0..len {
                                    self.blocks.swap(free_start+i, start+i);
                                }

                                // Put unused free blocks back into a free list.
                                let leftover_size = use_len-len;
                                if leftover_size > 0 {
                                    // println!("  Leftover piece sized {}", leftover_size);
                                    let leftover_location = free_start+len;
                                    let idx = free_blocks[leftover_size].partition_point(|&x| x <= leftover_location);
                                    free_blocks[leftover_size].insert(idx, leftover_location);
                                }
                            }
                            else {
                                // println!("  Found spot is worse that starting spot.");
                            }
                        }
                        None => {
                            assert!(false, "Not possible.");
                        }
                    }   
                }
                None => {
                    // No place to move it.
                    // println!("Couldn't move file {}", _file_id);
                }
            }         
        }

    }

    fn checksum(&self) -> usize {
        let mut sum = 0;
        for (n, block) in self.blocks.iter().enumerate() {
            match block {
                Some(file_id) => {
                    sum += file_id*n;
                }
                None => ()
            }
        }

        sum
    }
}

pub struct Day9 {
}

// Day9
impl Day9 {
    pub const fn new() -> Self {
        Self { }
    }

}

impl<'a> Day for Day9 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let mut disk = Disk::new(&Input::read(text));

        disk.defrag();

        Answer::Numeric(disk.checksum())
    }

    fn part2(&self, text: &str) -> Answer {
        let mut disk = Disk::new(&Input::read(text));

        disk.defrag2();

        Answer::Numeric(disk.checksum())
    }
}

#[cfg(test)]

mod test {

    use crate::day9::{Day9, Input, Disk};
    use crate::day::{Day, Answer};
    
    // TODO Place example inputs here.
    const EXAMPLE1: &str = "\
2333133121414131402
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.digits.len(), 19);
    }

    #[test]
    fn test_disk() {
        let disk = Disk::new(&Input::read(EXAMPLE1));

        assert_eq!(disk.data_blocks, 28);
        assert_eq!(disk.blocks.len(), 42);
    }

    #[test]
    fn test_defrag() {
        let mut disk = Disk::new(&Input::read(EXAMPLE1));

        disk.defrag();

        assert_eq!(disk.data_blocks, 28);
        assert_eq!(disk.blocks.len(), 28);
    }

    #[test]
    fn test_scan() {
        let disk = Disk::new(&Input::read(EXAMPLE1));
        let (free_blocks, data_blocks) = disk.scan_blocks();

        assert_eq!(free_blocks[1].len(), 5);
        assert_eq!(free_blocks[3].len(), 3);

        assert_eq!(data_blocks.len(), 10);
    }
    
    #[test]
    fn test_defrag2() {
        let mut disk = Disk::new(&Input::read(EXAMPLE1));

        disk.defrag2();

        assert_eq!(disk.data_blocks, 28);
        assert_eq!(disk.blocks.len(), 42);
        assert_eq!(disk.checksum(), 2858);
    }

    #[test]
    fn test_checksum() {
        let mut disk = Disk::new(&Input::read(EXAMPLE1));

        disk.defrag();

        assert_eq!(disk.checksum(), 1928);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day9::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(1928));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day9::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(2858));
    }
    
}