use crate::enums::KeyboardKey;

use super::ffi::IsKeyPressed;

pub fn is_key_pressed(key: KeyboardKey) -> bool {
  return unsafe { IsKeyPressed(key as i32) };
}
