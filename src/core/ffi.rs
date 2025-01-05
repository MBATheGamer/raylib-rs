use crate::structs::{Camera2D, Color};

unsafe extern "C" {
  // Window-related functions
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);
  pub unsafe fn CloseWindow();
  pub unsafe fn WindowShouldClose() -> bool;

  // Drawing-related functions
  pub unsafe fn ClearBackground(color: Color);
  pub unsafe fn BeginDrawing();
  pub unsafe fn EndDrawing();
  pub unsafe fn BeginMode2D(camera: Camera2D);
  pub unsafe fn EndMode2D();

  // Timing-related functions
  pub unsafe fn SetTargetFPS(fps: i32);

  // Random values generation functions
  pub unsafe fn GetRandomValue(min: i32, max: i32) -> i32;

  // Input-related functions: keyboard
  pub unsafe fn IsKeyPressed(key: i32) -> bool;
  pub unsafe fn IsKeyDown(key: i32) -> bool;

  // Input-related functions: mouse
  pub unsafe fn GetMouseWheelMove() -> f32;

  // Gestures and Touch handling functions
  pub unsafe fn IsGestureDetected(gesture: u32) -> bool;
}
