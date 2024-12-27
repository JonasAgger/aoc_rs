use anyhow::Result;
use itertools::Itertools;

use crate::utils::*;

use super::super::AocDay;

const NUM_CHARS: usize = 11;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

//  there is a set of depth independent optimal moves => a LUT of the replacements can be built
//  since all moves are independent (i.e. their order does not matter) a state becomes a vector of moves
//  the replacement becomes a linear transform => all codes can be added beforehand and transformed as one
//  since the number of steps is fixed, we can precompute the matrix
//  the final count is just a dot with the vector of all 1s => precompute dot with matrix
//  => each part reduces to a single dot product

impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut counts = vec![0; NUM_CHARS * NUM_CHARS];

        for inp in input {
            let inp = inp.trim();
            let num_val = inp[..inp.len() - 1].parse::<u64>().unwrap();

            for (c, cn) in "A".chars().chain(inp.chars()).map(encode).tuple_windows() {
                counts[(c as usize) * NUM_CHARS + (cn as usize)] += num_val;
            }
        }
        Ok(counts
            .iter()
            .zip(LUT3.iter())
            .map(|(x, y)| x * y)
            .sum::<u64>()
            .into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let mut counts = vec![0; NUM_CHARS * NUM_CHARS];

        for inp in input {
            let inp = inp.trim();
            let num_val = inp[..inp.len() - 1].parse::<u64>().unwrap();

            for (c, cn) in "A".chars().chain(inp.chars()).map(encode).tuple_windows() {
                counts[(c as usize) * NUM_CHARS + (cn as usize)] += num_val;
            }
        }
        Ok(counts
            .iter()
            .zip(LUT26.iter())
            .map(|(x, y)| x * y)
            .sum::<u64>()
            .into())
    }
}

const LUT3: [u64; 121] = [
    1, 25, 12, 19, 26, 13, 20, 27, 14, 21, 10, 21, 1, 10, 11, 12, 19, 20, 13, 20, 21, 22, 16, 18,
    1, 10, 21, 12, 19, 22, 13, 20, 17, 21, 19, 18, 1, 22, 21, 12, 23, 22, 13, 16, 22, 16, 17, 18,
    1, 10, 11, 12, 19, 20, 23, 17, 21, 16, 17, 18, 1, 10, 21, 12, 19, 18, 22, 22, 21, 16, 19, 18,
    1, 22, 21, 12, 17, 23, 17, 18, 19, 16, 17, 18, 1, 10, 11, 24, 18, 22, 17, 18, 21, 16, 17, 18,
    1, 10, 19, 23, 23, 22, 17, 22, 21, 16, 19, 18, 1, 18, 18, 26, 21, 12, 27, 22, 13, 28, 23, 14,
    1,
];
const LUT26: [u64; 121] = [
    1,
    31420065369,
    14752615084,
    24095973437,
    31420065370,
    14752615085,
    24095973438,
    31420065371,
    14752615086,
    24095973439,
    14287938116,
    27052881363,
    1,
    14287938116,
    14287938117,
    14752615084,
    24095973437,
    24095973438,
    14752615085,
    24095973438,
    24095973439,
    27052881364,
    20790420654,
    22411052532,
    1,
    14287938116,
    28154654777,
    14752615084,
    24095973437,
    28154654778,
    14752615085,
    24095973438,
    22778092491,
    27622800565,
    22411052533,
    22411052532,
    1,
    28154654778,
    28154654777,
    14752615084,
    28154654779,
    28154654778,
    14752615085,
    20790420654,
    27052881364,
    20790420654,
    22778092491,
    22778092492,
    1,
    14287938116,
    14287938117,
    14752615084,
    24095973437,
    24095973438,
    27052881365,
    20790420655,
    27622800565,
    20790420654,
    22778092491,
    22411052532,
    1,
    14287938116,
    28154654777,
    14752615084,
    24095973437,
    22778092492,
    27622800566,
    27622800566,
    27622800565,
    20790420654,
    22411052533,
    22411052532,
    1,
    28154654778,
    28154654777,
    14752615084,
    20790420655,
    27052881365,
    20790420655,
    22778092492,
    22778092493,
    20790420654,
    22778092491,
    22778092492,
    1,
    14287938116,
    14287938117,
    27052881366,
    20790420656,
    27622800566,
    20790420655,
    22778092492,
    27622800565,
    20790420654,
    22778092491,
    22411052532,
    1,
    14287938116,
    22778092493,
    27622800567,
    27622800567,
    27622800566,
    20790420655,
    27622800566,
    27622800565,
    20790420654,
    22411052533,
    22411052532,
    1,
    20790420656,
    22411052532,
    31420065370,
    28154654777,
    14752615084,
    31420065371,
    28154654778,
    14752615085,
    31420065372,
    28154654779,
    14752615086,
    1,
];

fn encode(c: char) -> u8 {
    let o = c as u8;
    o % 16 + (o / 64) * 9
}

// struct NumPad {
//     values: Grid2D<NumpadValue>,
//     current: Point,
// }

// impl NumPad {
//     fn new() -> Self {
//         Self {
//             values: Grid2D::from_raw(
//                 vec![
//                     NumpadValue::Number(7),
//                     NumpadValue::Number(8),
//                     NumpadValue::Number(9),
//                     NumpadValue::Number(4),
//                     NumpadValue::Number(5),
//                     NumpadValue::Number(6),
//                     NumpadValue::Number(1),
//                     NumpadValue::Number(2),
//                     NumpadValue::Number(3),
//                     NumpadValue::None,
//                     NumpadValue::Number(0),
//                     NumpadValue::Activate,
//                 ],
//                 3,
//             ),
//             current: Point::new(2, 3),
//         }
//     }
// }

// struct DirectionPad {
//     values: Grid2D<DirectionPadValue>,
//     current: Point,
// }

// impl DirectionPad {
//     fn new() -> Self {
//         Self {
//             values: Grid2D::from_raw(
//                 vec![
//                     DirectionPadValue::None,
//                     DirectionPadValue::Vec(Vec2D::UP),
//                     DirectionPadValue::Activate,
//                     DirectionPadValue::Vec(Vec2D::LEFT),
//                     DirectionPadValue::Vec(Vec2D::DOWN),
//                     DirectionPadValue::Vec(Vec2D::RIGHT),
//                 ],
//                 3,
//             ),
//             current: Point::new(2, 1),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum NumpadValue {
//     Number(u8),
//     Activate,
//     None,
// }

// impl Display for NumpadValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum DirectionPadValue {
//     Vec(Vec2D),
//     Activate,
//     None,
// }

// impl Display for DirectionPadValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(self, f)
//     }
// }
