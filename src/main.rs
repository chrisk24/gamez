
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

/*
 *All of the module imports should go here
 */
mod app_base;
mod art;
mod button;
mod title_screen;
mod game_board;
//use piston::window::WindowSettings;
//use piston::event_loop::*;
use piston::input::*;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache};
use app_base::*;
use graphics::*;

use art::*;
use title_screen::*;
use game_board::*;
/*
 * How does one make a game with this engine?
 * 1. Set up the main App to have a game state enum
 * 2. Pass Arguments to game states on events
 * 3. Game events have all data for their state and can talk to the
 *    main game only through events in the implemented methods
 * 4. 
 *
 */

enum GameState {
    Title(TitleScreen),
    Game(GameBoard)
}

struct GameApp {
    state: GameState
}

impl App for GameApp {
    fn new() -> Self {
        GameApp {
            state: GameState::Title(TitleScreen::new(0))
        }
    }
    fn render(&self,
              gl: &mut GlGraphics,
              glyph: &mut GlyphCache,
              args: &RenderArgs, 
              pos: &PosArgs) {
        gl.draw(args.viewport(), |c, gl| {
            match &self.state {
                GameState::Title(title_screen) => {
                    title_screen.render(
                            &c.transform,
                            gl,
                            glyph,
                            args,
                            pos,
                            None
                        );
                },
                GameState::Game(game) => {
                    game.render(
                            &c.transform,
                            gl, 
                            glyph,
                            args,
                            pos,
                            None
                        );
                }
            }
        }); 
    }

    fn update(&mut self, _args: &UpdateArgs, _pos: &PosArgs) {
        let mut new_state: Option<GameState> = None;
        match &mut self.state {
            GameState::Title(ts) => {
                ts.update(_args);
            },
            GameState::Game(game) => {
                let e = game.update(_args);
                match e {
                    GameBoardEvent::Fail(len) => {
                        println!("Fail! {}", len);
                        new_state = Some(GameState::Title(
                                         TitleScreen::new(len)));
                    }
                    _ => {}
                }
            }
        }

        if let Some(new_state) = new_state {
            self.state = new_state;
        }
    }

    fn mouse_move(&mut self, _pos: &PosArgs) {
        match &mut self.state {
            GameState::Title(title_screen) => {
                title_screen.mouse_move(_pos);
            },
            GameState::Game(game) => {
                game.mouse_move(_pos);
            }
        }
    }

    fn click(&mut self, _pos: &PosArgs, _btn: MouseButton) {
        let mut new_state: Option<GameState> = None; 
        
        match &mut self.state {
            GameState::Title(ts) => {
                let e = ts.click(_pos, _btn);
                if let TitleEvent::PlayGameButtonPress = e {
                    println!("Play Game!");
                    new_state = Some(GameState::Game(
                                    GameBoard::new(10,10)));
                }
            },
            GameState::Game(game) => {
                game.click(_pos, _btn);
            }
        }

        if let Some(state) = new_state {
            self.state = state;
        }
    }

    fn key_press(&mut self, key: Key) {
        match &mut self.state {
            GameState::Title(ts) => {
                ts.key_press(key);
            },
            GameState::Game(game) => {
                game.key_press(key);
            }
        }
    }

    fn key_release(&mut self, key: Key) {
        match &mut self.state {
            GameState::Title(ts) => {
                ts.key_release(key);
            },
            GameState::Game(game) => {
                game.key_release(key);
            }
        }
    }
}


fn main() {
    let settings = GameSettings {
        width: 400,
        height: 400,
        game_title: "game engine - snake".to_string(),
        font: "res/FiraSans-Regular.ttf".to_string()
    };

    start::<GameApp>(settings);
}
