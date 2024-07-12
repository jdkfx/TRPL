#[derive(Debug)]
struct Rec {
    width: u32,
    height: u32,
}

impl Rec {
    fn can_hold(&self, other: &Rec) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("予想させる値は1から100までの間でなければなりませんが, {}でした. ", value);
        }
        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello, {}!", name)
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("このテストを失敗させる!");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rec {
            width: 8,
            height: 7,
        };
        let smaller = Rec {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rec {
            width: 8,
            height: 7,
        };
        let smaller = Rec {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[should_panic]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "greetingは名前:Carolを含んでいません. その値は{}でした.",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn use_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 3 は 4 ではありません"))
        }
    }
}
