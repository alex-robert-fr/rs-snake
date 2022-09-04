use crossterm::terminal::size;
use rand::{thread_rng, Rng};

use crate::snake::Position;

pub struct Apple {
    pub texture: String,
    pub pos: Position,
}

impl Apple {
    pub fn new(texture: &str) -> Apple{
        Apple { texture: texture.to_string(), pos: Position { x: 0, y: 0 } }
    }
    pub fn generate(&mut self) {
        let mut rng = thread_rng();
        let (c, l) = size().unwrap();
        self.pos = Position {
            x: { rng.gen_range(5..c.try_into().unwrap()) },
            y: { rng.gen_range(5..l.try_into().unwrap()) },
        };
    }
}
