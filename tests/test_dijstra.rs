use algorithms::dijkstra::{Dijkstra, Edge};
use algorithms::test_data::{TEST_MATRIX, TEST_MATRIX_DISCONNECTED};

#[test]
fn test_correct_minimum_cost() {
    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    //
    // Chosen for its efficiency.
    let graph = vec![
        vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
        vec![Edge { node: 3, cost: 2 }],
        vec![
            Edge { node: 1, cost: 1 },
            Edge { node: 3, cost: 3 },
            Edge { node: 4, cost: 1 },
        ],
        vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
        vec![],
    ];

    assert_eq!(Dijkstra::shortest_path(&graph, 0, 1), Some(1));
    assert_eq!(Dijkstra::shortest_path(&graph, 0, 3), Some(3));
    assert_eq!(Dijkstra::shortest_path(&graph, 3, 0), Some(7));
    assert_eq!(Dijkstra::shortest_path(&graph, 0, 4), Some(5));
    assert_eq!(Dijkstra::shortest_path(&graph, 4, 0), None);
}

#[test]
fn test_correct_minimum_cost_of_an_adjacent_matrix() {
    let len = TEST_MATRIX.len();
    let mut adjacent_list: Vec<Vec<Edge>> = (0..len).map(|_| Vec::new()).collect();

    for i in 0..len {
        for j in 0..TEST_MATRIX[i].len() {
            if TEST_MATRIX[i][j] == 0 {
                continue;
            }

            adjacent_list[i].push(Edge {
                node: j,
                cost: TEST_MATRIX[i][j] as usize,
            })
        }
    }

    assert_eq!(Dijkstra::shortest_path(&adjacent_list, 0, 6), Some(44));
}

#[test]
fn test_correct_minimum_cost_of_a_disconnected_adjacent_matrix() {
    let len = TEST_MATRIX_DISCONNECTED.len();
    let mut adjacent_list: Vec<Vec<Edge>> = (0..len).map(|_| Vec::new()).collect();

    for i in 0..len {
        for j in 0..TEST_MATRIX_DISCONNECTED[i].len() {
            if TEST_MATRIX_DISCONNECTED[i][j] == 0 {
                continue;
            }

            adjacent_list[i].push(Edge {
                node: j,
                cost: TEST_MATRIX_DISCONNECTED[i][j] as usize,
            })
        }
    }

    assert_eq!(Dijkstra::shortest_path(&adjacent_list, 0, 5), None);
}
