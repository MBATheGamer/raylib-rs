use raylib_rs::{
  color::fade,
  consts::colors::{BLACK, BLUE, DARKGRAY, GREEN, RAYWHITE, RED, SKYBLUE},
  core::{
    begin_drawing, begin_mode_2d, clear_background, close_window, end_drawing, end_mode_2d,
    get_mouse_wheel_move, get_random_value, init_window, is_key_down, is_key_pressed,
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shapes::{draw_line, draw_rectangle, draw_rectangle_lines, draw_rectangle_rec},
  structs::{Camera2D, Color, Rectangle, Vector2},
  text::draw_text,
};

const MAX_BUILDINGS: usize = 100;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 2d camera",
  );

  let mut player = Rectangle {
    x: 400.0,
    y: 280.0,
    width: 40.0,
    height: 40.0,
  };
  let mut buildings = [Rectangle::default(); MAX_BUILDINGS];
  let mut build_colors = [Color::default(); MAX_BUILDINGS];

  let mut spacing = 0;

  for i in 0..MAX_BUILDINGS {
    buildings[i].width = get_random_value(50, 200) as f32;
    buildings[i].height = get_random_value(100, 800) as f32;
    buildings[i].y = SCREEN_HEIGHT as f32 - 130.0 - buildings[i].height;
    buildings[i].x = -6000.0 + spacing as f32;

    spacing += buildings[i].width as i32;

    build_colors[i] = Color {
      red: get_random_value(200, 240) as u8,
      green: get_random_value(200, 240) as u8,
      blue: get_random_value(200, 250) as u8,
      alpha: 255,
    }
  }

  let mut camera = Camera2D::default();
  camera.target = Vector2 {
    x: player.x + 20.0,
    y: player.y + 20.0,
  };
  camera.offset = Vector2 {
    x: SCREEN_WIDTH as f32 / 2.0,
    y: SCREEN_HEIGHT as f32 / 2.0,
  };
  camera.rotation = 0.0;
  camera.zoom = 1.0;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_down(KeyboardKey::KeyRight) {
      player.x += 2.0;
    } else if is_key_down(KeyboardKey::KeyLeft) {
      player.x -= 2.0;
    }

    camera.target = Vector2 {
      x: player.x + 20.0,
      y: player.y + 20.0,
    };

    if is_key_down(KeyboardKey::KeyA) {
      camera.rotation -= 1.0;
    } else if is_key_down(KeyboardKey::KeyS) {
      camera.rotation += 1.0;
    }

    if camera.rotation > 40.0 {
      camera.rotation = 40.0;
    } else if camera.rotation < -40.0 {
      camera.rotation = -40.0;
    }

    camera.zoom += get_mouse_wheel_move() * 0.05;

    if camera.zoom > 3.0 {
      camera.zoom = 3.0;
    } else if camera.zoom < 0.1 {
      camera.zoom = 0.1;
    }

    if is_key_pressed(KeyboardKey::KeyR) {
      camera.zoom = 1.0;
      camera.rotation = 0.0;
    }

    begin_drawing();
    clear_background(RAYWHITE);

    begin_mode_2d(camera);
    draw_rectangle(-6000, 320, 13000, 8000, DARKGRAY);

    for i in 0..MAX_BUILDINGS {
      draw_rectangle_rec(buildings[i], build_colors[i]);
    }

    draw_rectangle_rec(player, RED);

    draw_line(
      camera.target.x as i32,
      -SCREEN_HEIGHT * 10,
      camera.target.x as i32,
      SCREEN_HEIGHT * 10,
      GREEN,
    );
    draw_line(
      -SCREEN_WIDTH * 10,
      camera.target.y as i32,
      SCREEN_WIDTH * 10,
      camera.target.y as i32,
      GREEN,
    );
    end_mode_2d();

    draw_text("SCREEN AREA", 640, 10, 20, RED);

    draw_rectangle(0, 0, SCREEN_WIDTH, 5, RED);
    draw_rectangle(0, 5, 5, SCREEN_HEIGHT - 10, RED);
    draw_rectangle(SCREEN_WIDTH - 5, 5, 5, SCREEN_HEIGHT - 10, RED);
    draw_rectangle(0, SCREEN_HEIGHT - 5, SCREEN_WIDTH, 5, RED);

    draw_rectangle(10, 10, 250, 113, fade(SKYBLUE, 0.5));
    draw_rectangle_lines(10, 10, 250, 113, BLUE);

    draw_text("Free 2d camera controls:", 20, 20, 10, BLACK);
    draw_text("- Right/Left to move Offset", 40, 40, 10, DARKGRAY);
    draw_text("- Mouse Wheel to Zoom in-out", 40, 60, 10, DARKGRAY);
    draw_text("- A / S to Rotate", 40, 80, 10, DARKGRAY);
    draw_text("- R to reset Zoom and Rotation", 40, 100, 10, DARKGRAY);

    end_drawing();
  }
  close_window();
}
