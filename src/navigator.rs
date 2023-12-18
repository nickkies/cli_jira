use std::rc::Rc;

use anyhow::Result;

use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{EpicDetail, HomePage, Page, Prompts, StoryDetail},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    propmpts: Prompts,
    db: Rc<JiraDatabase>,
}

impl Navigator {
    pub fn new(db: Rc<JiraDatabase>) -> Self {
        Self {
            pages: vec![Box::new(HomePage { db: Rc::clone(&db) })],
            propmpts: Prompts::new(),
            db,
        }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail { epic_id } => self.pages.push(Box::new(EpicDetail {
                epic_id,
                db: Rc::clone(&self.db),
            })),
            Action::NavigateToStoryDetail { epic_id, story_id } => {
                self.pages.push(Box::new(StoryDetail {
                    epic_id,
                    story_id,
                    db: Rc::clone(&self.db),
                }))
            }
            Action::NavigateToPreviousPage => {
                if !self.pages.is_empty() {
                    self.pages.pop();
                }
            }
            Action::CreateEpic => todo!(),
            Action::UpdateEpicStatus { epic_id } => todo!(),
            Action::DeleteEpic { epic_id } => todo!(),
            Action::CreateStory { epic_id } => todo!(),
            Action::UpdateStoryStatus { story_id } => todo!(),
            Action::DeleteStory { epic_id, story_id } => todo!(),
            Action::Exit => todo!(),
        }

        Ok(())
    }

    // fn for testing
    #[allow(dead_code)]
    fn get_page_count(&self) -> usize {
        self.pages.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::db::test_utils::MockDB;

    use super::*;

    #[test]
    fn should_start_on_home_page() {
        let nav = Navigator::new(Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        }));
        assert_eq!(nav.get_page_count(), 1);

        let home_page = nav
            .get_current_page()
            .unwrap()
            .as_any()
            .downcast_ref::<HomePage>();
        assert_eq!(home_page.is_some(), true);
    }

    #[test]
    fn handle_action_should_navigate_pages() {
        let mut nav = Navigator::new(Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        }));

        nav.handle_action(Action::NavigateToEpicDetail { epic_id: 1 })
            .unwrap();
        assert_eq!(nav.get_page_count(), 2);

        let epic_detail_page = nav
            .get_current_page()
            .unwrap()
            .as_any()
            .downcast_ref::<EpicDetail>();
        assert_eq!(epic_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToStoryDetail {
            epic_id: 1,
            story_id: 2,
        })
        .unwrap();
        assert_eq!(nav.get_page_count(), 3);

        let story_detail_page = nav
            .get_current_page()
            .unwrap()
            .as_any()
            .downcast_ref::<StoryDetail>();
        assert_eq!(story_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(nav.get_page_count(), 2);

        let epic_detail_page = nav
            .get_current_page()
            .unwrap()
            .as_any()
            .downcast_ref::<EpicDetail>();
        assert_eq!(epic_detail_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(nav.get_page_count(), 1);

        let home_page = nav
            .get_current_page()
            .unwrap()
            .as_any()
            .downcast_ref::<HomePage>();
        assert_eq!(home_page.is_some(), true);

        nav.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(nav.get_page_count(), 0);

        nav.handle_action(Action::NavigateToPreviousPage).unwrap();
        assert_eq!(nav.get_page_count(), 0);
    }
}