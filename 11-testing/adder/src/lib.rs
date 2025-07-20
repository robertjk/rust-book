pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_adder_test() {
        assert_eq!(4, internal_adder(2, 2));
        assert_eq!(0, internal_adder(0, 0));
        assert_eq!(99, internal_adder(99, 0));
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail!");
    }

    #[test]
    fn can_hold() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));

        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 8,
            height: 1,
        };
        assert!(!larger.can_hold(&smaller));

        let larger = Rectangle {
            width: 2,
            height: 2,
        };
        let smaller = Rectangle {
            width: 1,
            height: 2,
        };
        assert!(!larger.can_hold(&smaller));
    }
}
