use algorithms::mst::kruskal::{Edge, Graph, MstKruskal};
use algorithms::test_data::TEST_MATRIX;

#[test]
fn test_correct_kruskal_minimum_spanning_tree() {
    let mut graph = Graph::new();
    for i in 0..TEST_MATRIX.len() {
        for j in 0..TEST_MATRIX[i].len() {
            if TEST_MATRIX[i][j] == 0 {
                continue;
            }

            graph.insert(Edge::new(i as u32, j as u32), TEST_MATRIX[i][j] as u32);
        }
    }

    let minimum_spanning_tree = MstKruskal::minimum_spanning_tree(&graph);

    assert_eq!(
        minimum_spanning_tree,
        Some(
            vec![
                Edge::new(3, 4),
                Edge::new(0, 1),
                Edge::new(0, 2),
                Edge::new(4, 6),
                Edge::new(3, 5),
                Edge::new(1, 3)
            ]
            .into_iter()
            .collect()
        )
    );
}
