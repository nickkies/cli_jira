pub fn get_column_string(text: &str, width: usize) -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;
        assert_eq!(get_column_string(text4, width), "".to_string());

        let width = 1;
        assert_eq!(get_column_string(text4, width), ".".to_string());

        let width = 2;
        assert_eq!(get_column_string(text4, width), "..".to_string());

        let width = 3;
        assert_eq!(get_column_string(text4, width), "...".to_string());

        let width = 4;
        assert_eq!(get_column_string(text4, width), "t...".to_string());

        let width = 6;
        assert_eq!(get_column_string(text1, width), "      ".to_string());
        assert_eq!(get_column_string(text2, width), "test  ".to_string());
        assert_eq!(get_column_string(text3, width), "testme".to_string());
        assert_eq!(get_column_string(text4, width), "tes...".to_string());
    }
}
