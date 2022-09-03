use crate::snake::Position;

pub fn move_home(){
    println!("\x1B[H");
}
pub fn cursor_move(pos: Position) {
    // y: Line
    // x: Columns
    print!("\x1B[{};{}f", pos.y, pos.x);
}