use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use piston::input::*;
use app_base::*;
use art::TileSheet;
use button;

pub enum TitleEvent {
    PlayGameButtonPress,
    NoEvent
}

impl GameEvent for TitleEvent {
    //placeholder
}

pub struct TitleScreen {
    playbtn: button::Button
}

impl TitleScreen {
    pub fn new() -> Self {
        TitleScreen {
            playbtn: button::Button {
               pos: button::ButtonPos::Centered(100),
               w: 200,
               h: 50,
               normal_skin: button::ButtonSkin::Text((
                        "Play Game".to_string(),
                        24,
                        [1.0,1.0,1.0,1.0],
                        [0.3,0.3,0.3,1.0]
                       )),
               hover_skin: button::ButtonSkin::Text((
                       "Play Game".to_string(),
                       24,
                       [1.0, 1.0, 1.0, 1.0],
                       [0.5, 0.5, 0.5,1.0]
                       )),
               state: button::ButtonState::Normal
            }
        }
    }
}

impl Renderable for TitleScreen { 
    fn render(&self, 
              _t: &math::Matrix2d,
              _gl: &mut GlGraphics,
              _glyph: &mut GlyphCache, 
              _args: &RenderArgs,
              _pos: &PosArgs,
              _sheet: Option<&TileSheet>) {
       clear([0.1,0.1,0.1,1.0], _gl);
           
       self.playbtn.render(
                _t,
                _gl,
                _glyph,
                _args,
                _pos,
                None
           ); 
    }
}

impl Entity<TitleEvent> for TitleScreen {
    fn update(&mut self, _args: &UpdateArgs) -> TitleEvent {
        TitleEvent::NoEvent
    }
}

impl MouseInteract<TitleEvent> for TitleScreen { 
    fn mouse_move(&mut self, _pos: &PosArgs) -> TitleEvent {
        self.playbtn.mouse_move(_pos);   
        
        TitleEvent::NoEvent
    }


    fn click(&mut self, 
             _pos: &PosArgs, 
             _btn: MouseButton) -> TitleEvent {
        
        let event = self.playbtn.click(_pos, _btn);

        match event {
            button::ButtonEvent::Clicked => {           
                TitleEvent::PlayGameButtonPress
            },
            _ => {
                TitleEvent::NoEvent
            }
        }
    }

    fn resize(&mut self, _pos: &PosArgs) -> TitleEvent {
        TitleEvent::NoEvent
    }

    fn in_bound(&self, _pos: &PosArgs) -> bool {
        true
    }
}

impl KeyInteract<TitleEvent> for TitleScreen {
    fn key_press(&mut self, key: Key) -> TitleEvent {
        TitleEvent::NoEvent
    }
    fn key_release(&mut self, key: Key) -> TitleEvent {
        TitleEvent::NoEvent
    }
}
