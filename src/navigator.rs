use std::rc::Rc;

use anyhow::Result;

use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{HomePage, Page, Prompts},
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
            Action::NavigateToEpicDetail { epic_id } => todo!(),
            Action::NavigateToStoryDetail { epic_id, story_id } => todo!(),
            Action::NavigateToPreviousPage => todo!(),
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
        let db = Rc::new(JiraDatabase {
            database: Box::new(MockDB::new()),
        });
        let nav = Navigator::new(db);

        assert_eq!(nav.get_page_count(), 1);

        let current_page = nav.get_current_page().unwrap();
        let home_page = current_page.as_any().downcast_ref::<HomePage>();

        assert_eq!(home_page.is_some(), true);
    }
}
