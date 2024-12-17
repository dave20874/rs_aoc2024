mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use day::{Day, Answer};
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;

use data_aoc2024::DAY1_INPUT;
use data_aoc2024::DAY2_INPUT;
use data_aoc2024::DAY3_INPUT;
use data_aoc2024::DAY4_INPUT;
use data_aoc2024::DAY5_INPUT;
use data_aoc2024::DAY6_INPUT;
use data_aoc2024::DAY7_INPUT;
use data_aoc2024::DAY8_INPUT;
use data_aoc2024::DAY9_INPUT;
use data_aoc2024::DAY10_INPUT;
use data_aoc2024::DAY11_INPUT;
use data_aoc2024::DAY12_INPUT;
use data_aoc2024::DAY13_INPUT;
use data_aoc2024::DAY14_INPUT;
use data_aoc2024::DAY15_INPUT;
use data_aoc2024::DAY16_INPUT;
use data_aoc2024::DAY17_INPUT;
use data_aoc2024::DAY18_INPUT;
use data_aoc2024::DAY19_INPUT;
use data_aoc2024::DAY20_INPUT;
use data_aoc2024::DAY21_INPUT;
use data_aoc2024::DAY22_INPUT;
use data_aoc2024::DAY23_INPUT;
use data_aoc2024::DAY24_INPUT;
use data_aoc2024::DAY25_INPUT;

static DAYS: [(&dyn Day, &str); 25] = [
    (&Day1::new(), DAY1_INPUT),  // Dec 1
    (&Day2::new(), DAY2_INPUT),
    (&Day3::new(), DAY3_INPUT),
    (&Day4::new(), DAY4_INPUT),
    (&Day5::new(), DAY5_INPUT),
    (&Day6::new(), DAY6_INPUT),
    (&Day7::new(), DAY7_INPUT),
    (&Day8::new(), DAY8_INPUT),
    (&Day9::new(), DAY9_INPUT),
    (&Day10::new(), DAY10_INPUT),
    (&Day11::new(), DAY11_INPUT),
    (&Day12::new(), DAY12_INPUT),
    (&Day13::new(), DAY13_INPUT),
    (&Day14::new(), DAY14_INPUT),
    (&Day15::new(), DAY15_INPUT),
    (&Day16::new(), DAY16_INPUT),
    (&Day17::new(), DAY17_INPUT),
    (&Day18::new(), DAY18_INPUT),
    (&Day19::new(), DAY19_INPUT),
    (&Day20::new(), DAY20_INPUT),
    (&Day21::new(), DAY21_INPUT),
    (&Day22::new(), DAY22_INPUT),
    (&Day23::new(), DAY23_INPUT),
    (&Day24::new(), DAY24_INPUT),
    (&Day25::new(), DAY25_INPUT),
];

fn report_day(day_no: usize) {
    
    let (day, text) = DAYS[day_no-1];

    let ans1 = day.part1(text);
    let msg1 = match ans1 {
        Answer::None => String::from("        -"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };

    let ans2 = day.part2(text);
    let msg2 = match ans2 {
        Answer::None => String::from("        -"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };
    println!("Day {day_no:2}: {msg1:>16} {msg2:>16}");
}


fn main() {
    println!("Advent of Code 2024!\n");

    let target_day = 0;

    match target_day {
        0 => {
            // report all days
            println!("{:7} {:>16} {:>16}", "", "Part 1", "Part 2");
            for day_no in 1..=25 {
                report_day(day_no);
            }
        }
        1..=25 => {
            // report a specific day
            report_day(target_day);
        }
        _ => {
            // invalid day 
            println!("Day {target_day} is invalid.\n");
        }
    }
    println!();

}

#[cfg(test)]
mod test {
    use crate::day::Answer;
    use crate::DAYS;

    const ANSWERS: [(Answer, Answer); 25] = [
        (Answer::Numeric(2000468), Answer::Numeric(18567089)),   // Dec 1
        (Answer::Numeric(663), Answer::Numeric(692)),
        (Answer::Numeric(192767529), Answer::Numeric(104083373)),
        (Answer::Numeric(2447), Answer::Numeric(1868)),
        (Answer::Numeric(7024), Answer::Numeric(4151)),
        (Answer::Numeric(4752), Answer::Numeric(1719)),
        (Answer::Numeric(8401132154762), Answer::Numeric(95297119227552)),
        (Answer::Numeric(323), Answer::Numeric(1077)),
        (Answer::Numeric(6421128769094), Answer::Numeric(6448168620520)),
        (Answer::Numeric(512), Answer::Numeric(1045)),  
        (Answer::Numeric(189547), Answer::Numeric(224577979481346)),
        (Answer::Numeric(1549354), Answer::Numeric(937032)),
        (Answer::Numeric(38839), Answer::Numeric(75200131617108)),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
        (Answer::None, Answer::None),
    ];

    #[test]
    fn test_all() {
        for day in 1..25 {
            let (d, text) = DAYS[day-1];
            assert_eq!(d.part1(text), ANSWERS[day-1].0);
            assert_eq!(d.part2(text), ANSWERS[day-1].1);
        }
    }

    #[test]
    fn test_day() {
        let day = 13;
        let (d, text) = DAYS[day-1];

        assert_eq!(d.part1(text), ANSWERS[day-1].0);
        assert_eq!(d.part2(text), ANSWERS[day-1].1);
    }

}
