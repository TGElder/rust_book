//! # Add One
//!
//! Highly useful crate containing highly useful function to add one to a arbitrary number
//!

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one::add_one(5));
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}