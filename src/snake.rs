use crate::game_engine::Game;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Rigth,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone)]
pub struct Part {
    pub current_pos: Position,
    pub next_pos: Position,
}

#[derive(Clone)]
pub struct Snake {
    pub texture: String,
    pub speed: u8,
    pub life: u8,
    pub size: u8,
    pub direction: Direction,
    pub parts: Vec<Part>,
}

impl Snake {
    pub fn new(tex: &str, speed: u8, life: u8) -> Snake {
        let size = 5;
        Snake {
            texture: tex.to_string(),
            speed,
            life,
            size,
            direction: Direction::Rigth,
            parts: Snake::init(size, &mut Position { x: 50, y: 5 }),
        }
    }

    pub fn init(size: u8, initial_pos: &mut Position) -> Vec<Part> {
        let mut parts: Vec<Part> = Vec::new();
        for _ in 0..size {
            initial_pos.x -= 1;
            parts.push(Part {
                current_pos: Position {
                    x: initial_pos.x,
                    y: initial_pos.y,
                },
                next_pos: Position {
                    x: initial_pos.x,
                    y: initial_pos.y,
                },
            });
        }
        parts
    }

    pub fn reset(&mut self) {
        let mut parts: Vec<Part> = Vec::new();
        let mut initial_pos = self.parts[0].current_pos;
        match self.direction {
            Direction::Up => initial_pos.y += 1,
            Direction::Rigth => initial_pos.x -= 1,
            Direction::Down => initial_pos.y -= 1,
            Direction::Left => initial_pos.x += 1,
        }
        for i in 0..self.size {
            let i: usize = i.try_into().unwrap();
            let size_without_last = self.size - 1;
            if i < size_without_last.into() {
                parts.push(Part {
                    current_pos: self.parts[i].current_pos,
                    next_pos: self.parts[i].next_pos,
                });
            } else {
                parts.push(Part {
                    current_pos: self.parts[i - 1].current_pos,
                    next_pos: self.parts[i - 1].next_pos,
                });
            }
        }
        self.parts = parts;
    }

    pub fn change_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up
                }
            }
            Direction::Rigth => {
                if self.direction != Direction::Left {
                    self.direction = Direction::Rigth
                }
            }
            Direction::Down => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down
                }
            }
            Direction::Left => {
                if self.direction != Direction::Rigth {
                    self.direction = Direction::Left
                }
            }
        }
    }

    pub fn forward(dir: Direction, pos: Position) -> Position {
        match dir {
            Direction::Up => {
                return Position {
                    y: pos.y - 1,
                    x: pos.x,
                }
            }
            Direction::Rigth => {
                return Position {
                    y: pos.y,
                    x: pos.x + 1,
                }
            }
            Direction::Down => {
                return Position {
                    y: pos.y + 1,
                    x: pos.x,
                }
            }
            Direction::Left => {
                return Position {
                    y: pos.y,
                    x: pos.x - 1,
                }
            }
        }
    }

    pub fn calc_pos(&mut self) {
        let mut i = 0;
        while i < self.size.into() {
            self.parts[i].current_pos = self.parts[i].next_pos;
            self.parts[i].next_pos = {
                match i {
                    0 => Snake::forward(self.direction, self.parts[0].current_pos),
                    _ => self.parts[i - 1].current_pos,
                }
            };
            i += 1;
        }
    }

    pub fn verify_pos(game: &mut Game) {
        // If exit screen
        let x_max = match i16::try_from(game.term_size.0) {
            Ok(val) => val,
            Err(_) => panic!("Oups"),
        };
        let y_max = match i16::try_from(game.term_size.1) {
            Ok(val) => val,
            Err(_) => panic!("Oups"),
        };

        let head_part_pos = game.snake.parts[0].current_pos;
        if head_part_pos.x < 0
            || head_part_pos.x > x_max
            || head_part_pos.y < 1
            || head_part_pos.y > y_max
        {
            game.snake.life -= 1;
            game.reset = true;
        }

        //If touch apple
        if head_part_pos == game.apple.pos {
            game.score += 1;
            game.apple.pos = Position { x: 0, y: 0 };
            game.snake.size += 1;
            game.snake.reset();
        }

        // If touch yourself
        let mut j = 1;
        while j < game.snake.size.into() {
            if head_part_pos == game.snake.parts[j].current_pos {
                game.snake.life -= 1;
                game.reset = true;
            }
            j += 1;
        }
    }
}
