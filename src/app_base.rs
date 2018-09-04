extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

/*
 *All of the module imports should go here
 */
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::{math};
use art::{TileSheet};

pub struct PosArgs {
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub win_w: u32,
    pub win_h: u32
}

pub struct GameSettings {
    pub width: u32,
    pub height: u32,
    pub game_title: String,
    pub font: String
}

pub trait GameEvent {
    //placeholder
}

pub trait App {
    fn new() -> Self;

    fn render(&self,
              _gl: &mut GlGraphics,
              _glyph: &mut GlyphCache,
              _args: &RenderArgs, 
              _pos: &PosArgs) {

    }
    fn update(&mut self, 
              _args: &UpdateArgs, 
              _pos: &PosArgs) {

    }
    fn mouse_move(&mut self, 
                  _pos: &PosArgs) {

    }
    fn click(&mut self, 
             _pos: &PosArgs, 
             _btn: MouseButton) {
        
    }
    fn resize(&mut self,
              _pos: &PosArgs) {
        
    }
}


//this should be updated to return a gameevent
pub trait Entity<T: GameEvent> {
    fn update(&mut self, 
              _args: &UpdateArgs) -> Option<T> {
        None        
    }
}

pub trait Renderable {
    fn render(&self, 
              _t: &math::Matrix2d,
              _gl: &mut GlGraphics,
              _glyph: &mut GlyphCache, 
              _args: &RenderArgs,
              _pos: &PosArgs,
              _sheet: Option<&TileSheet>) {

    }
}


//these should be updated to return a gameevent
pub trait MouseInteract<T: GameEvent> {
    fn mouse_move(&mut self, 
                  _pos: &PosArgs) -> Option<T> {
        None
    }

    fn click(&mut self, 
             _pos: &PosArgs, 
             _btn: MouseButton) -> Option<T> {
        None
    } 

    fn resize(&mut self, 
              _pos: &PosArgs) -> Option<T> {
        None
    }
}

pub fn start<A: App>(settings: GameSettings) {
    let opengl = OpenGL::V3_2;

    let mut window_pos = PosArgs {
        mouse_x: 0,
        mouse_y: 0,
        win_w: settings.width,
        win_h: settings.height
    };

    let mut window: Window = WindowSettings::new(
            settings.game_title,
            [window_pos.win_w, window_pos.win_h]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap(); //will panic if doesn't start correctly which is OK here

    let mut glyph = GlyphCache::new(&settings.font, (),
                                    TextureSettings::new())
                                    .unwrap();
    //initialize the app here
    let mut app: A = A::new();

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    

    //main game loop
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            //render
            app.render(&mut gl, &mut glyph, &r, &window_pos);
        }
        
        if let Some(r) = e.update_args() {
            //update
            app.update(&r, &window_pos);
        }

        if let Some(Button::Mouse(btn)) = e.press_args() {
            //mouse press
            app.click(&window_pos, btn);
        }

        e.mouse_cursor(|x,y| {
            window_pos.mouse_x = x as u32;
            window_pos.mouse_y = y as u32; 
            //mouse move
            app.mouse_move(&window_pos);
        });

        e.resize(|w,h| {
            window_pos.win_w = w;
            window_pos.win_h = h;
            //resize
            app.resize(&window_pos);
        });
    }
}

