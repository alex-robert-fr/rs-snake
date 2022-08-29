use std::io::{stdout, Write};

pub fn clear_all_screen() {
    print!("\x1B[2J");
    stdout().flush().unwrap();
}