use std::{collections::HashSet, fmt::Display};

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
            let mut moves = vec![];
            if moved(&grid, player_position, movement, &mut moves) {
                apply(&mut grid, &moves);
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
            let mut moves = vec![];
            if moved(&grid, player_position, movement, &mut moves) {
                apply(&mut grid, &moves);
                player_position = player_position + movement;
            }
        }

        let sum: usize = grid
            .find_all(|&s| s == GridElement::BoxP2L)
            .into_iter()
            .map(|p| p.x() + 100 * p.y())
            .sum();
        Ok(sum.into())
    }
}

fn moved(
    grid: &grid::Grid2D<GridElement>,
    position: Point,
    movement: Vec2D,
    moves: &mut Vec<Move>,
) -> bool {
    let destination = position + movement;

    let moved = match grid.get(destination) {
        Some(&destination_element) => match destination_element {
            GridElement::Wall => false,
            GridElement::Box => moved(grid, destination, movement, moves),
            GridElement::Player => panic!("Went into player?"),
            GridElement::Empty => true,
            // p2
            GridElement::BoxP2L => {
                //  dont fan out if we're moving boxex straight up
                let origin = *grid.get(position).unwrap();
                if movement.y() != 0 && origin != GridElement::BoxP2L {
                    moved(grid, destination + Vec2D::RIGHT, movement, moves)
                        && moved(grid, destination, movement, moves)
                } else {
                    moved(grid, destination, movement, moves)
                }
            }
            GridElement::BoxP2R => {
                //  dont fan out if we're moving boxex straight up
                let origin = *grid.get(position).unwrap();
                if movement.y() != 0 && origin != GridElement::BoxP2R {
                    moved(grid, destination + Vec2D::LEFT, movement, moves)
                        && moved(grid, destination, movement, moves)
                } else {
                    moved(grid, destination, movement, moves)
                }
            }
        },
        None => panic!("Went OOB"),
    };

    if moved {
        moves.push(Move {
            src: position,
            dest: destination,
        });
    }

    moved
}

fn apply(grid: &mut grid::Grid2D<GridElement>, moves: &[Move]) {
    let mut filter = HashSet::new(); // Dont apply double moves
    for Move { src, dest } in moves.into_iter().filter(|&m| filter.insert(m.clone())) {
        grid.swap(*src, *dest);
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_up_p2() {
        let s = r"
########
#......#
#......#
#...O..#
#.#.O..#
#...@O.#
#...O..#
########";
        let lines: Vec<String> = s.lines().into_iter().map(|s| s.to_owned()).collect();

        let mut grid = grid::Grid2D::parse(&lines, GridElement::parse2);

        let player_position = grid.find(|&s| s == GridElement::Player).unwrap();

        let mut moves = vec![];
        let did_move = moved(&grid, player_position, Vec2D::UP, &mut moves);

        assert!(did_move);

        apply(&mut grid, &moves);
        println!("{grid}");
        assert_eq!(
            grid.to_string(),
            r"
################
##............##
##......[]....##
##......[]....##
##..##..@.....##
##........[]..##
##......[]....##
################
"
        );
    }

    #[test]
    fn move_up_p2_2() {
        let rows = vec![
            pp(".....[]."),
            pp("...[].##"),
            pp("..[][].."),
            pp("..[][].."),
            pp("...[][]."),
            pp("[][]@..."),
        ];
        let mut grid = grid::Grid2D::build(8, 6, |col, row| rows[row][col]);

        let player_position = grid.find(|&s| s == GridElement::Player).unwrap();

        let mut moves = vec![];
        let did_move = moved(&grid, player_position, Vec2D::UP, &mut moves);

        assert!(did_move);

        apply(&mut grid, &moves);
    }

    fn pp(s: &str) -> Vec<GridElement> {
        s.as_bytes()
            .iter()
            .map(|&b| match b {
                b'.' => GridElement::Empty,
                b'#' => GridElement::Wall,
                b'[' => GridElement::BoxP2L,
                b']' => GridElement::BoxP2R,
                b'@' => GridElement::Player,
                _ => unreachable!(),
            })
            .collect()
    }
}

/*
####[]..........##[]..........[]........[]...[].[].........[]......[].[].....[][].............[]..##
##............##..[]##..[]......##........[][]..[]##[]........####[]..##...[].##....[][]##........##
##[][][]......[]....[][]..................[]..##....[]......[]##......[]..[][]......[]##[]......[]##
##......[][]....[]....[]##......[]....[]..##[]......[]##..................[][]......[]........[]..##
##[][]..[]..[][]....[][]..##........[]........[][][]........[][]......[]...[][].....[]..[]......[]##
##....[]..[]..[]......[][]..##....[][]....##[].............[].##......[][][]@...[]......[]##[][]..##
*/
