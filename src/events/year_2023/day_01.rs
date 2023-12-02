use anyhow::Result;
use fancy_regex::Regex;

use crate::utils::AoCResult;

use super::super::AocDay;


pub struct Day {
    regex: Regex,
}

impl Day {
    pub fn new() -> Self {
        Self {
            // Match with backtracking any of a digit and the named digit values
            regex: fancy_regex::RegexBuilder::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))")
            .build()
            .unwrap(),
        }
    }
    // had to add another regex lib to support backtracking
    // In hindsight, we probably should have just brute-force matched
    // This is not an efficient approach, but the most "readable" imo
    fn reduce_part2(&self, line: &String) -> Vec<String> {
        self.regex.captures_iter(&line)
            .map(|capture| {
                let regex_match = capture.unwrap().get(1).unwrap();
                regex_match.as_str().to_string()
            })
            .collect()
    }
}

impl AocDay for Day {
    fn run_part1(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let sum: i64 = input
        .into_iter()
        .map(|s| s.chars().filter(|c| char::is_ascii_digit(c)).collect::<Vec<_>>())
        .map(|digits| {
            let sum = match digits[..] {
                [first, .., last] => format!("{first}{last}").parse::<i64>(), // always can parse
                [single] => format!("{single}{single}").parse::<i64>(), // always can parse
                _ => Ok(0)
            };

            sum.expect(format!("Could not parse digits from vec: {:?}", digits).as_str())
        }).sum();

        Ok(sum.into())
    }


    // First time I've encountered a day 1 that was not trivial.
    fn run_part2(&mut self, input: &Vec<String>) -> Result<AoCResult> {
        let sum: i64 = input
        .into_iter()
        // reduce string to "parts"
        .map(|s| self.reduce_part2(s))
        // map parts to digits
        .map(|parts|  {
            parts
            .iter()
            .map(|item| {
                match item.as_str() {
                    "one" => 1i64,
                    "two" => 2i64,
                    "three" => 3i64,
                    "four" => 4i64,
                    "five" => 5i64,
                    "six" => 6i64,
                    "seven" => 7i64,
                    "eight" => 8i64,
                    "nine" => 9i64,
                    _ => item.parse::<i64>().unwrap()
                }
            }).collect::<Vec<_>>()
        })
        // map digits to string values
        .map(|digits| {
            match digits[..] {
                [first, .., last] => first * 10 + last, // always can parse
                [single] => single * 10 + single,
                _ => 0
            }
        }).sum();

        Ok(sum.into())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_parsing() {
        let input = String::from("7pqrstsixteen");
        let parts = Day::new().reduce_part2(&input);

        assert_eq!(parts[..], ["7", "six"]);
    }

    #[test]
    fn part2_parsing2() {
        let input = String::from("xtwone3four");
        let parts = Day::new().reduce_part2(&input);

        assert_eq!(parts[..], ["two", "one", "3", "four"]);
    }

    #[test]
    fn part2_parsing3() {
        let input = String::from("24");
        let parts = Day::new().reduce_part2(&input);

        assert_eq!(parts[..], ["2", "4"]);
    }

    #[test]
    fn sum() {
        let input = vec![
            String::from("24"),
        ];
        let res = Day::new().run_part2(&input).unwrap();

        assert_eq!(res, AoCResult::Int(24));
    }

    #[test]
    fn part2() {
        let input = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];
        let res = Day::new().run_part2(&input).unwrap();

        assert_eq!(res, AoCResult::Int(281));
    }
}