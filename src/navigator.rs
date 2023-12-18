use std::rc::Rc;

use anyhow::Result;

use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{Page, Prompts},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    propmpts: Prompts,
    db: Rc<JiraDatabase>,
}

impl Navigator {
    pub fn new(db: Rc<JiraDatabase>) -> Self {
        todo!()
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        todo!()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigator_mod_test() {
        assert_eq!(true, true);
    }
}
