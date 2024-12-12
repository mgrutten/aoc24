use array2d::Array2D;
use petgraph::algo::kosaraju_scc;
use petgraph::{Graph, Undirected};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::ops::Add;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Location(i32, i32);

impl Add for Location {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Location(self.0 + other.0, self.1 + other.1)
    }
}


fn part1(map: &Array2D<char>) {
    // Undirected graph
    let mut graph = Graph::new_undirected();

    // Node for each element in the graph
    let node_map = Array2D::filled_by_row_major(
        || graph.add_node(()), map.num_rows(), map.num_columns());

    // Add edges
    for row in 1..map.num_rows() {
        if map[(row - 1, 0)] == map[(row, 0)] {
            graph.add_edge(node_map[(row - 1, 0)], node_map[(row, 0)], ());
        }
    }
    for col in 1..map.num_columns() {
        if map[(0, col - 1)] == map[(0, col)] {
            graph.add_edge(node_map[(0, col - 1)], node_map[(0, col)], ());
        }
    }
    for row in 1..map.num_rows() {
        for col in 1..map.num_columns() {
            if map[(row - 1, col)] == map[(row, col)] {
                graph.add_edge(node_map[(row - 1, col)], node_map[(row, col)], ());
            }
            if map[(row, col - 1)] == map[(row, col)] {
                graph.add_edge(node_map[(row, col - 1)], node_map[(row, col)], ());
            }
        }
    }

    // Find connected components
    let components = kosaraju_scc(&graph);

    // Add up the price
    let mut price = 0;
    for component in components {
        // Area is the number of components
        let area = component.len();

        // The perimeter of each plot is just 4 minus the number of edges
        let perimeter = component.iter()
            .map(|index| 4 - graph.edges(*index).count())
            .sum::<usize>();

        price += area * perimeter;
    }

    println!("Part 1: {}", price);
}


fn part2(map: &Array2D<char>) {
    // Undirected graph
    let mut graph: Graph<Location, (), Undirected> = Graph::new_undirected();

    // Node for each element in the graph
    let nodes = map.rows_iter().enumerate()
        .map(|(ri, row)| row.enumerate()
            .map(|(ci, _)| graph.add_node(Location(ri as i32, ci as i32)))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let node_map = Array2D::from_rows(&nodes).unwrap();

    // Add edges
    for row in 1..map.num_rows() {
        if map[(row - 1, 0)] == map[(row, 0)] {
            graph.add_edge(node_map[(row - 1, 0)], node_map[(row, 0)], ());
        }
    }
    for col in 1..map.num_columns() {
        if map[(0, col - 1)] == map[(0, col)] {
            graph.add_edge(node_map[(0, col - 1)], node_map[(0, col)], ());
        }
    }
    for row in 1..map.num_rows() {
        for col in 1..map.num_columns() {
            if map[(row - 1, col)] == map[(row, col)] {
                graph.add_edge(node_map[(row - 1, col)], node_map[(row, col)], ());
            }
            if map[(row, col - 1)] == map[(row, col)] {
                graph.add_edge(node_map[(row, col - 1)], node_map[(row, col)], ());
            }
        }
    }

    // Find connected components
    let components = kosaraju_scc(&graph);

    let corners = vec![
        (Location(-1, 0), Location(0, -1), Location(-1, -1)),
        (Location(-1, 0), Location(0, 1), Location(-1, 1)),
        (Location(1, 0), Location(0, -1), Location(1, -1)),
        (Location(1, 0), Location(0, 1), Location(1, 1)),
    ];

    // Add up the price
    let mut price = 0;
    for component in components {
        // Area is the number of components
        let area = component.len();

        // Grid locations in this component
        let locations = component.iter()
            .map(|index| *graph.node_weight(*index).unwrap())
            .collect::<HashSet<_>>();

        // Perimeter is the number of sides (= number of corners)
        let mut corner_count = 0;
        for location in locations.iter() {
            for corner in corners.iter() {
                // Outside corner
                if !locations.contains(&(*location + corner.0)) &&
                    !locations.contains(&(*location + corner.1)) {
                    corner_count += 1;
                }

                // Inside corner
                if locations.contains(&(*location + corner.0)) &&
                    locations.contains(&(*location + corner.1)) &&
                    !locations.contains(&(*location + corner.2)) {
                    corner_count += 1;
                }
            }
        }
        price += area * corner_count;
    }

    println!("Part 2: {}", price);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day12/day12.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let map = Array2D::from_rows(&map_vec).unwrap();

    part1(&map);
    part2(&map);

    Ok(())
}