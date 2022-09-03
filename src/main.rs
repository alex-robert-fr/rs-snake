use std::{
    io::{stdout, Write},
    process,
    thread::sleep,
    time::Duration,
};

mod interface;
mod snake;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use interface::{cursor::cursor_move, screen::clear_all_screen};
use snake::{Snake, Direction};

struct Game {
    fps: u64,
    snake: Snake,
    term_size: (u16, u16),
}

#[derive(PartialEq)]
enum Actions {
    Direction(Direction),
    Reset,
    Exit,
    None,
}

impl Game {
    fn new(fps: u64, snake: Snake) -> Game {
        Game {
            fps,
            snake,
            term_size: size().unwrap(),
        }
    }

    fn game_loop(&mut self) {
        loop {
            let input = Game::process_input();
            let update = Game::update(self, &input);
            Game::renderer(self);
            if (input == Actions::Exit) || update ==  Actions::Exit {
                break;
            }
            sleep(Duration::from_millis(1000 / self.fps));
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

    fn update(&mut self, event: &Actions)  -> Actions{
        if self.snake.life <= 0 {
            return Actions::Exit;
        } 
        match event {
            Actions::Direction(dir) => self.snake.change_direction(dir),
            Actions::None => (),
            Actions::Exit => (),
            Actions::Reset => {
                self.snake.parts = Snake::reset(self.snake.size - 1)
            }
            _ => ()
        }

        let _x_max = match i16::try_from(self.term_size.0) {
            Ok(val) => val,
            Err(_) => panic!("Oups")
        };
        let snake_x = self.snake.pos.x;
        match snake_x {
            snake_x if snake_x == 0 => self.snake.pos.x = _x_max,
            snake_x if snake_x > _x_max => self.snake.pos.x = 0,
            _ => (),
        }
        let _y_max = match i16::try_from(self.term_size.1) {
            Ok(val) => val,
            Err(_) => panic!("Oups")
        };
        let snake_y = self.snake.pos.y;
        match snake_y {
            snake_y if snake_y == 0 => self.snake.pos.y = _y_max,
            snake_y if snake_y > _y_max => self.snake.pos.y = 0,
            _ => (),
        }
        self.snake.forward();
        Actions::None
    }
    // pos.x = 1
    // déplace a 1

    fn renderer(&mut self) {
        clear_all_screen();
        cursor_move(0, 150);
        print!(
            "Life: {}",
            self.snake.life,
        );
        cursor_move(self.snake.pos.y, self.snake.pos.x);
        print!("#");
        let mut i = 0;
        loop {
            if i == 0 {
                self.snake.parts[i].current_pos.x = self.snake.parts[i].next_pos.x;
                self.snake.parts[i].current_pos.y = self.snake.parts[i].next_pos.y;
                cursor_move(
                    self.snake.parts[i].current_pos.y,
                    self.snake.parts[i].current_pos.x,
                );
                print!("a");
                self.snake.parts[i].prev_pos.x = self.snake.parts[i].current_pos.x;
                self.snake.parts[i].prev_pos.y = self.snake.parts[i].current_pos.y;

                self.snake.parts[i].next_pos.x = self.snake.pos.x;
                self.snake.parts[i].next_pos.y = self.snake.pos.y;
            } else if i < self.snake.size.into() {
                if (self.snake.parts[i].current_pos.x == self.snake.pos.x) && (self.snake.parts[i].current_pos.y == self.snake.pos.y) {
                    self.snake.life -= 1;
                }
                self.snake.parts[i].current_pos.x = self.snake.parts[i].next_pos.x;
                self.snake.parts[i].current_pos.y = self.snake.parts[i].next_pos.y;
                cursor_move(
                    self.snake.parts[i].current_pos.y,
                    self.snake.parts[i].current_pos.x,
                );
                print!("a");
                self.snake.parts[i].prev_pos.x = self.snake.parts[i].current_pos.x;
                self.snake.parts[i].prev_pos.y = self.snake.parts[i].current_pos.y;

                self.snake.parts[i].next_pos.x = self.snake.parts[i - 1].current_pos.x;
                self.snake.parts[i].next_pos.y = self.snake.parts[i - 1].current_pos.y;
            } else {
                break;
            }
            i += 1;
        }
        stdout().flush().unwrap();
    }
}

fn main() {
    process::Command::new("clear");
    let snake = Snake::new("#", 2, 5);
    let mut game = Game::new(30, snake);
    game.game_loop();
}
