pub fn example_add(a: i32) -> i32 {
    a + 1
}

#[test]
fn test_add() {
    assert_eq!(example_add(1), 2);
}

#[test]
fn test_bad_add() {
    // This assert would fire and test will fail.
    assert_eq!(example_add(1), 3);
}
