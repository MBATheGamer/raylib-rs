use super::ffi::BeginDrawing;

#[inline]
pub fn begin_drawing() {
  unsafe {
    BeginDrawing();
  }
}
