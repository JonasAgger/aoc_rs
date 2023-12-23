use std::collections::HashMap;

use anyhow::Result;
use fancy_regex::RegexBuilder;

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
        let input = slice_utils::split_chunk_empty(input);

        let workflows: HashMap<_, _> = input[0]
            .iter()
            .map(|line| Workflow::parse(line.as_str()))
            .map(|wf| (wf.name.clone(), wf))
            .collect();

        let items: Vec<_> = input[1]
            .iter()
            .map(|item| parse_item(item.as_str()))
            .collect();

        let total: usize = items.iter().map(|item| get_value(&workflows, item)).sum();

        Ok(total.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        Ok(AoCResult::None)
    }
}

fn get_value(workflows: &HashMap<String, Workflow>, item: &[usize]) -> usize {
    let mut destination = &RuleDestination::Workflow("in".into());

    loop {
        match destination {
            RuleDestination::Workflow(next) => {
                destination = workflows.get(next).unwrap().check(&item)
            }
            RuleDestination::Accepted => return item.iter().sum::<usize>(),
            RuleDestination::Rejected => return 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum RuleDestination {
    Workflow(String),
    Accepted,
    Rejected,
}

#[derive(Debug, Clone)]
enum Condition {
    None,
    Greater(usize, usize),
    Less(usize, usize),
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    destination: RuleDestination,
}

impl Rule {
    fn parse(rule_str: &str) -> Self {
        let parts: Vec<_> = rule_str.split(&['<', '>', ':']).collect();

        match parts.as_slice() {
            ["A"] => Self {
                condition: Condition::None,
                destination: RuleDestination::Accepted,
            },
            ["R"] => Self {
                condition: Condition::None,
                destination: RuleDestination::Rejected,
            },
            [destination_workload] => Self {
                condition: Condition::None,
                destination: RuleDestination::Workflow(destination_workload.to_string()),
            },
            [index, value, destination] => {
                let index = match *index {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => unreachable!(),
                };
                let value = value.parse().unwrap();

                let condition = match rule_str.as_bytes()[1] {
                    b'>' => Condition::Greater(index, value),
                    b'<' => Condition::Less(index, value),
                    _ => unreachable!(),
                };

                let destination = match *destination {
                    "A" => RuleDestination::Accepted,
                    "R" => RuleDestination::Rejected,
                    val => RuleDestination::Workflow(val.to_string()),
                };

                Self {
                    condition,
                    destination,
                }
            }
            _ => panic!("wat"),
        }
    }

    fn check(&self, part: &[usize]) -> Option<&RuleDestination> {
        match self.condition {
            Condition::None => Some(&self.destination),
            Condition::Greater(index, value) => match part[index] > value {
                true => Some(&self.destination),
                false => None,
            },
            Condition::Less(index, value) => match part[index] < value {
                true => Some(&self.destination),
                false => None,
            },
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Self {
        let (name, conditions) = line.split_once('{').unwrap();
        Self {
            name: name.into(),
            rules: conditions
                .trim_end_matches('}')
                .split(',')
                .map(Rule::parse)
                .collect(),
        }
    }

    fn check(&self, part: &[usize]) -> &RuleDestination {
        self.rules
            .iter()
            .filter_map(|rule| rule.check(&part))
            .next()
            .unwrap()
    }
}
// {x=787,m=2655,a=1222,s=2876}

fn parse_item(line: &str) -> [usize; 4] {
    let line = line.trim_matches(&['{', '}']);

    let mut array = [0; 4];

    for (index, key_value) in line.split(',').enumerate() {
        let value = key_value.split_once('=').unwrap().1.parse().unwrap();
        array[index] = value;
    }

    array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_item() {
        let input = "{x=787,m=2655,a=1222,s=2876}";

        let item = parse_item(input);

        assert_eq!(item, [787, 2655, 1222, 2876])
    }

    #[test]
    fn test_workflow() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let input_item = "{x=787,m=2655,a=1222,s=2876}";

        let workflow = Workflow::parse(input);
        let item = parse_item(input_item);

        let destination = workflow.check(&item);

        assert_eq!(destination, &RuleDestination::Workflow("qkq".into()))
    }

    #[test]
    fn test_workflow2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let input_item = "{x=2127,m=1623,a=2188,s=1013}";

        let workflow = Workflow::parse(input);
        let item = parse_item(input_item);

        let destination = workflow.check(&item);

        assert_eq!(destination, &RuleDestination::Workflow("rfg".into()))
    }

    #[test]
    fn test_full_success() {
        let mut workflows = HashMap::new();

        workflows.insert("in".into(), Workflow::parse("in{next}"));
        workflows.insert("next".into(), Workflow::parse("next{s<537:gd,x>2440:R,A}"));

        let item = parse_item("{x=2127,m=1623,a=2188,s=1013}");

        let value = get_value(&workflows, &item);

        assert_eq!(value, 6951)
    }

    #[test]
    fn test_full_fail() {
        let mut workflows = HashMap::new();

        workflows.insert("in".into(), Workflow::parse("in{next}"));
        workflows.insert("next".into(), Workflow::parse("next{x>2662:A,R}"));

        let item = parse_item("{x=2461,m=1339,a=466,s=291}");

        let value = get_value(&workflows, &item);

        assert_eq!(value, 0)
    }
}
