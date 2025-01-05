use crate::{
  shapes::ffi::{DrawLine, DrawRectangle, DrawRectangleRec},
  structs::{Color, Rectangle},
};

#[inline]
pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
  unsafe {
    DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color);
  }
}

#[inline]
pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
  unsafe {
    DrawRectangle(pos_x, pos_y, width, height, color);
  }
}

#[inline]
pub fn draw_rectangle_rec(rectangle: Rectangle, color: Color) {
  unsafe {
    DrawRectangleRec(rectangle, color);
  }
}
