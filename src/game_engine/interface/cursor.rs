use std::io::{stdout, Write};

pub fn move_home(){
    println!("\x1B[H");
    // stdout().flush().unwrap();
}
