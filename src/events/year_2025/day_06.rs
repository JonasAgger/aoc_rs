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
        let numbers: Vec<Vec<usize>> = input
            .iter()
            .take(input.len() - 1)
            .map(|s| s.split_ascii_whitespace().map(|s| s.number()).collect())
            .collect();
        let operators: Vec<_> = input
            .last()
            .map(|s| s.split_ascii_whitespace().collect())
            .unwrap();

        let result: usize = operators
            .into_iter()
            .enumerate()
            .map(|(idx, op)| {
                numbers.iter().fold(0, |acc, curr| {
                    let nr = curr[idx];
                    match op {
                        "*" if acc > 0 => nr * acc,
                        "*" if acc == 0 => nr,
                        "+" => nr + acc,
                        unknown => unreachable!("cant do math with {}", unknown),
                    }
                })
            })
            .sum();

        Ok(result.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let math = parse_ops(input);

        let res: usize = math
            .into_iter()
            .map(|math| {
                math.numbers.into_iter().fold(0, |acc, nr| match math.op {
                    '*' if acc == 0 => nr,
                    '*' => acc * nr,
                    '+' => acc + nr,
                    _ => unreachable!(),
                })
            })
            .sum();

        Ok(res.into())
    }
}

fn parse_ops(input: &[String]) -> Vec<Maf> {
    let last_line = input.last().unwrap().as_bytes();

    let last_idx = input.len() - 1;
    let mut idx = 0;

    let mut mafs = vec![];

    loop {
        let op = last_line[idx] as char;

        let number_of_spaces_to_next_operator = last_line[(idx + 1)..]
            .iter()
            .take_while(|x| x.is_ascii_whitespace())
            .count();

        let maf = Maf {
            op,
            numbers: parse_numbers(&input[..last_idx], idx, number_of_spaces_to_next_operator),
        };

        mafs.push(maf);

        idx += number_of_spaces_to_next_operator + 1;
        if idx >= last_line.len() {
            return mafs;
        }
    }
}

fn parse_numbers(input: &[String], start: usize, len: usize) -> Vec<usize> {
    let len = len + 1; // off by 1 error on the end op here, but this works because of the space between ops
    // We're basically parsing numbers from behind.
    // we're indexing from right to left, and then just iterating through the lines:
    // eg:
    // 123
    //  45
    //   6
    // *
    // idx: 3 here is
    // line1: 3
    // line2: 5
    // line3: 6
    //
    // then idx 2:
    // line1: 2
    // line2: 4
    // line3: _
    //
    // then we just to_string and parse the number for ease
    (0..len)
        .rev()
        .filter_map(|idx| {
            let str_nr: Vec<u8> = input
                .iter()
                .map(|s| s.as_bytes()[start + idx])
                .filter(|c| c.is_ascii_digit())
                .collect();
            if str_nr.is_empty() {
                None
            } else {
                Some(String::from_utf8(str_nr).unwrap().number())
            }
        })
        .collect()
}

#[derive(Debug)]
struct Maf {
    op: char,
    numbers: Vec<usize>,
}
