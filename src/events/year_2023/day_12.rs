use anyhow::Result;

use crate::utils::*;

use super::super::AocDay;


pub struct Day {

}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Copy)]
enum ConditionRecord {
    Unknown,
    Damaged,
    Operational
}

impl ConditionRecord {
    pub fn parse(c: char) -> Self {
        match c {
            '?' => ConditionRecord::Unknown,
            '#' => ConditionRecord::Damaged,
            '.' => ConditionRecord::Operational,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct ConditionReport {
    records: Vec<ConditionRecord>,
    damaged_groups: Vec<usize>
}

impl ConditionReport {
    pub fn parse(s: &String) -> Self {
        let (p1, p2) = s.split_once(" ").unwrap();

        let records = p1.chars().map(|c| ConditionRecord::parse(c)).collect();
        let damaged_groups = p2.split(',').filter_map(|s| s.parse().ok()).collect();

        Self { records, damaged_groups }
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {

        let reports: Vec<_> = input.iter().map(|line| ConditionReport::parse(line)).collect();

        

        Ok(AoCResult::None)
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        Ok(AoCResult::None)
    }
}