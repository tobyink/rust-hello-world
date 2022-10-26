fn get_greeting() -> String {
    "Hello, world!".to_string()
}

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
}
