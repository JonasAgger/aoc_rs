use std::{collections::HashSet, ops::RangeInclusive};

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
        let res = input
            .into_iter()
            .flat_map(|s| s.split(','))
            .filter(|s| !s.is_empty())
            .map(|range| {
                let (start, end) = range.split_once('-').unwrap();
                (start.number::<usize, _>(), end.number::<usize, _>())
            })
            .flat_map(|(start, end)| {
                let range = start..=end;
                find(range)
            })
            .fold(0, |acc, number| acc + number);
        Ok(res.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let res = input
            .into_iter()
            .flat_map(|s| s.split(','))
            .filter(|s| !s.is_empty())
            .map(|range| {
                let (start, end) = range.split_once('-').unwrap();
                (start.number::<usize, _>(), end.number::<usize, _>())
            })
            .flat_map(|(start, end)| {
                let range = start..=end;
                find2(range)
            })
            .fold(0, |acc, number| acc + number);
        Ok(res.into())
    }
}

fn find(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut current = *range.start();
    let mut res = vec![];

    while let Some(nr) = find_next(current, &range) {
        res.push(nr);
        current = nr + 1;
    }

    res
}

fn find2(range: RangeInclusive<usize>) -> impl IntoIterator<Item = usize> {
    let start_len = range.start().ilog10() + 1;
    let end_len = range.end().ilog10() + 1;
    let set: HashSet<usize> = (2..=end_len)
        .flat_map(|rep| find_repeats(start_len, rep, &range))
        .collect();

    set.into_iter()
}

fn find_repeats(
    smalles_len: u32,
    rep_len: u32,
    range: &RangeInclusive<usize>,
) -> impl Iterator<Item = usize> + use<'_> {
    // find the
    let repeat_count = 10u64.pow((smalles_len / rep_len).max(1) - 1) as usize;
    // Generate possible combinations for repeat count :D
    (repeat_count..)
        .map(move |n| {
            let digits = n.checked_ilog10().unwrap_or(0) + 1;
            (0..rep_len)
                .map(|p| 10usize.pow(digits * p as u32) * n)
                .sum::<usize>()
        })
        .skip_while(move |n| n < range.start()) // only take stuff which fits
        .take_while(move |n| range.contains(n))
}

fn find_next(current: usize, range: &RangeInclusive<usize>) -> Option<usize> {
    if !range.contains(&current) {
        return None;
    }

    // if is even, then we can ignore.
    if current.ilog10() % 2 == 0 {
        // Find next number where we can get something
        let next_ilog10 = current.ilog10() + 1;
        let next_number = 10usize.pow(next_ilog10);
        if next_number > *range.end() {
            return None;
        }
        return find_next(next_number, range);
    }

    // find 10 exp
    let current_ilog = current.ilog10();

    let number_count_div2 = (current_ilog / 2) + 1;
    let exponent = 10usize.pow(number_count_div2);

    // construct duplicate of upper / lower half
    let duplicate = current / exponent;
    let number = duplicate * exponent + duplicate;

    if number < current {
        let next = (duplicate + 1) * exponent;
        return find_next(next, range);
    }

    match range.contains(&number) {
        true => Some(number),
        false => None,
    }
}

fn find_next2(current: usize, range: &RangeInclusive<usize>) -> Option<usize> {
    if !range.contains(&current) {
        return None;
    }

    // find 10 exp
    let current_ilog = current.ilog10();

    for i in current_ilog..0 {}

    let number_count_div2 = (current_ilog / 2) + 1;

    let exponent = 10usize.pow(number_count_div2);

    // construct duplicate of upper / lower half
    let duplicate = current / exponent;
    let number = duplicate * exponent + duplicate;

    if number < current {
        let next = (duplicate + 1) * exponent;
        return find_next(next, range);
    }

    match range.contains(&number) {
        true => Some(number),
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let range = 11..=22usize;

        let d = find(range);

        assert_eq!(d, vec![11, 22])
    }

    #[test]
    fn test3() {
        let range = 11..=22usize;

        let iter = find_next(11, &range);
        assert!(false)
    }

    #[test]
    fn test2() {
        let range = 95..=115usize;

        assert_eq!(find(range).into_iter().single(), 99)
    }

    #[test]
    fn test4() {
        let range = 95..=115usize;

        assert_eq!(find2(range).into_iter().collect::<Vec<_>>(), vec![111, 99])
    }
}
