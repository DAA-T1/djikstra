use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Graph {
    // the index corresponds to a vertex and the value at that index
    // are its neighbors
    pub adj: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    fn new(adj: Vec<Vec<(usize, usize)>>) -> Self {
        Self { adj }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParseGraphError(String);

impl FromStr for Graph {
    type Err = ParseGraphError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n_vertices, edges) = s
            .split_once('\n')
            .ok_or(ParseGraphError("cannot split on newline".to_string()))?;

        let n_vertices = n_vertices
            .parse::<usize>()
            .map_err(|e| ParseGraphError(format!("cannot parse n_vertices: {}", e)))?;

        let mut edge_vec = vec![vec![]; n_vertices];

        for (vertex, neighbors) in edges.lines().enumerate() {
            let neighbors_parsed = neighbors.split_whitespace().map(|edge_str| {
                edge_str.split_once(',').ok_or(ParseGraphError(
                    "vertex doesnt have weight with it".to_string(),
                ))
            });

            for res in neighbors_parsed {
                let (v, weight) = res?;
                edge_vec[vertex].push((
                    v.parse()
                        .map_err(|e| ParseGraphError(format!("cannot parse vertex: {}", e)))?,
                    weight
                        .parse()
                        .map_err(|e| ParseGraphError(format!("cannot parse weight: {}", e)))?,
                ))
            }
        }

        Ok(Self { adj: edge_vec })
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        if self.adj.len() != other.adj.len() {
            return false;
        }
        for (vertex, neighbors) in self.adj.iter().enumerate() {
            let other_neighbors = &other.adj[vertex];
            if neighbors.len() != other_neighbors.len() {
                return false;
            }
            if !(neighbors.iter().all(|x| other_neighbors.contains(x))
                && other_neighbors.iter().all(|x| neighbors.contains(x)))
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_equal() {
        let g1 = Graph::new(vec![
            vec![(1, 3), (2, 3)],
            vec![(2, 2), (0, 3)],
            vec![(1, 2), (0, 3)],
        ]);
        let g2 = Graph::new(vec![
            vec![(1, 3), (2, 3)],
            vec![(0, 3), (2, 2)],
            vec![(0, 3), (1, 2)],
        ]);
        assert_eq!(g1, g2);
    }

    #[test]
    fn correctly_unequal() {
        let g1 = Graph::new(vec![
            vec![(1, 3), (2, 3)],
            vec![(2, 2), (0, 3)],
            vec![(1, 2), (0, 3)],
        ]);
        let g2 = Graph::new(vec![
            vec![(1, 3), (2, 3)],
            vec![(0, 3), (2, 2)],
            vec![(0, 3)],
        ]);
        assert_ne!(g1, g2);
    }

    #[test]
    fn parses_correctly() {
        let graph_str = r#"3
    1,3 2,3
    2,2 0,3
    1,2 0,3
    "#;
        let should_be = Graph::new(vec![
            vec![(1, 3), (2, 3)],
            vec![(2, 2), (0, 3)],
            vec![(1, 2), (0, 3)],
        ]);
        let parsed = Graph::from_str(graph_str);
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), should_be);
    }

    #[test]
    fn fails_when_no_n_vertices() {
        let graph_str = r#"1,3 2,3
    2,2 0,3
    1,2 0,3
    "#;
        let parsed = Graph::from_str(graph_str);
        assert!(parsed.is_err());
    }
}
