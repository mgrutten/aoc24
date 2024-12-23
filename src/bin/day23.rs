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


fn part1(connected: &HashMap<[char; 2], HashSet<[char; 2]>>) {
    let mut triplets = HashSet::new();
    for key in connected.keys() {
        for (first, second) in find_pairs(&connected.get(key).unwrap()) {
            if connected.get(&first).unwrap().contains(&second) &&
                (key[0] == 't' || first[0] == 't' || second[0] == 't') {
                let mut sorted_triplet = [*key, first, second];
                sorted_triplet.sort();
                triplets.insert((sorted_triplet[0], sorted_triplet[1], sorted_triplet[2]));
            }
        }
    }

    println!("Part 1: {:?}", triplets.len());
}


fn part2(connected: &HashMap<[char; 2], HashSet<[char; 2]>>) {
    let mut visited = HashSet::new();
    let mut max_clique = HashSet::new();

    for key in connected.keys() {
        // Check key wasn't in previous clique
        if visited.contains(key) {
            continue;
        }

        // Find clique containing key
        let mut current_clique = HashSet::new();
        current_clique.insert(*key);

        let neighbours = connected.get(key).unwrap()
            .difference(&visited)
            .collect::<HashSet<_>>();
        for node in neighbours {
            if connected.get(node).unwrap().is_superset(&current_clique) {
                current_clique.insert(*node);
            }
        }

        // Add clique to visited nodes
        visited.extend(&current_clique);

        // Check for max clique
        if current_clique.len() > max_clique.len() {
            max_clique = current_clique;
        }
    }

    // Print max clique members in sorted order
    let mut sorted_clique = max_clique.into_iter().collect::<Vec<_>>();
    sorted_clique.sort();

    print!("Part 2: ");
    sorted_clique.iter().for_each(|&x| print!("{}{},", x[0], x[1]));
    println!();
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

    let mut connected = HashMap::new();
    for connection in connections {
        connected.entry(connection[0]).or_insert(HashSet::new()).insert(connection[1]);
        connected.entry(connection[1]).or_insert(HashSet::new()).insert(connection[0]);
    }

    part1(&connected);
    part2(&connected);

    Ok(())
}
