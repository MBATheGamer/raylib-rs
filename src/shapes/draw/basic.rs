use crate::{shapes::ffi::DrawRectangle, structs::Color};

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
  unsafe {
    DrawRectangle(pos_x, pos_y, width, height, color);
  }
}
