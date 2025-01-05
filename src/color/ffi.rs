use crate::structs::Color;

unsafe extern "C" {
  pub unsafe fn Fade(color: Color, alpha: f32) -> Color;
}
