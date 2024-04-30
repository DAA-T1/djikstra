use ::djikstra::djikstra::djikstra;
use djikstra::graph::Graph;
fn main() {
    println!("Hello, world!");
    let g1 = Graph::new(vec![
        vec![(1, 3), (6, 2)],
        vec![(0, 3), (2, 4), (3, 1), (6, 1), (4, 4), (7, 6)],
        vec![(6, 6), (1, 4), (3, 2), (4, 2)],
        vec![(1, 1), (2, 2), (4, 1), (7, 2)],
        vec![(2, 2), (3, 1), (1, 4), (7, 1), (5, 3)],
        vec![(4, 3), (7, 4)],
        vec![(0, 2), (1, 1), (2, 6), (4, 5)],
        vec![(4, 1), (5, 4), (3, 2), (1, 6)]
    ]);
    
    let paths = djikstra(&g1, 6);
}
