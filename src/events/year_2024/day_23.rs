use std::{
    collections::{HashMap, HashSet},
    vec,
};

use anyhow::Result;

use crate::utils::*;

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let map = parse(input);

        let connections: HashSet<Connection> =
            map.keys().flat_map(|&key| union(key, &map)).collect();

        Ok(connections
            .into_iter()
            .filter(|conn| conn.p1())
            .count()
            .into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let map = parse(input);

        Ok(biggest_union(map).into())
    }
}

fn parse(input: &[String]) -> HashMap<u16, Vec<u16>> {
    let mut cache: HashMap<_, Vec<_>> = HashMap::new();

    for s in input {
        let (n1, n2) = s.split_once('-').unwrap();
        let n1: u16 = u16::from_be_bytes(n1.as_bytes().try_into().unwrap());
        let n2: u16 = u16::from_be_bytes(n2.as_bytes().try_into().unwrap());

        cache.entry(n1).or_default().push(n2);
        cache.entry(n2).or_default().push(n1);
    }

    cache
}

fn union(key: u16, map: &HashMap<u16, Vec<u16>>) -> Vec<Connection> {
    let mut result = vec![];
    let connected_nodes = map.get(&key).unwrap();

    let node_len = connected_nodes.len();
    for i in 0..(node_len - 1) {
        let n2 = connected_nodes[i];
        for j in i..node_len {
            let n3 = connected_nodes[j];

            if can_connect(n2, n3, map) {
                result.push(Connection::make([key, n2, n3]));
            }
        }
    }

    result
}

fn biggest_union(map: HashMap<u16, Vec<u16>>) -> String {
    let mut biggest_clique = vec![];
    let mut has_seen = HashSet::new();

    for key in map.keys() {
        // If we have already processed it, just continue, since we have already explored that graph.
        if !has_seen.insert(*key) {
            continue;
        }

        let conns = map.get(key).unwrap();
        // If conns is less than the current biggest graph, just return.
        if conns.len() < biggest_clique.len() {
            continue;
        }

        let mut current_clique = vec![*key];

        for &conn in conns {
            // If can connect to all edges
            if current_clique.iter().all(|&c| can_connect(conn, c, &map)) {
                has_seen.insert(conn); // We have evaluated this connection
                current_clique.push(conn);
            }
        }

        if current_clique.len() > biggest_clique.len() {
            biggest_clique = current_clique;
        }
    }

    // format output, nodes must be sorted alplabetically.
    let mut strings: Vec<String> = biggest_clique
        .into_iter()
        .map(|x| String::from_utf8(x.to_be_bytes().to_vec()).unwrap())
        .collect();

    strings.sort();

    strings.join(",")
}

fn can_connect(n2: u16, n3: u16, map: &HashMap<u16, Vec<u16>>) -> bool {
    let n2_connected = map.get(&n2).unwrap();

    n2_connected.iter().any(|&conn| conn == n3)
}

#[derive(PartialEq, Eq, Hash)]
struct Connection {
    nodes: [u16; 3],
}

impl Connection {
    fn make(mut nodes: [u16; 3]) -> Self {
        nodes.sort();
        Self { nodes }
    }

    fn p1(&self) -> bool {
        self.nodes.iter().any(|&n| {
            let [first, _] = n.to_be_bytes();
            first == b't'
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_p1() {
        let n1: u16 = u16::from_be_bytes("ta".as_bytes().try_into().unwrap());

        let conn = Connection { nodes: [n1, 5, 5] };

        assert!(conn.p1())
    }

    #[test]
    fn can_make_conn() {
        let input: Vec<String> = vec!["ta-ch".into(), "ch-xx".into(), "xx-ta".into()];
        let map = parse(&input);
        let n1: u16 = u16::from_be_bytes("ta".as_bytes().try_into().unwrap());

        let mut un = union(n1, &map);

        assert!(un.len() == 1);

        let conn = un.pop().unwrap();
        assert!(conn.p1())
    }

    #[test]
    fn biggest_union_test() {
        let input: Vec<String> = vec!["ta-ch".into(), "ch-xx".into(), "xx-ta".into()];
        let map = parse(&input);

        let un = biggest_union(map);

        assert_eq!(un, "ch,ta,xx")
    }
}
