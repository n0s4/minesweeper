use crate::minefield::{Cell, CellState, Minefield};
use macroquad::prelude::*;

#[derive(Default, Debug)]
pub struct Frontend {
    /// The actual size of each grid block, calculated to be as big as possible while fitting in the screen.
    cell_size: f32,
    /// Actual position of (the top left corner of) the grid on the screen. This means padding will be added if the window ratio
    /// Doesn't match up with the grid ratio.
    grid_pos: XY<f32>,
    /// The actual size of the grid on the window.
    grid_dims: XY<f32>,
}

impl Frontend {
    /// Updates the renderer and draws the minefield on the given background colour.
    pub fn draw(&mut self, field: &Minefield, bg: Color) {
        self.update(field.dimensions());

        clear_background(bg);

        let mut draw_pos = XY {
            x: self.grid_pos.x,
            y: self.grid_pos.y,
        };
        for row in &field.cells {
            for cell in row {
                cell.draw(draw_pos.x, draw_pos.y, self.cell_size);
                draw_pos.x += self.cell_size;
            }
            draw_pos.x = self.grid_pos.x;
            draw_pos.y += self.cell_size;
        }
    }

    /// Translates the mouse's position into a grid position, if the mouse is inside the grid.
    pub fn mouse_grid_pos(&self) -> Option<XY<usize>> {
        let (mousex, mousey) = mouse_position();

        Some(XY {
            x: ((mousex - self.grid_pos.x) / self.cell_size) as usize,
            y: ((mousey - self.grid_pos.y) / self.cell_size) as usize,
        })
        .filter(|_| self.mouse_in_grid())
    }

    pub fn mouse_in_grid(&self) -> bool {
        let (mousex, mousey) = mouse_position();

        [
            mousex > self.grid_pos.x,
            mousey > self.grid_pos.y,
            mousex < self.grid_pos.x + self.grid_dims.x,
            mousey < self.grid_pos.y + self.grid_dims.y,
        ]
        .into_iter()
        .all(|cond| cond == true)
    }

    /// Updates the rendering info.
    fn update(&mut self, minefield_dims: XY<usize>) {
        let (scrw, scrh) = (screen_width(), screen_height());

        self.cell_size = (scrw / minefield_dims.x as f32).min(scrh / minefield_dims.y as f32);

        let gridw = self.cell_size * minefield_dims.x as f32;
        let gridh = self.cell_size * minefield_dims.y as f32;

        // accounts padding on each side of the screen
        self.grid_pos.x = (scrw - gridw) / 2.0;
        self.grid_pos.y = (scrh - gridh) / 2.0;

        self.grid_dims = XY { x: gridw, y: gridh };
    }
}

impl Cell {
    pub fn draw(&self, x: f32, y: f32, cell_size: f32) {
        let cell_colour = match self.state {
            CellState::Flagged => ORANGE,
            CellState::Open(_) if self.is_mine => RED,
            CellState::Open(_) => GRAY,
            CellState::Closed => LIGHTGRAY,
        };

        draw_rectangle(x, y, cell_size, cell_size, cell_colour);
        draw_rectangle_lines(x, y, cell_size, cell_size, 1.0, BLACK);

        // We don't want to show mine numbers on bombs.
        if self.is_mine {
            return;
        }

        // Draw amount of neighbouring mines if there are any.
        if let CellState::Open(neighbouring_mines @ 1..) = self.state {
            let text_colour = match neighbouring_mines {
                1 => GREEN,
                2 => BLUE,
                3 => RED,
                _ => PURPLE,
            };

            draw_text(
                &neighbouring_mines.to_string(),
                x + cell_size * 0.3,
                y + cell_size * 0.8,
                cell_size,
                text_colour,
            );
        }
    }
}
