use crate::structs::Color;

unsafe extern "C" {
  // Window-related functions
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);
  pub unsafe fn CloseWindow();
  pub unsafe fn WindowShouldClose() -> bool;

  // Drawing-related functions
  pub unsafe fn ClearBackground(color: Color);
  pub unsafe fn BeginDrawing();
  pub unsafe fn EndDrawing();

  // Timing-related functions
  pub unsafe fn SetTargetFPS(fps: i32);

  // Input-related functions: keyboard
  pub unsafe fn IsKeyPressed(key: i32) -> bool;

  // Gestures and Touch Handling Functions
  pub unsafe fn IsGestureDetected(gesture: u32) -> bool;
}
