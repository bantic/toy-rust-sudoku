use sdl2::rect::{Rect,Point};
use sdl2::pixels::{Color};
use sdl2::render::{Renderer};

pub struct Drawer<'a> {
  renderer: Renderer<'a>,
  screen_size: (u32, u32),
  grid: Grid
}

pub struct Grid {
    rows: i32,
    cols: i32,
    origin: (i32, i32),
    width: u32,
    height: u32,
    cells: Vec<GridCell>
}

impl Grid {
    pub fn new(origin: (i32, i32), width: u32, height: u32) -> Grid {
        let rows:i32 = 9;
        let cols:i32 = 9;
        let mut cells = vec![];

        let grid_cell_width = width / (cols as u32);
        let grid_cell_height = height / (rows as u32);
        let x = origin.0;
        let y = origin.1;

        for r in (0..rows) {
            for c in (0..cols) {
                let _x = x + (c as i32) * (grid_cell_width as i32);
                let _y = y + (r as i32) * (grid_cell_height as i32);

                let grid_cell = GridCell {
                    origin: (_x, _y),
                    width: grid_cell_width,
                    height: grid_cell_height,
                    border_width: 3,
                    border_color: Color::RGB(0,100,100)
                };
                cells.push(grid_cell);
            }
        }

        Grid {
            rows: rows,
            cols: cols,
            origin: origin,
            width: width,
            height: height,
            cells: cells
        }
    }
}

pub struct GridCell {
    width: u32,
    height: u32,
    origin: (i32, i32),
    border_width: u32,
    border_color: Color
}

impl<'a> Drawer<'a> {
  pub fn new(renderer: Renderer<'a>) -> Drawer {
      let screen_size = renderer.window().unwrap().drawable_size();
      let margin = 10;
      let grid = Grid::new(
          (margin, margin),
          screen_size.0 - (margin as u32)*2,
          screen_size.1 - (margin as u32)*2
      );
      Drawer {
          renderer: renderer,
          screen_size: screen_size,
          grid: grid
      }
  }

  pub fn draw(&mut self) {
      self.renderer.clear();

      self.draw_grid();

      self.renderer.present();
  }

  pub fn draw_grid(&mut self) {
      for grid_cell in &mut self.grid.cells {
          //println!("Drawing grid cell ({},{}) {}x{}", grid_cell.origin.0, grid_cell.origin.1, grid_cell.width,grid_cell.height);
          //self.draw_grid_cell(grid_cell);
          let prev_color = self.renderer.draw_color();

          self.renderer.set_draw_color(grid_cell.border_color);

          let x = grid_cell.origin.0;
          let y = grid_cell.origin.1;

          // fixme there is probably a better way to deal with i32 v u32 than
          // to recast all the variables
          let width = grid_cell.width;
          let iwidth = width as i32;
          let height = grid_cell.height;
          let iheight = height as i32;
          let border_width = grid_cell.border_width;
          let iborder_width = border_width as i32;

          // draw rectangle for each side line
          self.renderer.fill_rect(Rect::new_unwrap(x, y, width, border_width));
          self.renderer.fill_rect(Rect::new_unwrap(x, y, border_width, height));
          self.renderer.fill_rect(Rect::new_unwrap(x+iwidth-iborder_width, y,
                                                   border_width, height));
          self.renderer.fill_rect(Rect::new_unwrap(x,y+iheight-iborder_width,
                                                   width, border_width));

          // restore prev color
          self.renderer.set_draw_color(prev_color);
      }
  }

  pub fn draw_grid_cell(&mut self, grid_cell: &GridCell)  {
      let prev_color = self.renderer.draw_color();

      self.renderer.set_draw_color(grid_cell.border_color);

      let x = grid_cell.origin.0;
      let y = grid_cell.origin.1;

      // fixme there is probably a better way to deal with i32 v u32 than
      // to recast all the variables
      let width = grid_cell.width;
      let iwidth = width as i32;
      let height = grid_cell.height;
      let iheight = height as i32;
      let border_width = grid_cell.border_width;
      let iborder_width = border_width as i32;

      // draw rectangle for each side line
      self.renderer.fill_rect(Rect::new_unwrap(x, y, width, border_width));
      self.renderer.fill_rect(Rect::new_unwrap(x, y, border_width, height));
      self.renderer.fill_rect(Rect::new_unwrap(x+iwidth-iborder_width, y,
                                               border_width, height));
      self.renderer.fill_rect(Rect::new_unwrap(x,y+iheight-iborder_width,
                                               width, border_width));

      // restore prev color
      self.renderer.set_draw_color(prev_color);
  }
}
