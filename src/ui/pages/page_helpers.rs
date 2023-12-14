pub fn mod_test() -> bool {
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mod() {
        assert_eq!(mod_test(), true);
    }
}
