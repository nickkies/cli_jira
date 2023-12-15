use std::{any::Any, rc::Rc};

use anyhow::Result;

use crate::{db::JiraDatabase, models::Action};

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
