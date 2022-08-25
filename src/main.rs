use crossterm::event::{poll, read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::thread::sleep;
use std::time::Duration;

fn game_loop() {
    loop {
        process_input();
        update();
        renderer();
        sleep(Duration::from_millis(1000 / 30));
    }
}

fn process_input() {
    enable_raw_mode().unwrap();
    match poll(Duration::from_millis(100)) {
        Ok(val) => {
            if val {
                match read() {
                    Ok(val) => println!("{:?}", val),
                    Err(_) => (),
                };
            } else {
                println!("Process Input");
            }
        }
        Err(_) => (),
    }
    disable_raw_mode().unwrap();
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
