use anyhow::Result;

use crate::utils::*;

use super::super::AocDay;

const P1_OPERATORS: [Operator; 2] = [Operator::Add, Operator::Mul];
const P2_OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Concat];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
    Concat,
}

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let parts: Vec<(usize, Vec<usize>)> = input
            .into_iter()
            .map(|s| s.split_once(':').unwrap())
            .map(|(sum, numbers)| {
                (
                    sum.number(),
                    numbers
                        .split_ascii_whitespace()
                        .map(|s| s.trim().number())
                        .collect(),
                )
            })
            .collect();

        let sum: usize = parts
            .into_iter()
            .map(|(sum, members)| check_p1(sum, members))
            .sum();

        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let parts: Vec<(usize, Vec<usize>)> = input
            .into_iter()
            .map(|s| s.split_once(':').unwrap())
            .map(|(sum, numbers)| {
                (
                    sum.number(),
                    numbers
                        .split_ascii_whitespace()
                        .map(|s| s.trim().number())
                        .collect(),
                )
            })
            .collect();

        let sum: usize = parts
            .into_iter()
            .map(|(sum, members)| check_p2(sum, members))
            .sum();

        Ok(sum.into())
    }
}

fn check_p1(sum: usize, members: Vec<usize>) -> usize {
    for operators in permutations_p1(members.len() - 1) {
        if check(sum, &members, &operators) {
            return sum;
        }
    }
    0
}

fn check_p2(sum: usize, members: Vec<usize>) -> usize {
    for operators in permutations_p2(members.len() - 1) {
        if check(sum, &members, &operators) {
            return sum;
        }
    }
    0
}

fn check(sum: usize, members: &[usize], operators: &[Operator]) -> bool {
    let mut part_sum = 0;
    for (idx, operator) in operators.into_iter().enumerate() {
        let op1 = if idx == 0 { members[0] } else { part_sum };

        let op2 = members[idx + 1];
        part_sum = match operator {
            Operator::Add => op1 + op2,
            Operator::Mul => op1 * op2,
            Operator::Concat => format!("{}{}", op1, op2).number(),
        };

        if part_sum > sum {
            break;
        }
    }

    part_sum == sum
}

fn permutations_p1(size: usize) -> Vec<Vec<Operator>> {
    let mut ops = vec![];

    for i in 0..2_usize.pow(size as u32) {
        let mut perm = vec![];
        for idx in 0..size {
            let wat = match (i >> idx) & 1 {
                0 => Operator::Add,
                1 => Operator::Mul,
                other => unreachable!(),
            };

            perm.push(wat);
        }

        ops.push(perm);
    }

    ops
}

fn permutations_p2(size: usize) -> Vec<Vec<Operator>> {
    let mut ops = vec![];
    let mut combination = Vec::with_capacity(size);

    fn wat_recurse(size: usize, combination: &mut Vec<Operator>, ops: &mut Vec<Vec<Operator>>) {
        if combination.len() == size {
            ops.push(combination.clone());
            return;
        }

        for op in P2_OPERATORS {
            combination.push(op);
            wat_recurse(size, combination, ops);
            combination.pop();
        }
    }

    wat_recurse(size, &mut combination, &mut ops);
    ops
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    #[test]
    fn perms() {
        let perms = permutations_p1(3);
        assert_eq!(perms.len(), 8); // 2^n
        let perms: Vec<_> = perms.into_iter().unique().collect();
        assert_eq!(perms.len(), 8);
    }

    #[test]
    fn perms2() {
        let perms = permutations_p2(3);
        assert!(perms.contains(&vec![Operator::Mul, Operator::Concat, Operator::Mul]));
        assert_eq!(perms.len(), 27); // 3^n
        let perms: Vec<_> = perms.into_iter().unique().collect();
        assert_eq!(perms.len(), 27);
    }

    #[test]
    fn test() {
        assert_eq!(check_p1(190, vec![10, 19]), 190);
        assert_eq!(check_p1(3267, vec![81, 40, 27]), 3267);
        assert_eq!(check_p1(292, vec![11, 6, 16, 20]), 292);
        assert_eq!(check_p1(83, vec![17, 5]), 0);
    }

    #[test]
    fn test_p2() {
        assert_eq!(check_p2(156, vec![15, 6]), 156);
        assert_eq!(check_p2(7290, vec![6, 8, 6, 15]), 7290);
        assert_eq!(check_p2(192, vec![17, 8, 14]), 192);
    }
}
