use crossterm::event::Event;

use crate::game_engine::interface::{screen::clear_all_screen, cursor::move_home};

pub fn renderer(input: Option<Event>) {
    clear_all_screen();
    move_home();
    match input {
        Some(key) => println!("{:?}", key),
        None => ()
    }
    println!("Renderer");
}