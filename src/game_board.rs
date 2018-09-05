extern crate rand;

use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use piston::input::*;
use art::TileSheet;
use app_base::*;
use game_board::rand::*;

pub enum GameBoardEvent {
    Finish,
    Fail(u32),
    NoEvent
}

impl GameEvent for GameBoardEvent {
    //placeholder
}


#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct GameBoard {
    width: u32,
    height: u32,
    snake: Vec<(u32, u32)>, //vec of (x,y)
    dir: Direction,
    food: (u32, u32),
    time_since_update: f64,
    rand: rand::rngs::ThreadRng
}


impl GameBoard {
    pub fn new(w: u32, h: u32) -> Self {
        GameBoard {
            width: w,
            height: h,
            snake: vec![(0,0)],
            dir: Direction::Right,
            food: (w-1,h-1),
            time_since_update: 0.0,
            rand: rand::thread_rng()
        }
    }

    //determines if the next move is valid or not
    //is not valid if any of the following conditions are met
    //1. head_x is 0 and move left
    //2. head_x is w-1 and move right
    //3. head_y is 0 and move up
    //4. head_y is h-1 and move down
    //5. head_x, head_y hits part of the snake 
    fn is_bad_move(&self, new_head: (u32, u32)) -> bool {
        //check walls
        let (head_x, head_y) = &self.snake.get(0).unwrap();
        let (head_x, head_y) = (*head_x, *head_y);

        let invalid = 
            (head_x == 0 && 
             self.dir == Direction::Left) ||
            (head_x == self.width - 1 && 
             self.dir == Direction::Right) ||
            (head_y == 0 &&
             self.dir == Direction::Up) ||
            (head_y == self.height - 1 && 
             self.dir == Direction::Down);


        invalid || self.pos_in_snake(new_head)
    }

    fn get_next_head(&self) -> (u32, u32) {
        let (curx, cury) = &self.snake.get(0).unwrap();
        let (curx, cury) = (*curx, *cury);

        let (x,y) = match self.dir {
            Direction::Left => {
                if curx == 0 {
                    (curx, cury)
                } else {
                    (curx - 1, cury)
                }
            },
            Direction::Right => {
                (curx + 1, cury)
            },
            Direction::Up => {
                if cury == 0 {
                    (curx, cury)
                } else {
                    (curx, cury - 1)
                }
            },
            Direction::Down => {
                (curx, cury + 1)
            }
        };
        (x,y)
    }

    fn pos_in_snake(&self, pos: (u32, u32)) -> bool {
        let (px, py) = pos;
        for (x, y) in &self.snake {
            if (px == *x && py == *y) {
                return true;
            }
        }
        false
    }

    fn should_eat(&self, pos: (u32, u32)) -> bool {
        let (px, py) = pos;
        let (x, y) = &self.snake.get(0).unwrap();
        (*x == px && *y == py)
    }
    fn shift_snake(&mut self, new_head: (u32, u32)) {
        let top = self.snake.len();
        for i in (1..top).rev() {
            let tmp = self.snake.get(i-1).unwrap().clone();
            *self.snake.get_mut(i).unwrap() = tmp;
        }
        *self.snake.get_mut(0).unwrap() = new_head;
    }
    fn reset_food(&mut self) {
        let possible_x: Vec<u32> = (0..self.width).collect();
        let possible_y: Vec<u32> = (0..self.height).collect();

        while true {
            let new_pos = (
                self.rand.choose(&possible_x),
                self.rand.choose(&possible_y)
            );
            if let (Some(x), Some(y)) = new_pos {
                if !self.pos_in_snake((*x,*y)) {
                    self.food = (*x,*y);
                    return;
                }
            }
        }
    }
    fn grow_snake(&mut self) {
        let len = self.snake.len();
        let last_pos = self.snake.get(len-1).unwrap().clone();
        self.snake.push(last_pos);
    }
}


impl Entity<GameBoardEvent> for GameBoard { 
    fn update(&mut self, _args: &UpdateArgs) -> GameBoardEvent {
        self.time_since_update += _args.dt;
        if self.time_since_update > 0.5 {
            self.time_since_update = 0.0;
            let new_head = self.get_next_head();
            if self.is_bad_move(new_head) {
                let len = self.snake.len() as u32;
                return GameBoardEvent::Fail(len);
            } else {
                self.shift_snake(new_head);            
            }

            if self.should_eat(self.food) {
                self.grow_snake();
                self.reset_food();
            }

        }
        GameBoardEvent::NoEvent
    }
}

impl Renderable for GameBoard {
    fn render(&self, 
              _t: &math::Matrix2d,
              _gl: &mut GlGraphics,
              _glyph: &mut GlyphCache, 
              _args: &RenderArgs,
              _pos: &PosArgs,
              _sheet: Option<&TileSheet>) {
        //we are going to make the board a checker board
        clear([1.0,1.0,1.0,1.0], _gl);


        let (cw, ch) = (
            _pos.win_w as f64 / self.width as f64,
            _pos.win_h as f64 / self.height as f64
        );

        let sqr = rectangle::square(0.0, 0.0, 1.0);

        //draw the board

        for xc in 0..self.width {
            for yc in 0..self.height {
                let col = if (xc + yc) % 2 == 0 {
                    [0.2,0.2,0.2,1.0]
                } else {
                    [0.24,0.24,0.24,1.0]
                };
                rectangle(col, sqr, 
                          _t.trans(xc as f64 * cw,
                                   yc as f64 * ch)
                          .scale(cw, ch),
                          _gl);        
            }
        }

        //draw the snake
        let snake_col = [0.0, 0.5,0.0,1.0];
        for (xc, yc) in &self.snake {
            rectangle(snake_col, 
                      sqr, 
                      _t.trans(*xc as f64 * cw,
                               *yc as f64 * ch)
                      .scale(cw, ch),
                      _gl);
        }


        //draw the food

        let (food_x, food_y) = self.food;
        let food_col = [0.5,0.0,0.0,1.0];

        rectangle(food_col, 
                  sqr, 
                  _t.trans(food_x as f64 * cw,
                           food_y as f64 * ch)
                  .scale(cw, ch),
                  _gl);
    }
}

impl MouseInteract<GameBoardEvent> for GameBoard {
    fn mouse_move(&mut self, _pos: &PosArgs) -> GameBoardEvent {
        GameBoardEvent::NoEvent
    }

    fn click(&mut self, 
             _pos: &PosArgs, 
             _btn: MouseButton) -> GameBoardEvent {
        GameBoardEvent::NoEvent
    }

    fn resize(&mut self, _pos: &PosArgs) -> GameBoardEvent {
        GameBoardEvent::NoEvent
    }

    fn in_bound(&self, _pos: &PosArgs) -> bool {
        true
    }
}

impl KeyInteract<GameBoardEvent> for GameBoard {
    fn key_press(&mut self, key: Key) -> GameBoardEvent {
        match key {
            Key::W | Key::Up => {
                self.dir = Direction::Up;
            },
            Key::A | Key::Left => {
                self.dir = Direction::Left;
            },
            Key::S | Key::Down => {
                self.dir = Direction::Down;
            },
            Key::D | Key::Right => {
                self.dir = Direction::Right;
            },  
            _ => {}
        }
        GameBoardEvent::NoEvent
    }
    fn key_release(&mut self, key: Key) -> GameBoardEvent {
        GameBoardEvent::NoEvent
    }
}

