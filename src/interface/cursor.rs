pub fn move_home(){
    println!("\x1B[H");
}
pub fn cursor_move(y: i16, x: i16) {
    // y: Line
    // x: Columns
    print!("\x1B[{y};{x}f");
}