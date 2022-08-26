use std::io::{stdout, Write};

pub fn move_home(){
    println!("\x1B[H");
    // stdout().flush().unwrap();
}
pub fn cursor_move(x: isize, y: isize) {
    print!("\x1B[{y};{x}f");
}