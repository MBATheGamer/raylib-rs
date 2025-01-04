use raylib_rs::{
  consts::colors::{LIGHTGRAY, RAYWHITE},
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - basic window",
  );

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();
    clear_background(RAYWHITE);
    draw_text(
      "Congrats! You created your first window!",
      190,
      200,
      20,
      LIGHTGRAY,
    );
    end_drawing();
  }
  close_window();
}
