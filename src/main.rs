
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

//use piston::window::WindowSettings;
//use piston::event_loop::*;
use piston::input::*;
//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache};
use app_base::*;
use graphics::*;

use art::*;


//test stuff
//should be moved to its own folder

struct Square {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    col: [f32; 4] 
}

struct TestApp {
    sqr: Square,
    sheet: TileSheet,
    btn: button::Button
}

impl Renderable for Square {
    fn render(&self, 
              _t: &math::Matrix2d,
              _gl: &mut GlGraphics,
              _glyph: &mut GlyphCache, 
              _args: &RenderArgs,
              _pos: &PosArgs,
              _sheet: Option<&TileSheet>) {

        match _sheet {
            Some(sheet) => {
                let transform = _t.trans(self.x as f64, 
                                         self.y as f64);
                sheet.render_tile(0,0,
                                   self.w as f64, 
                                   self.h as f64,
                                   &transform,
                                   _gl);
            },
            None => {
                let sqr = rectangle::square(0.0, 0.0, 1.0);
                rectangle(self.col, sqr,
                          _t.trans(self.x as f64, 
                                   self.y as f64)
                          .scale(self.w as f64,
                                 self.h as f64), 
                          _gl);
            }
        }
    }
}

impl App for TestApp {
    fn new() -> Self {
        TestApp {
            sqr: Square {
                x: 5,
                y: 5,
                w: 100,
                h: 100,
                col: [1.0, 0.0, 0.0, 1.0]
            },
            sheet: TileSheet::new("res/sample.jpg".to_string(), 
                                  3,3),
            btn: button::Button {
                pos: button::ButtonPos::Centered(100),
                w: 100,
                h: 100,
                normal_skin: button::ButtonSkin::Text((
                            "Not Hover".to_string(),
                            24,
                            [1.0,1.0,1.0,1.0],
                            [0.2,0.2,0.2,1.0]
                        )),
                hover_skin: button::ButtonSkin::Text((
                            "Hover".to_string(),
                            30,
                            [1.0,1.0,1.0,1.0],
                            [0.0,0.0,0.0,1.0]
                        )),
                state: button::ButtonState::Normal
            }
        }
    }
    fn render(&self,
              gl: &mut GlGraphics,
              glyph: &mut GlyphCache,
              args: &RenderArgs, 
              pos: &PosArgs) {
        gl.draw(args.viewport(), |c, gl| {

            clear([0.0,1.0,0.0,1.0], gl);
            self.sqr.render(
                &c.transform,
                gl,
                glyph,
                args,
                pos,
                Some(&self.sheet)
            );
            render_text("Hello World", 
                        24,
                        [0.2,0.2,0.2,1.0],
                        &c.transform
                          .trans(5.0,25.0),
                        gl, 
                        glyph);

            self.btn.render(
                    &c.transform,
                    gl,
                    glyph,
                    args,
                    pos,
                    None
                );
        }); 
    }

    fn mouse_move(&mut self, _pos: &PosArgs) {
        let event = self.btn.mouse_move(_pos);
    }

    fn click(&mut self, _pos: &PosArgs, _btn: MouseButton) {
        if let button::ButtonEvent::Clicked = 
                    self.btn.click(_pos, _btn) {
            println!("clicked");
        }
    }
}


fn main() {
    let settings = GameSettings {
        width: 400,
        height: 400,
        game_title: "game engine".to_string(),
        font: "res/FiraSans-Regular.ttf".to_string()
    };

    start::<TestApp>(settings);
}
