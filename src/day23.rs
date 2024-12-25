use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day::{Day, Answer};


lazy_static! {
    // When used on text like "ab-cd"
    // captures 1 is "ab", capture 2 is "cd"
    static ref LINE_RE: Regex = Regex::new("([a-z]{2})-([a-z]{2})").unwrap();
}

struct Pair {
    first: String,
    second: String,
}

// A representation of the puzzle inputs.
// Today it's just a list (Vec) of Strings, one for each input line.
struct Input {
    pairs: Vec<Pair>,
}

impl Input {
    fn read(text: &str) -> Input
    {
        let pairs = text.lines()
            .filter_map(|line| { LINE_RE.captures(line)})
            .map(|caps| { 
                Pair { first: caps[1].to_string(), second: caps[2].to_string() }
            })
            .collect();

        Input { pairs }
    }
}

struct Network {
    nodes: Vec<String>,         // Nodes are identified by their position in nodes vec.
    neighbors: Vec<Vec<usize>>,  // For each node, a Vector of connected nodes, (ref by index.)
}

impl Network {
    fn new(input: &Input) -> Network {
        let mut nodes = Vec::new();
        let mut neighbors = Vec::new();
        let mut name_to_id: HashMap<String, usize> = HashMap::new();

        // From nodes, construct 
        for pair in &input.pairs {
            let first_id = match name_to_id.get(&pair.first) {
                Some(first_id) => { *first_id },
                None => {
                    // New node id, register in nodes and name_to_id
                    nodes.push(pair.first.clone());
                    neighbors.push(Vec::new());
                    name_to_id.insert(pair.first.clone(), nodes.len());
                    nodes.len()
                }
            };
            let second_id = match name_to_id.get(&pair.second) {
                Some(second_id) => { *second_id },
                None => {
                    // New node id, register in nodes and name_to_id
                    nodes.push(pair.second.clone());
                    neighbors.push(Vec::new());
                    name_to_id.insert(pair.second.clone(), nodes.len());
                    nodes.len()
                }
            };

            neighbors[first_id].push(second_id);
            neighbors[second_id].push(first_id);
        }

        Network { nodes, neighbors }
    }

    // Get a nodes name
    fn node_name(&self, n: usize) -> &str {
        &self.nodes[n]
    }

    // Return a set (repr as a vector) of fully connected sets (another Vec)
    // Nodes are represented by id, usize
    fn fc(&self, n: usize) -> Vec<Vec<usize>> {

    }
}

pub struct Day23 {
}

// Day23
impl Day23 {
    pub const fn new() -> Self {
        Self { }
    }

    fn t_triples(input: &Input) -> usize {
        // Construct a set of all triples by constructing a Network and
        // asking it for all fully connected sets of 3.
        let network = Network::new(input);
        let triples = network.fc(3);

        triples.iter()
    }
}

impl<'a> Day for Day23 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let _input = Input::read(text);

        Answer::None
    }

    fn part2(&self, text: &str) -> Answer {
        let _input = Input::read(text);

        Answer::None
    }
}

#[cfg(test)]

mod test {

    use crate::day23::{Day23, Input};
    use crate::day::{Day, Answer};
    
    // Example inputs
    const EXAMPLE1: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    // Read and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.pairs.len(), 32);
        assert_eq!(input.pairs[0].first, "kh");
        assert_eq!(input.pairs[31].second, "yn");
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d= Day23::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::None);
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day23::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}