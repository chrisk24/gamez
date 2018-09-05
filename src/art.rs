/*Handler for art in the game engine
 *
 */
extern crate opengl_graphics;
extern crate image;

use opengl_graphics::{GlGraphics, 
                      Texture, 
                      TextureSettings, 
                      GlyphCache};
use graphics::*;
use art::image::{GenericImage};


pub fn render_text(txt: &str, 
                   font_size: u32,
                   col: [f32; 4],
                   t: &math::Matrix2d,
                   gl: &mut GlGraphics,
                   glyph: &mut GlyphCache) {
    let txt = text::Text::new_color(col, font_size)
                .draw(txt, 
                      glyph, 
                      &DrawState::default(),
                      *t, 
                      gl);

    match txt {
        Ok(x) => {},
        Err(e) => {}
    }
}


pub struct TileSheet {
   width: u32,
  height: u32,
   sheet: Vec<Texture> 
}

impl TileSheet {    
    fn get_index(x: u32, y: u32, w: u32) -> u32{
        x + y * w
    }

    pub fn render_tile(&self, 
                   x: u32, 
                   y: u32, 
                   scaled_width: f64,
                   scaled_height: f64,
                   t: &math::Matrix2d,
                   gl: &mut GlGraphics) {

        let index = TileSheet::get_index(x,y,self.width) as usize;
        match &self.sheet.get(index) {
            Some(tile) => {
                let img: &Texture = tile;
                
                let (scale_x, scale_y) = 
                        (scaled_width / tile.get_width() as f64,
                         scaled_height / tile.get_height() as f64);

                image(img, 
                      t.scale(scale_x, scale_y), 
                      gl);
            },
            None => {
            
            }
        }
    }

    pub fn new(path: String, width: u32, height: u32) -> Self {
        let mut sheet = Vec::new();
        
        let base_img = image::open(path).unwrap();

        let (image_width, image_height) = (base_img.width(),
                                           base_img.height());


        for i in 0..width*height {
            
            let x = i % width;
            let y = i / width;

            let subimg = base_img.clone()
                            .crop(x*(image_width/width),
                                  y*(image_height/height),
                                  (image_width/width),
                                  (image_height/height))
                            .to_rgba();
            let tile: Texture = Texture::from_image(
                    &subimg,
                    &TextureSettings::new()
                );
            sheet.push(tile);

        }


        TileSheet {
            width: width,
            height: height,
            sheet: sheet
        }
    }
}

