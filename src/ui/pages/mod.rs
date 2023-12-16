use std::{any::Any, rc::Rc};

use anyhow::Result;
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
        todo!()
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
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
    use crate::models::Epic;

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
}
