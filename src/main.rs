fn main() {
    println!("Hello, world!");
}

fn give_two() -> i32 {
    2
}

#[cfg(test)]
mod dcode_test {
    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(1 == 1); // OK
        panic!("Oh no!");
    }

    #[test]
    fn test_equals() {
        assert_eq!(super::give_two(), 1 + 1);
        assert_ne!(super::give_two(), 1 + 2);
    }
}