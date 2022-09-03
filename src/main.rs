use std::process;

use game_engine::{Game, Engine};
use snake::Snake;

mod interface;
mod snake;
mod game_engine;

fn main() {
    process::Command::new("clear");
    Game::run();
}
