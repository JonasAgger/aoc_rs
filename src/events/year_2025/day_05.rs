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
        let (ranges, ids) = split_chunk_empty_once(input);
        let ranges: Vec<Range> = ranges
            .into_iter()
            .map(|r| {
                let (low, high) = r.split_once('-').unwrap();
                Range {
                    start: low.number(),
                    end: high.number(),
                }
            })
            .collect();

        let count = ids
            .into_iter()
            .map(|s| s.number())
            .filter(|&number| ranges.iter().any(|range| range.contains(number)))
            .count();

        Ok(count.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let (str_ranges, _) = split_chunk_empty_once(input);
        let ranges: Vec<Range> = str_ranges
            .iter()
            .map(|r| {
                let (low, high) = r.split_once('-').unwrap();
                Range {
                    start: low.number(),
                    end: high.number(),
                }
            })
            .collect();

        let ranges = consolidate(ranges);

        let count: usize = ranges.into_iter().map(|range| range.range()).sum();
        Ok(count.into())
    }
}

fn consolidate(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort();

    ranges.into_iter().fold(vec![], |mut items, curr| {
        // is empty just push to merged items, otherwise check if we overlap.
        if items.is_empty() || !items.last().unwrap().contains_range(&curr) {
            items.push(curr);
        } else {
            // if overlap, merge
            items.last_mut().unwrap().merge(&curr);
        }

        items
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn range(self) -> usize {
        (self.end - self.start) + 1 // +1 because inclusive
    }

    fn contains(&self, idx: usize) -> bool {
        let s = self.start;
        let e = self.end;
        idx >= s && idx <= e
    }

    fn contains_range(&self, other: &Range) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }

    fn merge(&mut self, other: &Range) {
        self.end = self.end.max(other.end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let r1 = Range { start: 10, end: 15 };
        let r2 = Range { start: 10, end: 17 };

        let mut ranges = vec![r2, r1];

        ranges.sort();

        assert_eq!(ranges, vec![r1, r2])
    }

    #[test]
    fn test2() {
        let r1 = Range { start: 8, end: 15 };
        let r2 = Range { start: 10, end: 15 };

        let mut ranges = vec![r2, r1];

        ranges.sort();

        assert_eq!(ranges, vec![r1, r2])
    }

    #[test]
    fn test3() {
        let r1 = Range { start: 8, end: 15 };
        let mut r2 = Range { start: 10, end: 17 };

        let res = Range { start: 8, end: 17 };

        r2.merge(&r1);

        assert_eq!(r2, res)
    }

    #[test]
    fn test4() {
        let r1 = Range { start: 8, end: 15 };
        let mut r2 = Range { start: 10, end: 15 };

        let res = Range { start: 8, end: 15 };

        r2.merge(&r1);

        assert_eq!(r2, res)
    }
}
