use macroquad::prelude::*;
use macroquad::rand;

//const SCREEN_WIDTH: f32 = screen_width();
//const SCREEN_HEIGHT: i32 = 50;
//const FRAME_DURATION: f32 = 60.0;
const GRAVITY: f32 = 8.0;
const FLAP_POWER: f32 = 6.0;
const FLYING_SPEED: f32 = 150.0;
const PLAYER_SIZE: f32 = 26.0;

enum GameMode {
  Menu,
  Playing,
  Paused,
  End,
}

struct Obstacle {
  x: f32,
  rect_top: Rect,
  rect_bottom: Rect,
}

struct Player {
  x: f32,
  y: f32,
  rect: Rect,
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
      player: Player::new(80.0, 100.0),
      frame_time: 0.0,
      score: 0,
      obstacle: Obstacle::new(screen_width(), 0),
    }
  }

  fn reset(&mut self) {
    self.mode = GameMode::Playing;
    self.player.reset();
    self.frame_time = 0.0;
    self.score = 0;
    self.obstacle = Obstacle::new(screen_width(), 0);
  }

  fn tick(&mut self) {
    if self.player.x > self.obstacle.x {
      self.score += 1;
      self.obstacle = Obstacle::new(self.player.x + screen_width(), self.score);
    }

    if self.player.y > screen_height() || self.obstacle.collision_detection(&self.player) {
      self.mode = GameMode::End;
    }
  }

  fn render(&self) {
    let text = &format!("Score: {}", self.score);
    let font_size = 20.0;
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
      text,
      screen_width() - (text_dims.width / 2.0) - 50.0,
      30.0,
      20.0,
      WHITE,
    );
  }
}

impl Obstacle {
  fn new(x: f32, score: i32) -> Self {
    let gap_y = rand::gen_range(30, screen_height() as i32 - 60);
    let size = i32::max(50, 200 - score) / 2;

    Obstacle {
      x,
      rect_top: Rect::new(x, 0.0, 16.0, (gap_y - size) as f32),
      rect_bottom: Rect::new(x, (gap_y + size) as f32, 16.0, screen_height() as f32),
    }
  }

  fn render(&mut self, curr_x: f32) {
    let draw_x = self.x - curr_x;
    self.rect_top.x = draw_x;
    self.rect_bottom.x = draw_x;

    draw_rectangle(
      self.rect_top.x,
      self.rect_top.y,
      self.rect_top.w,
      self.rect_top.h,
      LIGHTGRAY,
    );
    draw_rectangle(
      self.rect_bottom.x,
      self.rect_bottom.y,
      self.rect_bottom.w,
      self.rect_bottom.h,
      LIGHTGRAY,
    );
  }

  fn collision_detection(&self, player: &Player) -> bool {
    player.rect.intersect(self.rect_top).is_some()
      || player.rect.intersect(self.rect_bottom).is_some()
  }
}

impl Player {
  fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
      rect: Rect::new(x, y, PLAYER_SIZE, PLAYER_SIZE),
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

    if self.velocity < -4.0 {
      self.velocity = -4.0;
    }

    self.y += self.velocity;
    self.x += FLYING_SPEED * delta_time;

    /* test max height */
    if self.y < 11.0 {
      self.y = 11.0;
      self.velocity /= 2.0;
    }

    self.rect.y = self.y;
  }

  fn render(&self) {
    draw_circle(
      80.0 + PLAYER_SIZE / 2.0,
      self.y + PLAYER_SIZE / 2.0,
      PLAYER_SIZE / 2.0,
      YELLOW,
    );
  }
}

#[macroquad::main("Flappy Macro")]
async fn main() {
  rand::srand(macroquad::miniquad::date::now() as _);
  let mut state = State::new();
  let mut delta_time;

  loop {
    delta_time = get_frame_time();

    match state.mode {
      GameMode::Menu => {
        clear_background(DARKPURPLE);

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

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
          state.mode = GameMode::Playing;
        } else if is_key_pressed(KeyCode::Q) {
          break;
        }
      }
      GameMode::Playing => {
        clear_background(DARKGRAY);

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
          state.player.flap();
        } else if is_key_pressed(KeyCode::P) {
          state.mode = GameMode::Paused;
        }

        state.player.tick(delta_time);
        state.tick();

        state.player.render();
        state.obstacle.render(state.player.x);
        state.render();
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
        let text_dims: TextDimensions = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
          text,
          (screen_width() / 2.0) - (text_dims.width / 2.0),
          screen_height() / 2.0,
          20.0,
          WHITE,
        );

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
          state.reset();
        } else if is_key_pressed(KeyCode::Q) {
          break;
        }
      }
    }
    next_frame().await
  }
}
