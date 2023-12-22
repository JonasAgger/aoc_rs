use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use priority_queue::PriorityQueue;
use tracing::{debug, trace};

use crate::utils::{grid::Grid2D, vec2d::Vec2D, *};

use super::super::AocDay;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_vec2d(vec: &Vec2D) -> Self {
        match (vec.x(), vec.y()) {
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Down,
            (0, -1) => Direction::Up,
            _ => panic!("wat"),
        }
    }

    fn inverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    position: Point,
    direction: Direction,
    direction_count: usize,
}

impl Node {
    fn new(position: Point, direction: Direction, direction_count: usize) -> Self {
        Self {
            position,
            direction,
            direction_count,
        }
    }
}

// The state we'll track in our priority queue. We need to track the
// node above and the cost to get there.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We are using a min heap, so we are doing this backwards.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl AocDay for Day {
    fn run_part1(&mut self, input: &[String]) -> Result<AoCResult> {
        let grid = Grid2D::parse_char(input, |c| c.to_digit(10).unwrap());

        let start = Point::new(0, 0);
        let end = Point::new(grid.width() - 1, grid.height() - 1);
        let end_condition = |node: &Node| node.position == end;

        let path_cost = dijkstra::<1, 3, _>(&grid, &start, end_condition).unwrap_or_default();

        Ok(path_cost.into())
    }

    fn run_part2(&mut self, input: &[String]) -> Result<AoCResult> {
        let grid = Grid2D::parse_char(input, |c| c.to_digit(10).unwrap());

        let start = Point::new(0, 0);
        let end = Point::new(grid.width() - 1, grid.height() - 1);
        let end_condition = |node: &Node| node.position == end && node.direction_count >= 4;

        let path_cost = dijkstra::<4, 10, _>(&grid, &start, end_condition).unwrap_or_default();

        Ok(path_cost.into())
    }
}

fn neighbors<const MIN: usize, const MAX: usize>(node: &Node, grid: &Grid2D<u32>) -> Vec<Node> {
    let mut neighbors = Vec::new();
    // Get the possible next valid points.

    for point in Utils::get_neighbours_straight(node.position, &grid) {
        let direction = Direction::from_vec2d(&point.get_diff(node.position));

        // Cant go straight back
        if direction == node.direction.inverse() {
            continue;
        } else if direction != node.direction && node.direction_count >= MIN {
            // New direction, reset counter. We can only change direction if we have already headed
            // in the same direction a minimum of MIN times (1 in part1 and 4 in part2)
            neighbors.push(Node::new(point, direction, 1));
        } else if direction == node.direction && node.direction_count < MAX {
            // We can still keep going this direction!
            // But a maximum of MAX times (3 in part1 and 10 in part2)
            neighbors.push(Node::new(point, direction, node.direction_count + 1));
        }
    }
    neighbors
}

fn dijkstra<const MIN: usize, const MAX: usize, G>(
    grid: &Grid2D<u32>,
    start: &Point,
    goal_fn: G,
) -> Option<usize>
where
    G: Fn(&Node) -> bool,
{
    // Track our min distances at each Node. In our specific case, we
    // have multiple because we could be coming from Up or Left
    // at the start.
    let mut distances = HashMap::new();
    distances.insert(Node::new(start.clone(), Direction::Down, 0), 0);
    distances.insert(Node::new(start.clone(), Direction::Right, 0), 0);

    // Track paths we want to visit. Again, we are adding two because
    // we could be coming from either.
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        node: Node::new(start.clone(), Direction::Down, 0),
    });
    frontier.push(State {
        cost: 0,
        node: Node::new(start.clone(), Direction::Right, 0),
    });

    // Grab the next node from the frontier.
    while let Some(State { cost, node }) = frontier.pop() {
        // If we are at the goal, we are done.
        if goal_fn(&node) {
            return Some(cost);
        }

        // Otherwise, check our neighbors.
        for neighbor in neighbors::<MIN, MAX>(&node, grid) {
            // If we've already visited this node and it was cheaper,
            // we don't need to keep checking this way.
            let new_cost = cost + *grid.get(neighbor.position).unwrap() as usize;

            if let Some(&best) = distances.get(&neighbor) {
                if new_cost >= best {
                    continue;
                }
            }

            // Otherwise, add it to our distances and frontier.
            distances.insert(neighbor.clone(), new_cost);
            frontier.push(State {
                cost: new_cost,
                node: neighbor,
            });
        }
    }

    // If we get here, we didn't find a path. Not possible in our
    // case, but is in the general case.
    None
}
