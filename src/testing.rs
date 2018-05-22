macro_rules! assert_float_eq {
    ($left:expr, $right:expr, $eps:expr) => {{
        let left = $left;
        let right = $right;
        assert!(
            (left - right).abs() < $eps,
            "Expected: {}, got: {}",
            left,
            right
        );
    }};
}
