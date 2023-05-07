mod graph {

    use std::collections::{HashMap, HashSet};

    pub fn bfs(graph: &Vec<(i32, i32)>, start: i32) -> HashMap<i32, i32> {
        let mut visited = HashSet::new();
        let mut distances = HashMap::new();
        let mut queue = Vec::new();

        visited.insert(start);
        distances.insert(start, 0);
        queue.push(start);

        while !queue.is_empty() {
            let node = queue.remove(0);

            for edge in graph {
                if edge.0 == node && !visited.contains(&edge.1) {
                    visited.insert(edge.1);
                    distances.insert(edge.1, distances[&node] + 1);
                    queue.push(edge.1);
                }
            }
        }

        distances
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
    }
}
