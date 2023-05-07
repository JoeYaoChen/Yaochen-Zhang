use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet, VecDeque};

// Define the graph as a HashMap of HashSet
type Graph = HashMap<usize, HashSet<usize>>;

// reading the file
fn read_graph(filename: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    let file = std::fs::read_to_string(filename).expect("Failed to open file");
    for line in file.lines() {
        let edge: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        graph.entry(edge[0]).or_insert_with(HashSet::new).insert(edge[1]);
    }
    graph
}

// calculating distances between all vertices using BFS
fn bfs_distance(graph: &Graph, start: usize) -> Vec<Option<usize>> {
    let mut distances = vec![None; graph.len()];
    let mut queue = VecDeque::new();
    distances[start] = Some(0);
    queue.push_back(start);
    while let Some(vertex) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&vertex) {
            for neighbor in neighbors {
                if distances[*neighbor].is_none() {
                    distances[*neighbor] = Some(distances[vertex].unwrap() + 1);
                    queue.push_back(*neighbor);
                }
            }
        }
    }
    distances
}

// find the distance from the starting vertex
fn find_vertices_at_distance(graph: &Graph, start: usize, distance: usize) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(start);
    queue.push_back(start);
    let mut current_distance = 0;
    while !queue.is_empty() && current_distance <= distance {
        let level_size = queue.len();
        for _ in 0..level_size {
            let vertex = queue.pop_front().unwrap();
            if let Some(neighbors) = graph.get(&vertex) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(*neighbor);
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
        current_distance += 1;
    }
    visited
}

// reading files calculation and print
fn main() {
    let graph = read_graph("p2p-Gnuttella08.txt");
    let num_vertices = graph.len();
    let mut total_distance = 0;
    for i in 0..num_vertices {
        let distances = bfs_distance(&graph, i);
        for j in 0..num_vertices {
            if let Some(distance) = distances[j] {
                total_distance += distance;
            }
        }
    }
    let avg_distance = total_distance as f64 / (num_vertices * (num_vertices - 1)) as f64;
    println!("Average distance: {}", avg_distance);
    let start = 3;
    let distance = 2;
    let vertices = find_vertices_at_distance(&graph, start, distance);
    println!("Vertices at distance {} from {}: {:?}", distance, start, vertices);
}

// test, using bfs to compute the distance
#[test]
fn test_bfs() {
    let graph = vec![
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 4),
        (2, 4),
        (2, 5),
        (3, 5),
        (4, 6),
        (5, 6),
        (5, 7),
        (6, 8),
        (7, 8),
    ];
    let distances = bfs(&graph, 0);
    assert_eq!(distances[&0], 0);
    assert_eq!(distances[&1], 1);
    assert_eq!(distances[&2], 1);
    assert_eq!(distances[&3], 1);
    assert_eq!(distances[&4], 2);
    assert_eq!(distances[&5], 2);
    assert_eq!(distances[&6], 3);
    assert_eq!(distances[&7], 3);
    assert_eq!(distances[&8], 4);
}

