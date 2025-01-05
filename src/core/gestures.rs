use crate::enums::Gesture;

use super::ffi::IsGestureDetected;

#[inline]
pub fn is_gesture_detected(gesture: Gesture) -> bool {
  return unsafe { IsGestureDetected(gesture as u32) };
}
