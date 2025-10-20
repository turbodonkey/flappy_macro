use macroquad::{audio::play_sound_once, prelude::*};

const SCREEN_WIDTH: i32 = 80;
//const SCREEN_HEIGHT: i32 = 50;
//const FRAME_DURATION: f32 = 60.0;
const GRAVITY: f32 = 10.0;
const FLAP_POWER: f32 = 6.0;

enum GameMode {
  Menu,
  Playing,
  Paused,
  End,
}

struct Obstacle {
  x: i32,
  gap_y: i32,
  size: i32,
}

struct Player {
  x: f32,
  y: f32,
  velocity: f32,
}

struct State {
  mode: GameMode,
  player: Player,
  frame_time: f32,
  score: i32,
  obstacle: Obstacle,
}

impl State {
  fn new() -> Self {
    Self {
      mode: GameMode::Menu,
      player: Player::new(65.0, 100.0),
      frame_time: 0.0,
      score: 0,
      obstacle: Obstacle::new(SCREEN_WIDTH, 0),
    }
  }

  fn reset(&mut self) {
    self.mode = GameMode::Playing;
    self.player.reset();
    self.frame_time = 0.0;
    self.score = 0;
    //self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
  }
}

impl Obstacle {
  fn new(x: i32, score: i32) -> Self {
    Obstacle {
      x,
      gap_y: rand::gen_range(10, 40),
      size: i32::max(2, 20 - score),
    }
  }
}

impl Player {
  fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
      velocity: 0.0,
    }
  }

  fn reset(&mut self) {
    self.x = 65.0;
    self.y = 100.0;
    self.velocity = 0.0;
  }

  fn flap(&mut self) {
    self.velocity -= FLAP_POWER;
  }

  fn tick(&mut self, delta_time: f32) {
    self.velocity += GRAVITY * delta_time;

    if self.velocity < -5.0 {
      self.velocity = -5.0;
    }

    self.y += self.velocity;

    /* test max height */
    if self.y < 11.0 {
      self.y = 11.0;
      self.velocity /= 2.0;
    }
  }

  fn render(&mut self) {
    draw_circle(self.x, self.y, 16.0, YELLOW);
  }
}

#[macroquad::main("Flappy Macro")]
async fn main() {
  let mut state = State::new();
  rand::srand(miniquad::date::now() as u64);

  loop {
    let delta_time = get_frame_time();

    match state.mode {
      GameMode::Menu => {
        clear_background(DARKPURPLE);

        let text = "Press SPACE to Play";
        let font_size = 20.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
          text,
          (screen_width() / 2.0) - (text_dims.width / 2.0),
          screen_height() / 2.0,
          20.0,
          WHITE,
        );

        if is_key_pressed(KeyCode::Space) {
          state.mode = GameMode::Playing;
        }
      }
      GameMode::Playing => {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Space) {
          state.player.flap();
        } else if is_key_pressed(KeyCode::P) {
          state.mode = GameMode::Paused;
        }

        state.player.tick(delta_time);

        if state.player.y > (screen_height() - 11.0) {
          state.mode = GameMode::End;
        }

        state.player.render();
      }
      GameMode::Paused => {
        clear_background(DARKPURPLE);

        let text = "Paused";
        let font_size = 20.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
          text,
          (screen_width() / 2.0) - (text_dims.width / 2.0),
          screen_height() / 3.0,
          20.0,
          WHITE,
        );

        let text = "Press SPACE to Continue or 'Q' to Quit";
        let font_size = 20.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
          text,
          (screen_width() / 2.0) - (text_dims.width / 2.0),
          screen_height() / 2.0,
          20.0,
          WHITE,
        );

        if is_key_pressed(KeyCode::Space) {
          state.mode = GameMode::Playing;
        } else if is_key_pressed(KeyCode::Q) {
          break;
        }
      }
      GameMode::End => {
        clear_background(DARKPURPLE);

        let text = "GAME OVER";
        let font_size = 20.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
          text,
          (screen_width() / 2.0) - (text_dims.width / 2.0),
          screen_height() / 3.0,
          20.0,
          WHITE,
        );

        let text = "Press SPACE to Play or 'Q' to Quit";
        let font_size = 20.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
          text,
          (screen_width() / 2.0) - (text_dims.width / 2.0),
          screen_height() / 2.0,
          20.0,
          WHITE,
        );

        if is_key_pressed(KeyCode::Space) {
          state.reset();
        } else if is_key_pressed(KeyCode::Q) {
          break;
        }
      }
    }
    next_frame().await
  }
}
