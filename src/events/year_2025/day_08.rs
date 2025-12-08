use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
};

use anyhow::Result;

use crate::utils::{variable::variable, *};

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let junctions_to_connect = variable(10usize, 1000);

        let (mut junctions, mut circuits) = parse(input);

        // Find max based on size.
        let connections = find(&junctions, *junctions_to_connect);
        let connections = connections.into_sorted_vec();
        // reverse them into sorted

        for next in connections {
            let first = &junctions[next.id1];
            let second = &junctions[next.id2];

            let parent = first.circuit_id.min(second.circuit_id);
            let child = first.circuit_id.max(second.circuit_id);

            // If they're already in group together
            if parent == child {
                continue;
            }

            junctions[next.id1].circuit_id = parent;
            junctions[next.id2].circuit_id = parent;

            let child = std::mem::take(&mut circuits[child]).unwrap();

            for member in child.members {
                circuits[parent].as_mut().unwrap().members.insert(member);
                junctions[member].circuit_id = parent;
            }
        }

        circuits.retain(|x| x.is_some());
        circuits.sort();

        Ok(circuits
            .into_iter()
            .filter_map(|x| x)
            .take(3)
            .fold(1, |acc, item| acc * item.members.len())
            .into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let junction_count = input.len();

        let (mut junctions, mut circuits) = parse(input);

        // Find max based on size.
        let connections = find(&junctions, junctions.len() * 10); // find many
        let connections = connections.into_sorted_vec();
        // reverse them into sorted

        for next in connections {
            let first = &junctions[next.id1];
            let second = &junctions[next.id2];

            let parent = first.circuit_id.min(second.circuit_id);
            let child = first.circuit_id.max(second.circuit_id);

            // If they're already in group together
            if parent == child {
                continue;
            }

            junctions[next.id1].circuit_id = parent;
            junctions[next.id2].circuit_id = parent;

            let child = std::mem::take(&mut circuits[child]).unwrap();

            for member in child.members {
                circuits[parent].as_mut().unwrap().members.insert(member);
                junctions[member].circuit_id = parent;
            }

            if circuits[parent].as_ref().unwrap().members.len() == junction_count {
                let value = junctions[next.id1].x * junctions[next.id2].x;
                return Ok(value.into());
            }
        }

        anyhow::bail!("Did not find answer??")
    }
}

fn find(input: &[Tensor], max_count: usize) -> BinaryHeap<ShortestConnection> {
    let mut paths = BinaryHeap::new();

    for idx in 0..(input.len() - 1) {
        let curr = &input[idx];

        for i in (idx + 1)..input.len() {
            let distance = input[i].euclidian_distance_lossy(&curr);
            let shortest = ShortestConnection {
                distance,
                id1: idx,
                id2: i,
            };
            paths.push(shortest);
            if paths.len() > max_count {
                paths.pop();
            }
        }
    }

    paths
}

fn parse(input: &[String]) -> (Vec<Tensor>, Vec<Option<Circuit>>) {
    let junctions: Vec<_> = input
        .into_iter()
        .enumerate()
        .map(|(idx, s)| {
            let mut numbers = s.split(',').map(|s| s.number());
            Tensor {
                x: numbers.next().unwrap(),
                y: numbers.next().unwrap(),
                z: numbers.next().unwrap(),
                circuit_id: idx,
            }
        })
        .collect();

    let circuits: Vec<Option<Circuit>> = (0..junctions.len())
        .into_iter()
        .map(|x| {
            Some(Circuit {
                id: x,
                members: [x].into(),
            })
        })
        .collect();

    (junctions, circuits)
}
#[derive(Debug, PartialEq, Eq, Ord)]
struct ShortestConnection {
    distance: usize,
    id1: usize,
    id2: usize,
}

impl PartialOrd for ShortestConnection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Circuit {
    id: usize,
    members: HashSet<usize>,
}

impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.members.len().partial_cmp(&self.members.len())
    }
}

struct Tensor {
    x: isize,
    y: isize,
    z: isize,
    circuit_id: usize,
}

impl Tensor {
    pub fn euclidian_distance_lossy(&self, other: &Tensor) -> usize {
        let dist =
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (dist as f64).sqrt() as usize
    }
}

impl Debug for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}   {:?}",
            self.x, self.y, self.z, self.circuit_id
        )
    }
}
