use super::vector2::Vector2;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Camera2D {
  pub offset: Vector2,
  pub target: Vector2,
  pub rotation: f32,
  pub zoom: f32,
}
