use ::rand;
use macroquad::prelude::*;
use rand::thread_rng;

mod minefield;
mod rendering;
use minefield::*;
use rendering::*;

const BACKGROUND: Color = DARKGRAY;

#[macroquad::main("Minesweeper")]
async fn main() {
    let minefield_dims = XY { x: 8, y: 8 };
    let mines = 10;
    let mut minefield = Minefield::new(minefield_dims);

    let mut renderer = Frontend::default();
    let mut gamestate = GameState::PreClick;

    loop {
        renderer.draw(&minefield, BACKGROUND);

        use GameState::*;
        match gamestate {
            PreClick => {
                if is_mouse_button_released(MouseButton::Left) {
                    if let Some(safe_pos) = renderer.mouse_grid_pos() {
                        minefield.populate(&mut thread_rng(), mines, safe_pos);
                        // Ignore result as `populate` guarantees the first position is safe.
                        let _ = minefield.open(safe_pos);
                        gamestate = GameState::Playing;
                    }
                }
            }

            Playing => {
                if is_mouse_button_released(MouseButton::Left) {
                    if let Some(open_pos) = renderer.mouse_grid_pos() {
                        let opened_mine = minefield.open(open_pos);
                        if opened_mine {
                            eprintln!("KABOOM!");
                            gamestate = GameState::Lost;
                        } else if minefield.is_clear() {
                            eprintln!("VICTORY!");
                            gamestate = GameState::Won;
                        };
                    }
                }

                if is_mouse_button_released(MouseButton::Right) {
                    if let Some(flag_pos) = renderer.mouse_grid_pos() {
                        minefield.toggle_flag(flag_pos);
                    }
                }
            }

            Won | Lost => {
                /* Draw game over text over grid and prompt to play again */
                todo!("Make game over menu (:");
            }
        }

        next_frame().await;
    }
}

enum GameState {
    // State before the player has dug the first square.
    PreClick,
    Playing,
    Won,
    Lost,
}
