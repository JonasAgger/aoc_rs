use std::fmt::Display;

use anyhow::Result;
use vec2d::Vec2D;

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
        let [grid, movements] = slice_utils::split_chunk_empty(input).try_into().unwrap();

        let mut grid = grid::Grid2D::parse(&grid, GridElement::parse);
        let movements: Vec<_> = movements
            .into_iter()
            .flat_map(|s| {
                s.as_bytes()
                    .into_iter()
                    .map(|&c| to_vector(c))
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut player_position = grid.find(|&s| s == GridElement::Player).unwrap();

        for movement in movements {
            if moved(&mut grid, player_position, movement) {
                player_position = player_position + movement;
            }
        }

        let sum: usize = grid
            .find_all(|&s| s == GridElement::Box)
            .into_iter()
            .map(|p| p.x() + 100 * p.y())
            .sum();
        Ok(sum.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let [grid, movements] = slice_utils::split_chunk_empty(input).try_into().unwrap();

        let mut grid = grid::Grid2D::parse(&grid, GridElement::parse2);
        let movements: Vec<_> = movements
            .into_iter()
            .flat_map(|s| {
                s.as_bytes()
                    .into_iter()
                    .map(|&c| to_vector(c))
                    .collect::<Vec<_>>()
            })
            .collect();

        let mut player_position = grid.find(|&s| s == GridElement::Player).unwrap();

        for movement in movements {
            println!("{}", grid);
            if moved(&mut grid, player_position, movement) {
                player_position = player_position + movement;
            }
        }

        println!("{}", grid);

        let sum: usize = grid
            .find_all(|&s| s == GridElement::BoxP2L)
            .into_iter()
            .map(|p| p.x() + 100 * p.y())
            .sum();
        Ok(sum.into())
    }
}

fn moved(
    grid: &mut grid::Grid2D<GridElement>,
    position: Point,
    movement: Vec2D,
    moves: &mut Vec<Move>,
) -> bool {
    let destination = position + movement;

    let moved = match grid.get(destination) {
        Some(&destination_element) => match destination_element {
            GridElement::Wall => false,
            GridElement::Box => moved(grid, destination, movement),
            GridElement::Player => panic!("Went into player?"),
            GridElement::Empty => true,
            // p2
            GridElement::BoxP2L => {
                moved(grid, destination + Vec2D::RIGHT, movement, moves)
                    && moved(grid, destination, movement)
            }
            GridElement::BoxP2R => {
                moved(grid, destination + Vec2D::LEFT, movement)
                    && moved(grid, destination, movement)
            }
        },
        None => panic!("Went OOB"),
    };

    if moved {
        grid.swap(position, destination);
    }

    moved
}

struct Move {
    src: Point,
    dest: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridElement {
    Wall,
    Box,
    BoxP2L,
    BoxP2R,
    Player,
    Empty,
}

impl GridElement {
    fn parse(s: &String) -> Vec<Self> {
        s.as_bytes()
            .into_iter()
            .map(|c| match c {
                b'#' => Self::Wall,
                b'.' => Self::Empty,
                b'O' => Self::Box,
                b'@' => Self::Player,
                _ => unreachable!(),
            })
            .collect()
    }

    fn parse2(s: &String) -> Vec<Self> {
        s.as_bytes()
            .into_iter()
            .flat_map(|c| match c {
                b'#' => [Self::Wall, Self::Wall],
                b'.' => [Self::Empty, Self::Empty],
                b'O' => [Self::BoxP2L, Self::BoxP2R],
                b'@' => [Self::Player, Self::Empty],
                _ => unreachable!(),
            })
            .collect()
    }
}

impl Display for GridElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridElement::Wall => write!(f, "#"),
            GridElement::Box => write!(f, "O"),
            GridElement::Player => write!(f, "@"),
            GridElement::Empty => write!(f, "."),
            GridElement::BoxP2L => write!(f, "["),
            GridElement::BoxP2R => write!(f, "]"),
        }
    }
}

fn to_vector(c: u8) -> Vec2D {
    match c {
        b'<' => Vec2D::new(-1, 0),
        b'>' => Vec2D::new(1, 0),
        b'^' => Vec2D::new(0, -1),
        b'v' => Vec2D::new(0, 1),
        _ => unreachable!(),
    }
}

/*
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
*/
