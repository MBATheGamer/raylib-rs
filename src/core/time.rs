use super::ffi::SetTargetFPS;

pub fn set_target_fps(fps: i32) {
  unsafe {
    SetTargetFPS(fps);
  }
}
