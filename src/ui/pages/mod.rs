use std::{any::Any, rc::Rc};

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::{db::JiraDatabase, models::Action, ui::pages::page_helpers::get_column_string};

mod page_helpers;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>,
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("----------------------------- EPICS ------------------------------");
        println!("     id     |               name               |      status      ");

        let epics = self.db.read_db()?.epics;

        epics.keys().sorted().into_iter().for_each(|epic_id| {
            let epic = &epics[epic_id];
            let id_col = get_column_string(&epic_id.to_string(), 11);
            let name_col = get_column_string(&epic.name, 32);
            let status_col = get_column_string(&epic.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        });

        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epics = self.db.read_db()?.epics;

        match input {
            "q" => Ok(Some(Action::Exit)),
            "c" => Ok(Some(Action::CreateEpic)),
            input => {
                if let Ok(epic_id) = input.parse::<u32>() {
                    if epics.contains_key(&epic_id) {
                        return Ok(Some(Action::NavigateToEpicDetail { epic_id }));
                    }
                }
                Ok(None)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        let id_col = get_column_string(&self.epic_id.to_string(), 5);
        let name_col = get_column_string(&epic.name, 12);
        let desc_col = get_column_string(&epic.description, 27);
        let status_col = get_column_string(&epic.status.to_string(), 13);
        println!("{} | {} | {} | {}", id_col, name_col, desc_col, status_col);

        println!();

        println!("---------------------------- STORIES -----------------------------");
        println!("     id     |               name               |      status      ");

        let stories = &db_state.stories;

        epic.stories
            .iter()
            .sorted()
            .into_iter()
            .for_each(|epic_id| {
                let story = &stories[epic_id];
                let id_col = get_column_string(&epic_id.to_string(), 11);
                let name_col = get_column_string(&story.name, 32);
                let status_col = get_column_string(&story.status.to_string(), 17);
                println!("{} | {} | {}", id_col, name_col, status_col);
            });

        println!();
        println!();

        println!("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let db_state = self.db.read_db()?;
        let stories = db_state.stories;
        let epic_id = self.epic_id;

        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateEpicStatus { epic_id })),
            "d" => Ok(Some(Action::DeleteEpic { epic_id })),
            "c" => Ok(Some(Action::CreateStory { epic_id })),
            input => {
                if let Ok(story_id) = input.parse::<u32>() {
                    if stories.contains_key(&story_id) {
                        return Ok(Some(Action::NavigateToStoryDetail { epic_id, story_id }));
                    }
                }
                Ok(None)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        todo!()
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_utils::MockDB;
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;

        #[test]
        fn draw_page_should_not_throw_error() {
            let page = HomePage {
                db: Rc::new(JiraDatabase {
                    database: Box::new(MockDB::new()),
                }),
            };

            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let page = HomePage {
                db: Rc::new(JiraDatabase {
                    database: Box::new(MockDB::new()),
                }),
            };

            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let epic_id = db
                .create_epic(Epic::new("".to_string(), "".to_string()))
                .unwrap();
            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junck_input = "junckinput";
            let junck_input_with_valid_prefix = "qjunckinput";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(
                page.handle_input(&valid_epic_id).unwrap(),
                Some(Action::NavigateToEpicDetail { epic_id: 1 })
            );
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junck_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junck_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod epic_detail_page {
        use super::*;

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let invalid_epic_id = 999;
            let page = EpicDetail {
                epic_id: invalid_epic_id,
                db: Rc::new(JiraDatabase {
                    database: Box::new(MockDB::new()),
                }),
            };

            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let page = EpicDetail {
                epic_id: db
                    .create_epic(Epic::new("".to_string(), "".to_string()))
                    .unwrap(),
                db,
            };

            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let page = EpicDetail {
                epic_id: db
                    .create_epic(Epic::new("".to_string(), "".to_string()))
                    .unwrap(),
                db,
            };

            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let epic_id = db
                .create_epic(Epic::new("".to_string(), "".to_string()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_string(), "".to_string()), epic_id)
                .unwrap();
            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_input = "999";
            let junk_input: &str = "junkinput";
            let junk_input_with_valid_prefix = "pjunkinput";
            let input_with_trailing_white_spaces = "p\n";
            let epic_id = 1;

            assert_eq!(
                page.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateEpicStatus { epic_id })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteEpic { epic_id })
            );
            assert_eq!(
                page.handle_input(c).unwrap(),
                Some(Action::CreateStory { epic_id })
            );
            assert_eq!(
                page.handle_input(&story_id.to_string()).unwrap(),
                Some(Action::NavigateToStoryDetail {
                    epic_id,
                    story_id: 2
                })
            );
            assert_eq!(page.handle_input(invalid_input).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod story_detail_page {
        use super::*;

        use crate::db::{test_utils::MockDB, JiraDatabase};

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let invalid_story_id = 999;
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let page = StoryDetail {
                epic_id: db
                    .create_epic(Epic::new("".to_string(), "".to_string()))
                    .unwrap(),
                story_id: invalid_story_id,
                db,
            };
            assert_eq!(page.draw_page().is_err(), true);
        }
    }
}
