use std::collections::{HashMap, HashSet};

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
    neighbors: Vec<(usize, usize)>,  // For each node, a Vector of connected nodes, (ref by index.)
    fc_n: Vec<HashSet<Vec<usize>>>,
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
                    name_to_id.insert(pair.first.clone(), nodes.len()-1);
                    nodes.len()-1
                }
            };
            let second_id = match name_to_id.get(&pair.second) {
                Some(second_id) => { *second_id },
                None => {
                    // New node id, register in nodes and name_to_id
                    nodes.push(pair.second.clone());
                    name_to_id.insert(pair.second.clone(), nodes.len()-1);
                    nodes.len()-1
                }
            };

            neighbors.push((first_id, second_id));
        }

        // Vector of fully connected N sets.
        // Entries 0 and 1 are empty
        // Later entries will be computed and added here.
        let fc_n = vec![HashSet::new(), HashSet::new()];


        Network { nodes, neighbors, fc_n }
    }

    #[allow(unused)]
    // Get a nodes name
    fn node_name(&self, n: usize) -> &str {
        &self.nodes[n]
    }

    // Return a set (repr as a vector) of fully connected sets (another Vec)
    // Nodes are represented by id, usize
    fn fc(&mut self, n: usize) /* -> HashSet<Vec<usize>> */ {
        

        if n <= 1 {
            panic!("It don't work like that.");
        }
        else if n < self.fc_n.len() {
            // Already computed this
            return;
        }
        else if n == 2 {
            let mut fc_sets = HashSet::new();

            // From each pair, create a minimal connected set
            for pair in &self.neighbors {
                let mut connected_set = Vec::new();
                connected_set.push(pair.0);
                connected_set.push(pair.1);
                connected_set.sort();
                fc_sets.insert(connected_set);
            }

            self.fc_n.push(fc_sets);
        }
        else {
            // Get the set of fc(n-1) sets
            self.fc(n-1);
            let fc_m1 = &self.fc_n[n-1];

            let mut fc_sets = HashSet::new();

            // For each set in fc_m1, look for nodes not in a set but connected to every node in the set 
            for set in fc_m1.iter() {
                'nodes: for node in 0..self.nodes.len() {
                    if set.contains(&node) { continue; }

                    for set_node in set.iter() {
                        // node isn't connected to some node in the set, 
                        if !self.neighbors.contains(&(*set_node, node)) & 
                           !self.neighbors.contains(&(node, *set_node)) { continue 'nodes; }
                    }

                    // We can create an fc(n) set from set and node
                    let mut new_set = set.clone();
                    new_set.push(node);
                    new_set.sort();

                    fc_sets.insert(new_set);
                }
            }

            self.fc_n.push(fc_sets);
        }

    }

    fn is_candidate(&self, node: &usize) -> bool {
        self.nodes[*node].starts_with("t")
    }

    fn max_fc_size(&mut self) -> usize {

        let mut size = 2;
        loop {
            self.fc(size);
            if self.fc_n[size].len() == 0 {
                break;
            }
            size += 1;
        };

        // Max fc size is now size-1
        size-1
    }

    fn lan_passwd(&mut self) -> String {
        // Analyze the network to determine max fc_size
        let max_fc = self.max_fc_size();
        let mut nodes = Vec::new();

        if let Some(s) = self.fc_n[max_fc].iter().next() {
            // Collect the nodes of this set
            for node in s {
                nodes.push(&self.nodes[*node]);
            }
        }

        nodes.sort();

        let mut passwd: String = String::new();
        for n in 0..nodes.len() {
            if n > 0 { passwd.push(','); }
            passwd.push_str(&nodes[n]);
        }

        passwd
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
        let mut network = Network::new(input);
        network.fc(3);
        let triples = &network.fc_n[3];

        let t_triples: Vec<&Vec<usize>> = triples.iter()
            .filter(|v| { 
                v.iter().fold(false, |accum, node| {
                    let is_t = network.is_candidate(node);

                    accum | is_t
                })
            }).collect();

        t_triples.len()
    }
}

impl<'a> Day for Day23 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        Answer::Numeric(Self::t_triples(&input))
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);
        let mut network = Network::new(&input);

        Answer::String(network.lan_passwd())
    }
}

#[cfg(test)]

mod test {

    use crate::day23::{Day23, Input, Network};
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
    fn test_fc3() {
        let input = Input::read(EXAMPLE1);
        let mut network = Network::new(&input);

        network.fc(3);

        assert_eq!(network.fc_n[2].len(), 32);
        assert_eq!(network.fc_n[3].len(), 12);
    }

    #[test]
    fn test_t_triples() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(Day23::t_triples(&input), 7);
    }

    #[test]
    fn test_max_fc() {
        let input = Input::read(EXAMPLE1);
        let mut network = Network::new(&input);

        assert_eq!(network.max_fc_size(), 4);
    }

    #[test]
    fn test_lan_passwd_ex1() {
        let input = Input::read(EXAMPLE1);
        let mut network = Network::new(&input);

        assert_eq!(network.lan_passwd(), "co,de,ka,ta");
    }


    #[test]
    fn test_max_fc_d23() {
        let input = Input::read(data_aoc2024::DAY23_INPUT);
        let mut network = Network::new(&input);

        assert_eq!(network.max_fc_size(), 13);
    }

    // TODO : Compute LAN password from max_fc
    #[test]

    fn test_lan_passwd() {
        let input = Input::read(data_aoc2024::DAY23_INPUT);
        let mut network = Network::new(&input);

        assert_eq!(network.lan_passwd(), "az,ed,hz,it,ld,nh,pc,td,ty,ux,wc,yg,zz");
    }
    
    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1_ex1() {
        // Based on the example in part 1.
        let d= Day23::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(7));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1_d23() {
        // Based on the example in part 1.
        let d= Day23::new();
        assert_eq!(d.part1(data_aoc2024::DAY23_INPUT), Answer::Numeric(926));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day23::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::String(String::from("co,de,ka,ta")));
    }
    
}