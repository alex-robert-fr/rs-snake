use std::thread::sleep;
use std::time::Duration;
use game_engine::{input, renderer};

mod game_engine;

fn game_loop() {
    let mut x = 0;
    loop {
        let input = input::process_input();
        update();
        renderer::renderer(input, &mut x);
        sleep(Duration::from_millis(1000 / 30));
    }
}

fn update() {
    println!("Update");
}

fn main() {
    game_loop();
}
