mod ffi;

use crate::structs::Color;

pub fn fade(color: Color, alpha: f32) -> Color {
  return unsafe { ffi::Fade(color, alpha) };
}
