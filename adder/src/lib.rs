#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn panic_stations() {
    panic!("PANIC PANIC!");
}

#[cfg(test)]
mod tests {
    use super::*;

    static LARGER: Rectangle = Rectangle{ length: 8, width: 7 };
    static SMALLER: Rectangle = Rectangle{ length: 5, width: 1 };

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Panic panic!");
    }

    #[test]
    fn larger_can_hold_smaller() {
        assert!(LARGER.can_hold(&SMALLER));
    }

    #[test]
    fn smaller_can_hold_larger() {
        assert!(!SMALLER.can_hold(&LARGER));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "PANIC PANIC!")]
    fn test_panic_stations() {
        panic_stations();
    }
}
