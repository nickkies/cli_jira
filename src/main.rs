use std::rc::Rc;

use db::JiraDatabase;
use io_utils::{get_user_input, wait_for_key_press};
use navigator::Navigator;

mod db;
mod io_utils;
mod models;
mod navigator;
mod ui;

fn main() {
    let db = Rc::new(JiraDatabase::new("data/db.json".to_string()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            };

            let user_input = get_user_input();

            match page.handle_input(&user_input) {
                Err(error) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {}\nPress any key to continue...", error);
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
