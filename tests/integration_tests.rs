use f64_fixed::to_fixed_string;

#[test]
fn test_places() {
    assert_eq!(to_fixed_string(1234567.0, 5), "1.23+6");
    assert_eq!(to_fixed_string(1234567.0, 6), "1.23+6");
    assert_eq!(to_fixed_string(1000.0, 7), "1000   ");
    assert_eq!(to_fixed_string(1000.0, 8), "1000    ");
}

#[test]
fn test_6_places() {
    assert_eq!(to_fixed_string(0.0, 6), "0     ");
    assert_eq!(to_fixed_string(0.1234, 6), "0.1234");
    assert_eq!(to_fixed_string(0.01234, 6), "0.0123");
    assert_eq!(to_fixed_string(0.001234, 6), "0.0012");
    assert_eq!(to_fixed_string(-0.001234, 6), "-0.001");
    assert_eq!(to_fixed_string(0.0001234, 6), "0.12-3");
    assert_eq!(to_fixed_string(-0.0001234, 6), "-0.1-3");
    assert_eq!(to_fixed_string(12., 6), "12    ");
    assert_eq!(to_fixed_string(12.345, 6), "12.345");
    assert_eq!(to_fixed_string(12.36, 6), "12.36 ");
    assert_eq!(to_fixed_string(123., 6), "123   ");
    assert_eq!(to_fixed_string(1234., 6), "1234  ");
    assert_eq!(to_fixed_string(9999., 6), "9999  ");
    assert_eq!(to_fixed_string(-999., 6), "-999  ");
    assert_eq!(to_fixed_string(-1001., 6), "-1001 ");
    assert_eq!(to_fixed_string(10001., 6), "10001 ");
    assert_eq!(to_fixed_string(12345., 6), "12345 ");
    assert_eq!(to_fixed_string(123456., 6), "123456");
    assert_eq!(to_fixed_string(1234567., 6), "1.23+6");
    assert_eq!(to_fixed_string(12345678., 6), "12.3+6");
    assert_eq!(to_fixed_string(123456789., 6), "123+6 ");
}

#[test]
fn test_alignment() {
    assert_eq!(to_fixed_string(123456789., -6), " 123+6");
}
