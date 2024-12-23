use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn find_pairs(set: &HashSet<[char; 2]>) -> Vec<([char; 2], [char; 2])> {
    let mut pairs = Vec::new();

    let vec: Vec<_> = set.iter().collect();
    for i in 0..vec.len() {
        for j in i + 1..vec.len() {
            pairs.push((*vec[i], *vec[j]));
        }
    }

    pairs
}


fn part1(connections: &Vec<Vec<[char; 2]>>) {
    let mut connected = HashMap::new();
    for connection in connections {
        connected.entry(connection[0]).or_insert(HashSet::new()).insert(connection[1]);
        connected.entry(connection[1]).or_insert(HashSet::new()).insert(connection[0]);
    }

    let mut triplets = HashSet::new();
    for key in connected.keys() {
        for (first, second) in find_pairs(&connected.get(key).unwrap()) {
            if connected.get(&first).unwrap().contains(&second) &&
                (key[0] == 't' || first[0] == 't' || second[0] == 't') {
                let mut sorted_triplet = [*key, first, second];
                sorted_triplet.sort_by(|a, b| a.cmp(&b));
                triplets.insert((sorted_triplet[0], sorted_triplet[1], sorted_triplet[2]));
            }
        }
    }

    println!("Part 1: {:?}", triplets.len());
}


fn part2(connections: &Vec<Vec<[char; 2]>>) {
    let mut connected = HashMap::new();
    for connection in connections {
        connected.entry(connection[0]).or_insert(HashSet::new()).insert(connection[1]);
        connected.entry(connection[1]).or_insert(HashSet::new()).insert(connection[0]);
    }

    let mut r = HashSet::new();
    let mut p = connected.keys().cloned().collect();
    let mut x = HashSet::new();

    let max_clique = bron_kerbosch(&connected, &mut r, &mut p, &mut x);

    let mut sorted_clique = max_clique.into_iter().collect::<Vec<_>>();
    sorted_clique.sort();

    print!("Part 2: ");
    sorted_clique.iter().for_each(|&x| print!("{}{},", x[0], x[1]));
    println!();
}

fn bron_kerbosch(
    graph: &HashMap<[char; 2], HashSet<[char; 2]>>,
    current_clique: &HashSet<[char; 2]>,
    potential_nodes: &HashSet<[char; 2]>,
    excluded_nodes: &HashSet<[char; 2]>,
) -> HashSet<[char; 2]> {
    if potential_nodes.is_empty() && excluded_nodes.is_empty() {
        return current_clique.clone();
    }

    let mut max_clique = current_clique.clone();

    let mut potential_nodes = potential_nodes.clone();
    let mut excluded_nodes = excluded_nodes.clone();
    
    for node in potential_nodes.clone() {
        let mut new_clique = current_clique.clone();
        new_clique.insert(node);

        let neighbors = &graph[&node];
        let new_potential = potential_nodes.intersection(neighbors).cloned().collect::<HashSet<_>>();
        let new_excluded = excluded_nodes.intersection(neighbors).cloned().collect::<HashSet<_>>();

        let clique = bron_kerbosch(graph, &new_clique, &new_potential, &new_excluded);
        
        if clique.len() > max_clique.len() {
            max_clique = clique;
        }

        potential_nodes.remove(&node);
        excluded_nodes.insert(node);
    }

    max_clique
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day23/day23.txt")?;

    let connections = file_str.lines()
        .map(|line| {
            line.split('-')
                .map(|s| s.chars().collect::<Vec<_>>().try_into().unwrap())
                .collect::<Vec<[char; 2]>>()
        })
        .collect::<Vec<_>>();

    part1(&connections);
    part2(&connections);

    Ok(())
}
