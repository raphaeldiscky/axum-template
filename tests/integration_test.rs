use rs_template::add;

#[test]
fn test_add_positive_numbers() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_add_negative_numbers() {
    assert_eq!(add(-1, -2), -3);
}

#[test]
fn test_add_zero() {
    assert_eq!(add(0, 0), 0);
}
