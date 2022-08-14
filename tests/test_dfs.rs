use algorithms::dfs::Dfs;
use algorithms::test_data::{TEST_MATRIX, TEST_MATRIX_DISCONNECTED};

#[test]
fn test_is_matrix_fully_connected() {
    let mut dfs = Dfs::new();

    assert_eq!(dfs.dfs_adjacent_matrix(TEST_MATRIX, 0), 1);
}

#[test]
fn test_is_matrix_not_fully_connected() {
    let mut dfs = Dfs::new();

    assert_eq!(dfs.dfs_adjacent_matrix(TEST_MATRIX_DISCONNECTED, 0), 2);
}
