use crate::structs::Color;

unsafe extern "C" {
  pub unsafe fn DrawRectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
}
