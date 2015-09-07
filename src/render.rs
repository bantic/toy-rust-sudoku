use sdl2::rect::{Rect,Point};
use sdl2::pixels::{Color};
use sdl2::render::{Renderer};

pub struct Drawer<'a> {
  renderer: Renderer<'a>,
  screen_size: (u32, u32)
}

impl<'a> Drawer<'a> {
  pub fn new(renderer: Renderer<'a>) -> Drawer {
      let screen_size = renderer.window().unwrap().drawable_size();
      Drawer {
          renderer: renderer,
          screen_size: screen_size
      }
  }

  pub fn draw(&mut self) {
      println!("render");
      self.renderer.clear();

      self.renderer.set_draw_color(Color::RGB(0,255,0));
      let rect = Rect::new_unwrap(50, 75, 10, 20);
      self.renderer.fill_rect(rect);

      self.renderer.set_draw_color(Color::RGB(0,0,0));

      self.draw_grid();

      self.renderer.present();
  }

  pub fn draw_grid(&mut self) {
      let cols:i32 = 3;
      let rows:i32 = 3;
      let width = self.screen_size.0 as i32;
      let height = self.screen_size.1 as i32;
      let col_width = width / 3;
      let row_height = height / 3;

      self.renderer.set_draw_color(Color::RGB(255, 255, 255));

      for c in (0..cols) {
          let x = c * col_width;
          self.renderer.draw_line(Point::new(x,0), Point::new(x,height));
          println!("line from ({},{}) -> ({},{})",x,0,x,height);
      }
      for r in (0..rows) {
          let y = r * row_height;
          self.renderer.draw_line(Point::new(0,y), Point::new(width,y));
          println!("line from ({},{}) -> ({},{})",0,y,width,y);
      }

      self.renderer.set_draw_color(Color::RGB(0, 0, 0));
  }

  pub fn draw_inner_grid(&mut self, x:u32, y:u32) {
  }
}

