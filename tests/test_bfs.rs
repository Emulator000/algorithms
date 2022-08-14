use algorithms::bfs::Bfs;
use algorithms::test_data::{TEST_MATRIX, TEST_MATRIX_DISCONNECTED};

#[test]
fn test_is_matrix_fully_connected() {
    let mut bfs = Bfs::new();

    assert_eq!(bfs.bfs_adjacent_matrix(TEST_MATRIX, 0), 1);
}

#[test]
fn test_is_matrix_not_fully_connected() {
    let mut bfs = Bfs::new();

    assert_eq!(bfs.bfs_adjacent_matrix(TEST_MATRIX_DISCONNECTED, 0), 2);
}
