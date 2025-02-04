mod year_2018;
mod year_2019;
mod year_2023;
mod year_2024;


use anyhow::Result;

use crate::utils::AoCResult;

pub trait AocDay {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult>; 
    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult>; 
}

pub fn has_day(day: u8, year: u16) -> bool {
    get_day(day, year).is_ok()
}

pub fn get_day(day: u8, year: u16) -> Result<Box<dyn AocDay>> {
    match (day, year) {
        (1, 2018) => Ok(Box::new(year_2018::day_01::Day::new())),
(2, 2018) => Ok(Box::new(year_2018::day_02::Day::new())),
(3, 2018) => Ok(Box::new(year_2018::day_03::Day::new())),
(4, 2018) => Ok(Box::new(year_2018::day_04::Day::new())),
(5, 2018) => Ok(Box::new(year_2018::day_05::Day::new())),
(6, 2018) => Ok(Box::new(year_2018::day_06::Day::new())),
(2, 2019) => Ok(Box::new(year_2019::day_02::Day::new())),
(5, 2019) => Ok(Box::new(year_2019::day_05::Day::new())),
(7, 2019) => Ok(Box::new(year_2019::day_07::Day::new())),
(9, 2019) => Ok(Box::new(year_2019::day_09::Day::new())),
(1, 2023) => Ok(Box::new(year_2023::day_01::Day::new())),
(2, 2023) => Ok(Box::new(year_2023::day_02::Day::new())),
(3, 2023) => Ok(Box::new(year_2023::day_03::Day::new())),
(4, 2023) => Ok(Box::new(year_2023::day_04::Day::new())),
(5, 2023) => Ok(Box::new(year_2023::day_05::Day::new())),
(6, 2023) => Ok(Box::new(year_2023::day_06::Day::new())),
(7, 2023) => Ok(Box::new(year_2023::day_07::Day::new())),
(8, 2023) => Ok(Box::new(year_2023::day_08::Day::new())),
(9, 2023) => Ok(Box::new(year_2023::day_09::Day::new())),
(10, 2023) => Ok(Box::new(year_2023::day_10::Day::new())),
(11, 2023) => Ok(Box::new(year_2023::day_11::Day::new())),
(12, 2023) => Ok(Box::new(year_2023::day_12::Day::new())),
(13, 2023) => Ok(Box::new(year_2023::day_13::Day::new())),
(14, 2023) => Ok(Box::new(year_2023::day_14::Day::new())),
(15, 2023) => Ok(Box::new(year_2023::day_15::Day::new())),
(16, 2023) => Ok(Box::new(year_2023::day_16::Day::new())),
(17, 2023) => Ok(Box::new(year_2023::day_17::Day::new())),
(18, 2023) => Ok(Box::new(year_2023::day_18::Day::new())),
(19, 2023) => Ok(Box::new(year_2023::day_19::Day::new())),
(20, 2023) => Ok(Box::new(year_2023::day_20::Day::new())),
(1, 2024) => Ok(Box::new(year_2024::day_01::Day::new())),
(2, 2024) => Ok(Box::new(year_2024::day_02::Day::new())),
(3, 2024) => Ok(Box::new(year_2024::day_03::Day::new())),
(5, 2024) => Ok(Box::new(year_2024::day_05::Day::new())),
(7, 2024) => Ok(Box::new(year_2024::day_07::Day::new())),
(9, 2024) => Ok(Box::new(year_2024::day_09::Day::new())),
(11, 2024) => Ok(Box::new(year_2024::day_11::Day::new())),
(12, 2024) => Ok(Box::new(year_2024::day_12::Day::new())),
(13, 2024) => Ok(Box::new(year_2024::day_13::Day::new())),
(15, 2024) => Ok(Box::new(year_2024::day_15::Day::new())),
(17, 2024) => Ok(Box::new(year_2024::day_17::Day::new())),
(19, 2024) => Ok(Box::new(year_2024::day_19::Day::new())),
(21, 2024) => Ok(Box::new(year_2024::day_21::Day::new())),
(23, 2024) => Ok(Box::new(year_2024::day_23::Day::new())),
(25, 2024) => Ok(Box::new(year_2024::day_25::Day::new())),

        _ => anyhow::bail!("Received invalid day: {}", day) 
    }
} 