use anyhow::Result;

use crate::models::DBState;

trait Database {
    fn read_db(&self) -> Result<DBState>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_string(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;

            let result = read_db(file_contents);

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;

            let result = read_db(file_contents);

            assert_eq!(result.is_ok(), true);
        }

        fn read_db(file_contents: &str) -> Result<DBState> {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            write!(tmpfile, "{file_contents}").unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            db.read_db()
        }
    }
}
