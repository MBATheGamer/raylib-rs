use crate::structs::Color;

use super::ffi::{BeginDrawing, ClearBackground, EndDrawing};

#[inline]
pub fn clear_background(color: Color) {
  unsafe {
    ClearBackground(color);
  }
}

#[inline]
pub fn begin_drawing() {
  unsafe {
    BeginDrawing();
  }
}

#[inline]
pub fn end_drawing() {
  unsafe {
    EndDrawing();
  }
}
