use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};

use crate::{
    apple::Apple,
    interface::{cursor::cursor_move, screen::clear_all_screen},
    snake::{Direction, Position, Snake},
};

#[derive(PartialEq)]
pub enum Actions {
    Direction(Direction),
    Exit,
    None,
}

pub trait Engine {
    fn process_input() -> Actions;
    fn update(game: &mut Game, input: &Actions);
    fn renderer(game: &mut Game);
    fn run();
}

pub struct Game {
    fps: u64,
    snake: Snake,
    apple: Apple,
    score: u16,
    term_size: (u16, u16),
    exit: bool,
    reset: bool,
}

impl Engine for Game {
    fn process_input() -> Actions {
        enable_raw_mode().unwrap();
        match poll(Duration::from_millis(0)) {
            Ok(val) => {
                if val {
                    match read().unwrap() {
                        // Exit Game
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        })
                        | Event::Key(KeyEvent {
                            code: KeyCode::Char('Q'),
                            modifiers: KeyModifiers::SHIFT,
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
                        }

                        // Gauche
                        Event::Key(KeyEvent {
                            code: KeyCode::Left,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Direction(Direction::Left);
                        }

                        // Haut
                        Event::Key(KeyEvent {
                            code: KeyCode::Up,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Direction(Direction::Up);
                        }

                        // Bas
                        Event::Key(KeyEvent {
                            code: KeyCode::Down,
                            modifiers: KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::NONE,
                        }) => {
                            disable_raw_mode().unwrap();
                            return Actions::Direction(Direction::Down);
                        }

                        _ => (),
                    }
                }
            }
            Err(_) => (),
        }
        disable_raw_mode().unwrap();
        Actions::None
    }

    fn update(game: &mut Game, input: &Actions) {
        if game.snake.life <= 0 {
            game.exit = true;
        }

        if game.apple.pos == (Position { x: 0, y: 0 }) {
            game.apple.generate();
        }

        match input {
            Actions::Direction(dir) => game.snake.change_direction(dir),
            _ => (),
        }

        let x_max = match i16::try_from(game.term_size.0) {
            Ok(val) => val,
            Err(_) => panic!("Oups"),
        };
        let y_max = match i16::try_from(game.term_size.1) {
            Ok(val) => val,
            Err(_) => panic!("Oups"),
        };

        // Calcule snake position
        let snake = &mut game.snake;
        let mut i = 0;
        while i < snake.size.into() {
            let mut part = &mut snake.parts;
            part[i].current_pos = part[i].next_pos;
            part[i].next_pos = {
                match i {
                    0 => Snake::forward(snake.direction, {
                        // If exit screen
                        let head_part = part[0].current_pos;
                        if head_part.x > x_max
                            || head_part.x < 0
                            || head_part.y > y_max
                            || head_part.y < 1
                        {
                            game.score += 1;
                            game.reset = true;
                        }

                        //If touch apple
                        if part[0].current_pos == game.apple.pos {
                            game.score += 1;
                            game.apple.pos = Position { x: 0, y: 0 };
                            snake.size += 1;
                            // part = Snake::reset(snake.size, &mut part[0].next_pos);
                        }

                        // If touch yourself
                        let mut j = 1;
                        while j < snake.size.into() {
                            if head_part == part[j].current_pos && head_part != game.apple.pos {
                                game.score += 1;
                                game.reset = true;
                            }
                            j += 1;
                        }
                        part[0].current_pos
                    }),
                    _ => part[i - 1].current_pos,
                }
            };
            i += 1;
        }
    }

    fn renderer(game: &mut Game) {
        clear_all_screen();

        // Display Apple
        cursor_move(game.apple.pos);
        print!("{}", game.apple.texture);

        // Display Snake
        let snake = &mut game.snake;
        let mut i = 0;
        while i < snake.size.into() {
            let part = &mut snake.parts;
            cursor_move(part[i].current_pos);
            print!("{}", snake.texture);
            i += 1;
        }

        cursor_move(Position { x: 100, y: 0 });
        print!("Life: {}, Points: {}", snake.life, game.score);

        stdout().flush().unwrap();
    }

    fn run() {
        let mut life = 5;
        'main: loop {
            let game = &mut Game::new(20, life);
            'game: loop {
                let input = Self::process_input();
                Self::update(game, &input);
                Self::renderer(game);

                if input == Actions::Exit || game.exit {
                    break 'main;
                }

                if game.reset {
                    life -= 1;
                    break 'game;
                }

                sleep(Duration::from_millis(1000 / game.fps));
            }
        }
    }
}

impl Game {
    pub fn new(fps: u64, life: u8) -> Game {
        Game {
            fps,
            snake: Snake::new("#", 2, life),
            apple: Apple::new("o"),
            score: 0,
            term_size: size().unwrap(),
            exit: false,
            reset: false,
        }
    }
}
