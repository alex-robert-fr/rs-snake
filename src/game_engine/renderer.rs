use std::io::{stdout, Write};

use crossterm::event::Event;

use crate::game_engine::interface::{screen::clear_all_screen, cursor::move_home};

use super::interface::cursor::cursor_move;

pub fn move_snake(symbol: &str, x: isize) {
    cursor_move(x, 4);
    print!("{symbol}");
    stdout().flush().unwrap();
}

pub fn renderer(input: Option<Event>, x: &mut isize) {
    clear_all_screen();
    move_home();
    match input {
        Some(key) => println!("{:?}", key),
        None => ()
    }
    println!("\x1B[J");
    println!("Renderer {x}");
    move_snake("#", *x);
    *x += 1;
}