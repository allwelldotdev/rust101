pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: usize) -> usize {
    a + 2
}

pub fn add_three(a: usize) -> usize {
    add_two(a) + 1
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
    // let _ = name;
    // String::from("Hello!")
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.")
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.")
        }

        Self { value }
    }
}

// Showing function output
pub fn print_and_return_10(a: i32) -> i32 {
    println!("I got the value {a}.");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn test_panic() {
        panic!("This test fails");
    }

    #[test]
    fn test_larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 5,
        };
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 5,
        };
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_two() {
        let result = add_two(2);
        assert_eq!(add_two(2), result);
    }

    #[test]
    fn test_greeting() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "greeting() did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn test_guess_new() {
        Guess::new(200);
    }

    #[test]
    fn test_add_two_with_result_type() -> Result<(), String> {
        let result = add_two(2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal four"))
        }
    }

    #[test]
    fn test_print_and_return_10() {
        let result = print_and_return_10(2);
        assert_eq!(result, 10);
    }
}
