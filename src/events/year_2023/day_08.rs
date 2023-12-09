use std::collections::HashMap;

use anyhow::Result;

use crate::utils::*;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let (pattern, graph) = parse(input);

        let start = graph.get(&String::from("AAA")).unwrap();
        let end = graph.get(&String::from("ZZZ")).unwrap();

        let mut current = start;
        let mut count = 0;

        while current != end {
            let move_index = count % pattern.len();

            current = match pattern[move_index] {
                'R' => graph.get(&current.right).unwrap(),
                'L' => graph.get(&current.left).unwrap(),
                _ => unreachable!(),
            };

            count += 1;
        }

        Ok(count.into())
    }

    fn run_part2(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let (pattern, graph) = parse(input);

        let ghost_nodes: Vec<_> = graph
            .keys()
            .filter(|k| k.ends_with('A'))
            .filter_map(|k| graph.get(k))
            .collect();

        // Tried bruteforcing, dident work.
        // Since it's a repeatable pattern problem, we can determine how often each node "cycles"
        // Then we just gotta find the lowest common multiple

        let cycles: Vec<_> = ghost_nodes
            .iter()
            .map(|&node| {
                let mut count = 0;
                let mut current = node;

                while !current.name.ends_with("Z") {
                    let move_index = count % pattern.len();
                    current = match pattern[move_index] {
                        'R' => graph.get(&current.right).unwrap(),
                        'L' => graph.get(&current.left).unwrap(),
                        _ => unreachable!(),
                    };

                    count += 1;
                }

                count
            })
            .collect();

        let lcm = math_utils::lcm_multiple(cycles.as_slice());

        Ok(lcm.into())
    }
}

fn parse(input: &Vec<String>) -> (Vec<char>, HashMap<String, Node>) {
    let pattern: Vec<_> = input[0].chars().collect();

    let mut graph = HashMap::new();

    for line in input.iter().skip(2) {
        // AAA = (BBB, BBB)
        let node = Node {
            name: line[..3].to_string(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string(),
        };

        graph.insert(node.name.clone(), node);
    }

    (pattern, graph)
}
