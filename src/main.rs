use std::{
    io::{stdout, Write},
    process,
    thread::sleep,
    time::Duration,
};

mod interface;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use interface::{cursor::cursor_move, screen::clear_all_screen};

#[derive(PartialEq)]
enum Direction {
    Up,
    Rigth,
    Down,
    Left,
}

struct Position {
    x: i16,
    y: i16,
}

struct Snake {
    texture: String,
    speed: u8,
    life: u8,
    direction: Direction,
    pos: Position,
}

impl Snake {
    fn new(tex: &str, speed: u8, life: u8) -> Snake {
        Snake {
            texture: tex.to_string(),
            speed,
            life,
            direction: Direction::Rigth,
            pos: Position { x: 100, y: 10 },
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.direction = Direction::Up,
            Direction::Rigth => self.direction = Direction::Rigth,
            Direction::Down => self.direction = Direction::Down,
            Direction::Left => self.direction = Direction::Left,
        }
    }

    fn forward(&mut self) {
        match self.direction {
            Direction::Up => self.pos.y -= 1,
            Direction::Rigth => self.pos.x += 1,
            Direction::Down => self.pos.y += 1,
            Direction::Left => self.pos.x -= 1,
        }
    }
}

struct Game {
    fps: u8,
    snake: Snake,
}

#[derive(PartialEq)]
enum Actions {
    All,
    Direction(Direction),
    Exit,
    None,
}

impl Game {
    fn new(fps: u8, snake: Snake) -> Game {
        Game { fps, snake }
    }

    fn game_loop(&mut self) {
        loop {
            let input = Game::process_input();
            if input == Actions::Exit {
                break;
            }
            Game::update(self, input);
            Game::renderer(self);
            sleep(Duration::from_millis(30));
        }
    }

    fn process_input() -> Actions {
        enable_raw_mode().unwrap();
        match poll(Duration::from_millis(100)) {
            Ok(val) => {
                if val {
                    match read().unwrap() {
                        // Exit Game
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Exit;
                        }

                        // Droite
                        Event::Key(KeyEvent {
                            code: KeyCode::Right,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Direction(Direction::Rigth);
                        },

                        // Gauche
                        Event::Key(KeyEvent {
                            code: KeyCode::Left,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Direction(Direction::Left);
                        },

                        // Haut
                        Event::Key(KeyEvent {
                            code: KeyCode::Up,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Direction(Direction::Up);
                        },
                        
                        // Bas
                        Event::Key(KeyEvent {
                            code: KeyCode::Down,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Direction(Direction::Down);
                        },

                        _ => (),
                    }
                }
            }
            Err(_) => (),
        }
        disable_raw_mode().unwrap();
        Actions::None
    }

    fn update(&mut self, event: Actions) {
        match event {
            Actions::All => println!("Touche press"),
            Actions::Direction(dir) => self.snake.change_direction(dir),
            Actions::None => (),
            Actions::Exit => (),
        }

        self.snake.forward();
    }

    fn renderer(&self) {
        clear_all_screen();
        cursor_move(self.snake.pos.y, self.snake.pos.x);
        print!("{}", self.snake.texture);
        stdout().flush().unwrap();
    }
}

fn main() {
    process::Command::new("clear");
    let snake = Snake::new("#", 2, 5);
    let mut game = Game::new(30, snake);
    game.game_loop();
}
