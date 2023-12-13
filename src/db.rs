use crate::models::{DBState, Epic, Status, Story};
use anyhow::{anyhow, Result};
use std::fs;

pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut parsed = self.database.read_db()?;
        let new_id = parsed.last_item_id + 1;

        parsed.last_item_id = new_id;
        parsed.epics.insert(new_id, epic);

        let _ = self.database.write_db(&parsed);

        Ok(new_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed = self.database.read_db()?;
        let new_id = parsed.last_item_id + 1;

        parsed.last_item_id = new_id;
        parsed.stories.insert(new_id, story);
        parsed
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("could not find epic in database!"))?
            .stories
            .push(new_id);

        let _ = self.database.write_db(&parsed);

        Ok(new_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        todo!()
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        todo!()
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        todo!()
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        todo!()
    }
}

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path)?;
        let parsed: DBState = serde_json::from_str(&db_content)?;
        Ok(parsed)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    // bc test util
    #[allow(dead_code)]
    impl MockDB {
        pub fn new() -> Self {
            Self {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::MockDB;
    use super::*;

    mod database {
        use std::{collections::HashMap, io::Write};

        use super::*;

        #[test]
        fn create_epic_should_work() {
            let db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };
            let epic = Epic::new("".to_string(), "".to_string());

            let result = db.create_epic(epic.clone());

            assert_eq!(result.is_ok(), true);

            let id = result.unwrap();
            let db_state = db.read_db().unwrap();

            let expected_id = 1;

            assert_eq!(id, expected_id);
            assert_eq!(db_state.last_item_id, expected_id);
            assert_eq!(db_state.epics.get(&id), Some(&epic));
        }

        #[test]
        fn create_story_should_error_if_invalid_epic_id() {
            let db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };
            let story = Story::new("".to_string(), "".to_string());

            let non_existent_epic_id = 9999;

            let result = db.create_story(story, non_existent_epic_id);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn create_story_should_work() {
            let db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };
            let epic = Epic::new("".to_string(), "".to_string());
            let story = Story::new("".to_string(), "".to_string());

            let result = db.create_epic(epic);
            assert_eq!(result.is_ok(), true);

            let epic_id = result.unwrap();

            let result = db.create_story(story.clone(), epic_id);

            let id = result.unwrap();
            let db_state = db.read_db().unwrap();

            let expected_id = 2;

            assert_eq!(id, expected_id);
            assert_eq!(db_state.last_item_id, expected_id);
            assert_eq!(
                db_state.epics.get(&epic_id).unwrap().stories.contains(&id),
                true
            );
            assert_eq!(db_state.stories.get(&id), Some(&story));
        }

        #[test]
        fn delete_epic_should_error_if_invalid_epic_id() {
            let db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };

            let non_existent_epic_id = 9999;

            let result = db.delete_epic(non_existent_epic_id);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn delete_story_should_error_if_invalid_epic_id() {
            let db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };
            let epic = Epic::new("".to_string(), "".to_string());
            let story = Story::new("".to_string(), "".to_string());

            let result = db.create_epic(epic);
            assert_eq!(result.is_ok(), true);

            let epic_id = result.unwrap();

            let result = db.create_story(story, epic_id);
            assert_eq!(result.is_ok(), true);

            let story_id = result.unwrap();
            let non_existent_epic_id = 9999;

            let result = db.delete_story(non_existent_epic_id, story_id);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn delete_story_should_error_if_story_not_found_in_epic() {
            let db = JiraDatabase {
                database: Box::new(MockDB::new()),
            };
            let epic = Epic::new("".to_string(), "".to_string());
            let story = Story::new("".to_string(), "".to_string());

            let result = db.create_epic(epic);
            assert_eq!(result.is_ok(), true);

            let epic_id = result.unwrap();

            let result = db.create_story(story, epic_id);
            assert_eq!(result.is_ok(), true);

            let non_existent_epic_id = 9999;

            let result = db.delete_story(epic_id, non_existent_epic_id);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_string(),
            };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let file_content = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            let result = read_json(file_content);
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let file_content = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            let result = read_json(file_content);
            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            let file_content = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{file_content}").unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("Failed to convert tmpfile path to str")
                    .to_string(),
            };

            let story = Story {
                name: "story 1 name".to_string(),
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
