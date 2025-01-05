#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}
