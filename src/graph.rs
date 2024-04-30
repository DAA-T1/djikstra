use std::{fmt, str::FromStr};

/// Graph data structure based on adjacency lists
///
/// NOTE: no guarantees about the graph being in a valid state are made
/// and the user must therefore make sure that the string they are parsing
/// or they vector they are making a graph out of is a valid graph
///
#[derive(Debug)]
pub struct Graph {
    // `adj` is the adjacency list
    // the index corresponds to a vertex and the value at that index
    // is the list of neighbors with associated weights
    pub adj: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    /// create a graph from a given adjacency list
    ///
    /// # Example
    /// ```
    /// use djikstra::graph::Graph;
    ///
    /// let adj_list = vec![
    ///     vec![(2, 3), (1, 3)],
    ///     vec![(0, 3)],
    ///     vec![(0, 3)]
    /// ];
    /// let graph = Graph::new(adj_list);
    /// ```
    pub fn new(adj: Vec<Vec<(usize, usize)>>) -> Self {
        Self { adj }
    }

    /// Number of vertices
    pub fn n_vertices(&self) -> usize {
        self.adj.len()
    }

    /// Number of edges
    pub fn n_edges(&self) -> usize {
        self.adj.iter().fold(0, |acc, x| acc + x.len())
    }

    /// Get neighbors of a vertex
    pub fn neighbors_of(&self, vertex: usize) -> &[(usize, usize)] {
        &self.adj[vertex]
    }
}

/// The error type returned when we run into any error when parsing
/// a graph.
/// The cause of the error is within the struct and can be accessed easily
#[derive(Debug, PartialEq, Eq)]
pub struct ParseGraphError(String);

impl fmt::Display for ParseGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Graph {
    type Err = ParseGraphError;

    /// Parse a string into a graph
    /// # Examples
    /// ```
    /// use djikstra::graph::Graph;
    /// use std::str::FromStr;
    ///
    /// // creating a graph by parsing a string
    /// let graph_str = r#"3
    /// 2,3 1,3
    /// 0,3
    /// 0,3"#;
    ///
    /// let graph1 = Graph::from_str(graph_str);
    /// // or alternatively
    /// let graph2 = graph_str.parse::<Graph>();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n_vertex_str, edges) = s
            .split_once('\n')
            .ok_or(ParseGraphError("cannot split on newline".to_string()))?;

        let n_vertex = n_vertex_str
            .parse()
            .map_err(|e| ParseGraphError(format!("cannot parse n_vertices: {}", e)))?;

        let mut adj = vec![vec![]; n_vertex];

        for (vertex, neighbors) in edges.lines().take(n_vertex).enumerate() {
            let neighbors_parsed = neighbors.split_whitespace().map(|edge_str| {
                edge_str.split_once(',').ok_or(ParseGraphError(
                    "vertex doesnt have weight with it".to_string(),
                ))
            });

            for res in neighbors_parsed {
                let (v, weight) = res?;
                adj[vertex].push((
                    v.parse()
                        .map_err(|e| ParseGraphError(format!("cannot parse vertex: {}", e)))?,
                    weight
                        .parse()
                        .map_err(|e| ParseGraphError(format!("cannot parse weight: {}", e)))?,
                ))
            }
        }

        Ok(Self { adj })
    }
}

impl PartialEq for Graph {
    /// This method tests for self and other values to be equal, and is used by `==`.
    ///
    /// NOTE: we consider two graphs equal if each of their
    /// vertices have the same neighbors with same associated weights.
    /// the order in which the vertices are in the neighbor vector of
    /// a specific vertex does not matter. i.e, we consider the two graphs
    /// [[(1, 2), (2, 2)], [0, 2], [0, 2]] and
    /// [[(2, 2), (1, 2)], [0, 2], [0, 2]] equal.
    ///
    /// NOTE: there might be a better way of implementing this
    /// but oh well...
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
    fn correctly_unequal2() {
        let g1 = Graph::new(vec![
            vec![(1, 3), (2, 3)],
            vec![(2, 2), (0, 3)],
            vec![(1, 2), (0, 2)],
        ]);
        let g2 = Graph::new(vec![
            vec![(1, 3), (2, 3)],
            vec![(0, 3), (2, 2)],
            vec![(0, 3), (1, 2)],
        ]);
        assert_ne!(g1, g2);
    }

    #[test]
    fn parses_correctly() {
        let graph_str = r#"3
1,3 2,3
2,2 0,3
1,2 0,3"#;
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
    fn parses_correctly_one_empty() {
        let graph_str = r#"3
1,3

0,3"#;
        let should_be = Graph::new(vec![vec![(1, 3)], vec![], vec![(0, 3)]]);
        let parsed = Graph::from_str(graph_str);
        assert!(parsed.is_ok());

        assert_eq!(parsed.unwrap(), should_be);
    }

    #[test]
    fn parses_graph_last_empty() {
        let graph_str = r#"5
3,3
2,1
1,1
0,3"#;
        let should_be = Graph::new(vec![
            vec![(3, 3)],
            vec![(2, 1)],
            vec![(1, 1)],
            vec![(0, 3)],
            vec![],
        ]);
        let parsed = Graph::from_str(graph_str);
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), should_be);
    }

    #[test]
    fn fails_when_no_n_vertices() {
        let graph_str = r#"1,3 2,3
2,2 0,3
1,2 0,3"#;
        let parsed = Graph::from_str(graph_str);
        assert!(parsed.is_err());
    }

    #[test]
    fn n_edges_is_correct() {
        let g1 = Graph::new(vec![
            vec![(1, 3), (3, 4)],
            vec![(3, 4), (5, 2)],
            vec![(4, 2), (3, 2)],
            vec![(2, 2)],
            vec![(4, 2), (3, 2), (1, 4)],
        ]);
        assert_eq!(g1.n_edges(), 10);
    }

    #[test]
    fn parses_empty_graph() {
        let graph_str = r#"3


"#;
        let parsed = Graph::from_str(graph_str);

        let should_be = Graph::new(vec![vec![]; 3]);

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), should_be);
    }

    #[test]
    fn parses_graph_with_one_elist() {
        let graph_str = r#"4


3,3
2,3"#;
        let parsed = Graph::from_str(graph_str);

        let should_be = Graph::new(vec![vec![], vec![], vec![(3, 3)], vec![(2, 3)]]);

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), should_be);
    }
}
