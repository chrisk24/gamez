extern crate opengl_graphics;

use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::*;
use art::{render_text, TileSheet};
use app_base::*;
use piston::input::*;
use graphics::character::CharacterCache;

pub enum ButtonEvent {
    Clicked,
    Hover,
    NoEvent
}

impl GameEvent for ButtonEvent {
    //placeholder
}

pub enum ButtonPos {
    Fixed((u32, u32)),
    Centered(u32),
    CenteredOffset((i32, u32))
}

pub enum ButtonState {
    Normal,
    Hover
}

pub enum ButtonSkin {
    Text((String, //label
          u32, //font size
          [f32; 4],  //font color
          [f32; 4])) // bg color
}

pub struct Button {
    pub pos: ButtonPos,
    pub w: u32,
    pub h: u32,
    pub normal_skin: ButtonSkin,
    pub hover_skin: ButtonSkin,
    pub state: ButtonState
}

impl Button {
    fn get_left_x(&self, screen_width: u32) -> u32 {
        let x = (screen_width as i32 - self.w as i32) / 2;
        if x > 0 {
            x as u32
        } else {
            0
        }
    }

    fn get_upper_left(&self, screen_width: u32) -> (u32, u32) {
        match self.pos {
            ButtonPos::Fixed((x,y)) => (x,y),
            ButtonPos::Centered(y) => 
                (self.get_left_x(screen_width), y),
                ButtonPos::CenteredOffset((xoff, y)) => {
                    let xpos = self.get_left_x(screen_width) 
                        as i32 + xoff;
                    let xpos = if xpos > 0 {
                        xpos as u32
                    } else {
                        0
                    };
                    (xpos,y)}
        }
    }
}

impl Renderable for Button {

    fn render(&self, 
              _t: &math::Matrix2d,
              _gl: &mut GlGraphics,
              _glyph: &mut GlyphCache, 
              _args: &RenderArgs,
              _pos: &PosArgs,
              _sheet: Option<&TileSheet>) {


        let skin = match &self.state {
            ButtonState::Normal => &self.normal_skin,
            ButtonState::Hover => &self.hover_skin
        };

        match skin {
            ButtonSkin::Text((label, 
                              font_size, 
                              font_color, 
                              bg_color )) => {
                

                let rect = rectangle::square(0.0, 0.0, 1.0);

                let screen_width = _pos.win_w;

                let (xpos, ypos) = self.get_upper_left(screen_width);
                let base_pos_transform = _t.trans(
                    xpos as f64,
                    ypos as f64
                );

                rectangle(*bg_color, rect,
                          base_pos_transform.scale(
                              self.w as f64,
                              self.h as f64
                          ),
                          _gl);

                let text_width = 
                    match _glyph.width(*font_size,label) {
                        Ok(x) => x,
                        _ => 0.0
                    };

                render_text(label, *font_size, *font_color,
                            &base_pos_transform
                            .trans((self.w as f64 - text_width)/2.0,
                                   *font_size as f64), 
                            _gl, _glyph);
            } 
        }
    }
}

impl MouseInteract<ButtonEvent> for Button {
    fn mouse_move(&mut self, _pos: &PosArgs) -> 
        ButtonEvent {

            if self.in_bound(_pos) {
                self.state = ButtonState::Hover;
                ButtonEvent::Hover
            } else {
                self.state = ButtonState::Normal;
                ButtonEvent::NoEvent
            }
        }

    fn click(&mut self, 
             _pos: &PosArgs, 
             _btn: MouseButton ) -> ButtonEvent {
        if self.in_bound(_pos) {
            ButtonEvent::Clicked
        } else {
            ButtonEvent::NoEvent
        }
    }

    fn resize(&mut self, _pos: &PosArgs) -> ButtonEvent {
        ButtonEvent::NoEvent
    }

    fn in_bound(&self, _pos: &PosArgs) -> bool {
        let (xbound, ybound) = self.get_upper_left(_pos.win_w);

        (_pos.mouse_x >= xbound &&
         _pos.mouse_x <= xbound + self.w &&
         _pos.mouse_y >= ybound &&
         _pos.mouse_y <= ybound + self.h)
    }
}



