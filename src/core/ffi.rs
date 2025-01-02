unsafe extern "C" {
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);
  pub unsafe fn CloseWindow();
}
