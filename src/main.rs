use macroquad::prelude::*;

#[macroquad::main("Flappy Macro")]

async fn main() {
  loop {
    clear_background(DARKPURPLE);
    next_frame().await
  }
}
