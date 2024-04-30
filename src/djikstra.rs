// Djikstra algorithm !!!
// uses Graph and PriorityQueue

use crate::graph::Graph;
use crate::pq::PriorityQueue;

/// Djikstra algorithm that takes in a graph and a source node!
/// Returns a list of paths
pub fn djikstra(graph: &Graph, src: usize) -> (Vec<Vec<usize>>, Vec<usize>) {
    let n_elems = graph.n_vertices();
    let mut parents = vec![None; n_elems];
    let mut dists_from_src = vec![usize::MAX; n_elems];
    let mut checked = vec![false; n_elems];
    let mut pq: PriorityQueue<usize> = PriorityQueue::from_keys((0..n_elems - 1).collect());

    dists_from_src[src] = 0;
    pq.change_key(&src, 0);

    while let Some((node, dist_src)) = pq.extract_min() {
        let neighbours = graph.neighbors_of(node);

        for (neighbour, dist) in neighbours.iter() {
            let neighbour = *neighbour;

            if !checked[neighbour] && dists_from_src[neighbour] > dist + dist_src {
                dists_from_src[neighbour] = dist + dist_src;
                parents[neighbour] = Some(node);
                pq.change_key(&neighbour, dists_from_src[neighbour]);
            }
        }
        checked[node] = true;
    }

    let mut paths_from_src: Vec<Vec<usize>> = vec![vec![]; n_elems];
    for idx in 0..=(n_elems - 1) {
        if idx != src {
            let mut paths: Vec<usize> = vec![idx];
            let mut parent = parents[idx];
            while parent.unwrap() != src {
                paths.insert(0, parent.unwrap());
                parent = parents[parent.unwrap()];
            }
            paths.insert(0, src);
            paths_from_src[idx].append(&mut paths);
        }
    }
    paths_from_src[src].append(&mut vec![src]);

    (paths_from_src, dists_from_src)
}

// #[cfg(test)]
// mod tests {
//     use crate::djikstra::djikstra;
//     use crate::graph::Graph;

//     #[test]
//     fn correct_path() {
//         let g1 = Graph::new(vec![
//             vec![(1, 4), (2, 1)],
//             vec![(0, 4), (2, 2), (3, 5)],
//             vec![(0, 1), (1, 2), (3, 5)],
//             vec![(1, 5), (2, 1)],
//         ]);
//         let paths = djikstra(&g1, 2);
//         assert_eq!(paths, vec![vec![2, 0], vec![2, 1], vec![2], vec![3, 2]])
//     }
// }
