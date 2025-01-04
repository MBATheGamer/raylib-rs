use raylib_rs::{
  consts::colors::{BLUE, DARKBLUE, DARKGREEN, GRAY, GREEN, LIGHTGRAY, MAROON, PURPLE, RAYWHITE},
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, is_gesture_detected,
    is_key_pressed, set_target_fps, window_should_close,
  },
  enums::{Gesture, KeyboardKey},
  shapes::draw_rectangle,
  text::draw_text,
};

enum GameScreen {
  Logo = 0,
  Title,
  Gameplay,
  Ending,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - basic screen manager",
  );

  let mut current_screen = GameScreen::Logo;

  let mut frames_counter = 0;

  set_target_fps(60);

  while !window_should_close() {
    match current_screen {
      GameScreen::Logo => {
        frames_counter += 1;

        if frames_counter > 120 {
          current_screen = GameScreen::Title;
        }
      }
      GameScreen::Title => {
        if is_key_pressed(KeyboardKey::KeyEnter) || is_gesture_detected(Gesture::GestureTap) {
          current_screen = GameScreen::Gameplay;
        }
      }
      GameScreen::Gameplay => {
        if is_key_pressed(KeyboardKey::KeyEnter) || is_gesture_detected(Gesture::GestureTap) {
          current_screen = GameScreen::Ending;
        }
      }
      GameScreen::Ending => {
        if is_key_pressed(KeyboardKey::KeyEnter) || is_gesture_detected(Gesture::GestureTap) {
          current_screen = GameScreen::Title;
        }
      }
    }
    begin_drawing();
    clear_background(RAYWHITE);

    match current_screen {
      GameScreen::Logo => {
        draw_text("LOGO SCREEN", 20, 20, 40, LIGHTGRAY);
        draw_text("WAIT for 2 SECONDS", 290, 220, 20, GRAY);
      }
      GameScreen::Title => {
        draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, GREEN);
        draw_text("TITLE SCREEN", 20, 20, 40, DARKGREEN);
        draw_text(
          "PRESS ENTER or TAP to JUMP to GAMEPLAY SCREEN",
          120,
          220,
          20,
          DARKGREEN,
        );
      }
      GameScreen::Gameplay => {
        draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, PURPLE);
        draw_text("GAMEPLAY SCREEN", 20, 20, 40, MAROON);
        draw_text(
          "PRESS ENTER or TAP to JUMP to ENDING SCREEN",
          130,
          220,
          20,
          MAROON,
        );
      }
      GameScreen::Ending => {
        draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, BLUE);
        draw_text("ENDING SCREEN", 20, 20, 40, DARKBLUE);
        draw_text(
          "PRESS ENTER or TAP to RETURN to TITLE SCREEN",
          120,
          220,
          20,
          DARKBLUE,
        );
      }
    }

    end_drawing();
  }

  close_window();
}
