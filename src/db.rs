use std::collections::HashMap;

use anyhow::Result;

use crate::models::{DBState, Epic, Status, Story};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        todo!();
        Ok(DBState {
            last_item_id: 0,
            epics: HashMap::new(),
            stories: HashMap::new(),
        })
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        // TODO const or let?
        const INVALID_JSON: &str = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
        const VALID_JSON: &str = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_string(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let result = read_json(INVALID_JSON);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let result = read_json(VALID_JSON);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            write!(tmpfile, "{VALID_JSON}").unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("Failed to convert tmpfile path to str")
                    .to_string(),
            };

            let story = Story {
                name: "stroy 1 name".to_string(),
                description: "story 1 description".to_string(),
                status: Status::Open,
            };
            let epic = Epic {
                name: "epic 1 name".to_string(),
                description: "epic 1 description".to_string(),
                status: Status::Open,
                stories: vec![2],
            };

            let stories = HashMap::from([(2, story)]);
            let epics = HashMap::from([(1, epic)]);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }

        fn read_json(file_contents: &str) -> Result<DBState> {
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
