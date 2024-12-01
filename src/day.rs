#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Answer {
    None,
    Numeric(usize),
    String(String),
}

pub trait Day: Sync {

    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}