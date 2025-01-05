use crate::structs::{Color, Rectangle};

unsafe extern "C" {
  // Basic shapes drawing functions
  pub unsafe fn DrawLine(
    start_pos_x: i32,
    start_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: Color,
  );
  pub unsafe fn DrawRectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
  pub unsafe fn DrawRectangleRec(rectangle: Rectangle, color: Color);
  pub unsafe fn DrawRectangleLines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
}
