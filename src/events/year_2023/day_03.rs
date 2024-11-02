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
        let mut valid_parts: Vec<usize> = vec![];

        let lines: Vec<Vec<_>> = input.iter().map(|s| s.chars().collect()).collect();
        let mut parse_buffer = String::new();
        let mut start_parse_index = 0;
        let mut is_parsing = false;

        for (y_index, chars) in lines.iter().enumerate() {
            for (x_index, char) in chars.iter().enumerate() {
                // if digit, add to buffer
                if char::is_ascii_digit(char) {
                    if !is_parsing {
                        is_parsing = true;
                        start_parse_index = x_index;
                    }
                    parse_buffer.push(*char);
                }
                // stop parsing if we're already parsing a number
                else if is_parsing {
                    is_parsing = false;
                    let mut x_range = start_parse_index..x_index;
                    if x_range.any(|x| {
                        has_cmp_neighbour((x, y_index), &lines, |c| {
                            c.is_ascii_punctuation() && !c.eq(&'.')
                        })
                    }) {
                        valid_parts.push(parse_buffer.parse()?);
                    }
                    parse_buffer.clear();
                }
            }
            // case where we reach end of line
            if is_parsing {
                is_parsing = false;
                let mut x_range = start_parse_index..lines[0].len();
                if x_range.any(|x| {
                    has_cmp_neighbour((x, y_index), &lines, |c| {
                        c.is_ascii_punctuation() && !c.eq(&'.')
                    })
                }) {
                    valid_parts.push(parse_buffer.parse()?);
                }
                parse_buffer.clear();
            }
        }

        Ok(valid_parts.iter().sum::<usize>().into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut gear_ratios = vec![];

        let lines: Vec<Vec<_>> = input.iter().map(|s| s.chars().collect()).collect();
        let parts = parse_parts(&lines);

        for (y_index, chars) in lines.iter().enumerate() {
            for (x_index, char) in chars.iter().enumerate() {
                if char.eq(&'*') {
                    let neighbours =
                        get_neighbours_cmp(Point::new(x_index, y_index), &lines, |c| {
                            c.is_ascii_digit()
                        });

                    let neighbouring_parts: Vec<_> = parts
                        .iter()
                        .filter(|part| neighbours.iter().any(|nb| part.is_next_to(nb)))
                        .collect();

                    // gears needs exactly 2 parts as neighbours
                    if neighbouring_parts.len() == 2 {
                        let gear_ratio = neighbouring_parts[0]
                            .part_nr()
                            .checked_mul(neighbouring_parts[1].part_nr());
                        gear_ratios.push(gear_ratio.unwrap());
                    }
                }
            }
        }

        Ok(gear_ratios.iter().sum::<usize>().into())
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
struct Part {
    pub y: usize,
    pub x: usize,
    pub chars: String,
}

impl std::fmt::Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Part")
            .field("y", &self.y)
            .field("x", &self.x)
            .field("chars", &self.part_nr())
            .finish()
    }
}

impl Part {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            chars: String::new(),
        }
    }

    pub fn is_next_to(&self, p: &Point) -> bool {
        match self.chars.len() {
            1 => p.is_neighbour_diag(&Point::new(self.x, self.y)),
            len => {
                let x_start = self.x - (len - 1);
                for x in x_start..self.x {
                    if p.is_neighbour_diag(&Point::new(x, self.y)) {
                        return true;
                    }
                }
                false
            }
        }
    }

    pub fn part_nr(&self) -> usize {
        self.chars.parse::<usize>().unwrap()
    }
}

fn parse_parts(lines: &Vec<Vec<char>>) -> Vec<Part> {
    let mut parts = vec![];
    let mut part = Part::new();

    for (y_index, chars) in lines.iter().enumerate() {
        for (x_index, char) in chars.iter().enumerate() {
            // if digit, add to buffer
            if char::is_ascii_digit(char) {
                part.y = y_index;
                part.x = x_index;
                part.chars.push(*char);
            } else if !part.chars.is_empty() {
                parts.push(part);
                part = Part::new();
            }
        }
        if !part.chars.is_empty() {
            parts.push(part);
            part = Part::new();
        }
    }

    parts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_parsing() {
        let input = vec![
            String::from(".....4....."),
            String::from(".....*....."),
            String::from("......19..."),
        ];

        let res = Day::new().run_part2(&input).unwrap();

        assert_eq!(res, AoCResult::USize(76));
    }
}
