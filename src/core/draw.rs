use super::ffi::{BeginDrawing, EndDrawing};

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
