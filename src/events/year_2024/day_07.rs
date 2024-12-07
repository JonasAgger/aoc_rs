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
            .filter_map(|(sum, members)| {
                if check(sum, &members, &P1_OPERATORS) {
                    Some(sum)
                } else {
                    None
                }
            })
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
            .filter_map(|(sum, members)| {
                if check(sum, &members, &P2_OPERATORS) {
                    Some(sum)
                } else {
                    None
                }
            })
            .sum();

        Ok(sum.into())
    }
}

fn check(sum: usize, members: &[usize], operators: &[Operator]) -> bool {
    check_rec(sum, members, operators, 0, 0)
}

fn check_rec(
    sum: usize,
    members: &[usize],
    operators: &[Operator],
    idx: usize,
    current_sum: usize,
) -> bool {
    // Just check exits. if we are too big, just exit early. if we reach len, check equality
    if current_sum > sum || idx == (members.len() - 1) {
        return sum == current_sum;
    }

    for operator in operators {
        let op1 = if idx == 0 { members[0] } else { current_sum };

        let op2 = members[idx + 1];

        let op_result = match operator {
            Operator::Add => op1 + op2,
            Operator::Mul => op1 * op2,
            Operator::Concat => {
                let x = op2.ilog10();
                let y = 10usize.pow(x + 1);
                op1 * y + op2
            }
        };

        if check_rec(sum, members, operators, idx + 1, op_result) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(check(190, &[10, 19], &P1_OPERATORS));
        assert!(check(3267, &[81, 40, 27], &P1_OPERATORS));
        assert!(check(292, &[11, 6, 16, 20], &P1_OPERATORS));
    }

    #[test]
    fn test_p2() {
        assert!(check(156, &[15, 6], &P2_OPERATORS));
        assert!(check(7290, &[6, 8, 6, 15], &P2_OPERATORS));
        assert!(check(192, &[17, 8, 14], &P2_OPERATORS));
    }
}
