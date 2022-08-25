use std::time::Duration;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, event::{poll, read, Event}};

pub fn process_input() -> Option<Event> {
    enable_raw_mode().unwrap();
    match poll(Duration::from_millis(100)) {
        Ok(val) => {
            if val {
                match read() {
                    Ok(val) => return Some(val),
                    Err(_) => (),
                };
            } else {
                println!("Process Input");
            }
        }
        Err(_) => (),
    }
    disable_raw_mode().unwrap();
    None
}