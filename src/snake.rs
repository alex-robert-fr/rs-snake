#[derive(PartialEq)]
pub enum Direction {
    Up,
    Rigth,
    Down,
    Left,
}

pub struct Position {
    pub x: i16,
    pub y: i16,
}

pub struct Part {
    pub prev_pos: Position,
    pub current_pos: Position,
    pub next_pos: Position,
}

pub struct Snake {
    pub texture: String,
    pub speed: u8,
    pub life: u8,
    pub size: u8,
    pub direction: Direction,
    pub pos: Position,
    pub parts: Vec<Part>,
}

impl Snake {
    pub fn new(tex: &str, speed: u8, life: u8) -> Snake {
        let size = 11;
        Snake {
            texture: tex.to_string(),
            speed,
            life,
            size,
            direction: Direction::Rigth,
            pos: Position { x: 50, y: 5 },
            parts: Snake::reset(size),
        }
    }

    pub fn reset(size: u8) -> Vec<Part> {
        let mut initial_pos = Position { x: 50, y: 5 };
        let mut parts: Vec<Part> = Vec::new();
        for _ in 0..size {
            parts.push(Part {
                prev_pos: Position {
                    x: initial_pos.x,
                    y: initial_pos.y,
                },
                current_pos: Position {
                    x: initial_pos.x,
                    y: initial_pos.y,
                },
                next_pos: Position {
                    x: initial_pos.x,
                    y: initial_pos.y,
                },
            });
            initial_pos.x -= 1;
        }
        parts
    }

    pub fn change_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.direction = Direction::Up,
            Direction::Rigth => self.direction = Direction::Rigth,
            Direction::Down => self.direction = Direction::Down,
            Direction::Left => self.direction = Direction::Left,
        }
    }

    pub fn forward(&mut self) {
        match self.direction {
            Direction::Up => self.pos.y -= 1,
            Direction::Rigth => self.pos.x += 1,
            Direction::Down => self.pos.y += 1,
            Direction::Left => self.pos.x -= 1,
        }
    }
}
