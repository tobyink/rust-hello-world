/// Retrieve the greeting that will be printed by the app.
fn get_greeting() -> String {
    "Hello, world!".to_string()
}

/// Gives a friendly hello!
fn main() {
    println!("{}", get_greeting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(13, get_greeting().len());
    }

    #[test]
    fn test_main() {
        main();
        assert!(true);
    }
}
