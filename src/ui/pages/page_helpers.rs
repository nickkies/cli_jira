use std::cmp::Ordering::{Equal, Greater, Less};

use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let len = text.len();

    match len.cmp(&width) {
        Equal => text.to_string(),
        Less => {
            let left_over = width - len;
            let mut column_string = text.to_string();
            let padding = std::iter::repeat(' ').take(left_over).collect::<String>();

            column_string.push_str(&padding);

            column_string
        }
        Greater => {
            let result;
            if width == 0 {
                result = "".to_string();
            } else if width == 1 {
                result = ".".to_string();
            } else if width == 2 {
                result = "..".to_string();
            } else if width == 3 {
                result = "...".to_string();
            } else {
                result = text.truncate_ellipse(width - 3).to_string();
            }

            result
        }
    }
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
