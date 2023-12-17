use crate::{
    io_utils::{get_user_input, get_user_input_trimmed},
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Epic Name: ");
    let epic_name = get_user_input_trimmed();

    println!("Epic Description: ");
    let epic_desc = get_user_input_trimmed();

    Epic::new(epic_name, epic_desc)
}

fn create_story_prompt() -> Story {
    println!("----------------------------");
    println!("Story Name: ");
    let story_name = get_user_input_trimmed();

    println!("Story Description: ");
    let story_desc = get_user_input_trimmed();

    Story::new(story_name, story_desc)
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]: ");
    let input = get_user_input_trimmed();

    if input.eq("Y") {
        true
    } else {
        false
    }
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]: ");
    let input = get_user_input_trimmed();

    if input.eq("Y") {
        true
    } else {
        false
    }
}

fn update_status_prompt() -> Option<Status> {
    todo!();
}
