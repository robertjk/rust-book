//! # Playground
//!
//! `playground` is a collection of utilities to make performing certarin
//! calculations more convenient.

/// Add two numbers
///
/// # Examples
/// ```
/// let arg1 = 10;
/// let arg2 = 3;
/// let answer = playground::add(arg1, arg2);
///
///
/// assert_eq!(13, answer);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = playground::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
