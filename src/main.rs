use std::thread::sleep;
use std::time::Duration;

mod game_engine;

fn game_loop() {
    loop {
        game_engine::input::process_input();
        update();
        renderer();
        sleep(Duration::from_millis(1000 / 30));
    }
}



fn update() {
    println!("Update");
}

fn renderer() {
    println!("Renderer");
}

fn main() {
    game_loop();
}
