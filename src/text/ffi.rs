use crate::structs::Color;

unsafe extern "C" {
  pub unsafe fn DrawText(text: *const i8, pos_x: i32, pos_y: i32, font_size: i32, color: Color);
}
