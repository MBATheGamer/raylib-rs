use std::ffi::CString;

use super::ffi::InitWindow;

#[inline]
pub fn init_window(screen_width: i32, screen_height: i32, title: &str) {
  let title = CString::new(title).unwrap();

  unsafe {
    InitWindow(screen_width, screen_height, title.as_ptr());
  }
}
