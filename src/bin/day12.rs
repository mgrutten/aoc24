use array2d::Array2D;
use petgraph::algo::kosaraju_scc;
use petgraph::{Graph, Undirected};
use std::error::Error;
use std::fs;


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
    let mut graph: Graph<(usize, usize), (), Undirected> = Graph::new_undirected();

    // Node for each element in the graph
    let nodes = map.rows_iter().enumerate()
        .map(|(ri, row)| row.enumerate()
            .map(|(ci, _)| graph.add_node((ri, ci)))
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

    // Add up the price
    let mut price = 0;
    for component in components {
        // Area is the number of components
        let area = component.len();

        // Perimeter is the number of sides (= number of corners)
        let val = map[*graph.node_weight(component[0]).unwrap()];
        let mut corner_count = 0;
        for index in component {
            let location = graph.node_weight(index).unwrap();

            // Outside corners
            if (location.0 == 0 || map[(location.0 - 1, location.1)] != val) &&
                (location.1 == 0 || map[(location.0, location.1 - 1)] != val) {
                corner_count += 1;
            }

            if (location.0 == 0 || map[(location.0 - 1, location.1)] != val) &&
                (location.1 == map.num_columns() - 1 || map[(location.0, location.1 + 1)] != val) {
                corner_count += 1;
            }

            if (location.0 == map.num_rows() - 1 || map[(location.0 + 1, location.1)] != val) &&
                (location.1 == 0 || map[(location.0, location.1 - 1)] != val) {
                corner_count += 1;
            }

            if (location.0 == map.num_rows() - 1 || map[(location.0 + 1, location.1)] != val) &&
                (location.1 == map.num_columns() - 1 || map[(location.0, location.1 + 1)] != val) {
                corner_count += 1;
            }

            // Inside corners
            if location.0 > 0 && map[(location.0 - 1, location.1)] == val &&
                location.1 > 0 && map[(location.0, location.1 - 1)] == val &&
                map[(location.0 - 1, location.1 - 1)] != val {
                corner_count += 1;
            }

            if location.0 > 0 && map[(location.0 - 1, location.1)] == val &&
                location.1 < map.num_columns() - 1 && map[(location.0, location.1 + 1)] == val &&
                map[(location.0 - 1, location.1 + 1)] != val {
                corner_count += 1;
            }

            if location.0 < map.num_rows() - 1 && map[(location.0 + 1, location.1)] == val &&
                location.1 > 0 && map[(location.0, location.1 - 1)] == val &&
                map[(location.0 + 1, location.1 - 1)] != val {
                corner_count += 1;
            }

            if location.0 < map.num_rows() - 1 && map[(location.0 + 1, location.1)] == val &&
                location.1 < map.num_columns() - 1 && map[(location.0, location.1 + 1)] == val &&
                map[(location.0 + 1, location.1 + 1)] != val {
                corner_count += 1;
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