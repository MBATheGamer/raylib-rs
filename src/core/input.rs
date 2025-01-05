use crate::enums::KeyboardKey;

use super::ffi::{IsKeyDown, IsKeyPressed};

pub fn is_key_pressed(key: KeyboardKey) -> bool {
  return unsafe { IsKeyPressed(key as i32) };
}

pub fn is_key_down(key: KeyboardKey) -> bool {
  return unsafe { IsKeyDown(key as i32) };
}
