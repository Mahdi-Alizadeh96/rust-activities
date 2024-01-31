// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {

    match b {
        0 => None,
        _ => Some(a / b)
    }

}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_clamp_within_range() {
        assert_eq!(clamp(5, 1, 10), 5);
        assert_eq!(clamp(0, -5, 5), 0);
    }

    #[test]
    fn test_clamp_below_range() {
        assert_eq!(clamp(-3, 0, 10), 0);
    }

    #[test]
    fn test_clamp_above_range() {
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_div_valid_input() {
        assert_eq!(div(10, 2), Some(5));
        assert_eq!(div(15, 3), Some(5));
    }

    #[test]
    fn test_div_by_zero() {
        assert_eq!(div(5, 0), None);
    }

    #[test]
    fn test_concat_non_empty_strings() {
        assert_eq!(concat("Hello", "World"), "Hello World");
    }

    #[test]
    fn test_concat_empty_strings() {
        assert_eq!(concat("", ""), " ");
    }
}

fn main() {}