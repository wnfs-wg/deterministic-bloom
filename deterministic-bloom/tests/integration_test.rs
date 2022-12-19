#[test]
fn test_add() {
    assert_eq!(deterministic_bloom::add(3, 2), 5);
}
