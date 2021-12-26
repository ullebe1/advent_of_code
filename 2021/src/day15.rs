use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    rows: Vec<Vec<(usize, bool)>>,
    width: usize,
    height: usize,
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    let lines = input.lines();

    let rows: Vec<Vec<(usize, bool)>> = lines
        .map(|x| {
            x.chars()
                .map(|y| (String::from(y).parse::<usize>().unwrap(), false))
                .collect()
        })
        .collect();

    Input {
        width: rows[0].len(),
        height: rows.len(),
        rows: rows,
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> usize {
    let rows = input.rows.clone();

    solve_part1(rows, input.height, input.width)
}

pub fn calc_node_number(width: usize, x: usize, y: usize) -> usize {
    (y * width) + x
}

pub fn solve_part1(rows: Vec<Vec<(usize, bool)>>, height: usize, width: usize) -> usize {
    let mut graph: Vec<Vec<Edge>> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let mut node: Vec<Edge> = Vec::new();
            if y < height - 1 {
                node.push(Edge {
                    node: calc_node_number(width, x, y + 1),
                    cost: rows[y + 1][x].0,
                })
            }
            if y != 0 {
                node.push(Edge {
                    node: calc_node_number(width, x, y - 1),
                    cost: rows[y - 1][x].0,
                })
            }
            if x < width - 1 {
                node.push(Edge {
                    node: calc_node_number(width, x + 1, y),
                    cost: rows[y][x + 1].0,
                })
            }
            if x != 0 {
                node.push(Edge {
                    node: calc_node_number(width, x - 1, y),
                    cost: rows[y][x - 1].0,
                })
            }

            graph.push(node);
        }
    }
    shortest_path(&graph, 0, graph.len() - 1).unwrap()
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> usize {
    let rows = input.rows.clone();

    solve_part2(rows, input.height, input.width)
}

pub fn solve_part2(rows: Vec<Vec<(usize, bool)>>, height: usize, width: usize) -> usize {
    let mut new_rows: Vec<Vec<(usize, bool)>> = Vec::with_capacity(height * 5);
    for y_mult in 0..5 {
        for row in rows.clone() {
            let mut new_row: Vec<(usize, bool)> = Vec::with_capacity(width * 5);
            for x_mult in 0..5 {
                for item in &row {
                    new_row.push((((item.0 - 1 + (x_mult + y_mult)) % 9) + 1, false))
                }
            }
            new_rows.push(new_row)
        }
    }
    solve_part1(new_rows, height * 5, width * 5)
}

// From https://doc.rust-lang.org/std/collections/binary_heap/index.html
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}
