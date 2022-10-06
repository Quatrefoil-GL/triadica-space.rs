use glam::f32::Vec3;
use std::sync::RwLock;

lazy_static::lazy_static! {
  static ref VIEWER_POSITION: RwLock<Vec3> = RwLock::new(Vec3::new(0.0, 0.0, 0.0));
  static ref VIEWER_ANGLE: RwLock<f32> = RwLock::new(std::f32::consts::PI * 0.5);
  static ref VIEWER_Y_SHIFY: RwLock<f32> = RwLock::new(0.0);
  static ref DIRTY_MARK: RwLock<bool> = RwLock::new(true);

  /// TODO Control upward direction with touch control
  static ref VIEWER_UPWARD: RwLock<Vec3> = RwLock::new(Vec3::new(0.0, 1.0, 0.0));
}

pub fn move_viewer_by(p: Vec3) {
  let p1 = to_viewer_axis(p);
  let mut v_position = VIEWER_POSITION.write().expect("to read viewer position");
  let p0 = *v_position;
  *v_position = p1 + p0;
  mark_dirty();
}

pub fn rotate_view_by(x: f32) {
  let mut angle = VIEWER_ANGLE.write().expect("to load viewer angle");
  *angle += x;
  mark_dirty();
}

pub fn shift_viewer_by(dy: f32) {
  let mut y_shift = VIEWER_Y_SHIFY.write().expect("to load viewer y shift");
  *y_shift += 2.0 * dy;
  mark_dirty();
}

/// compare the point to viewer's position and angle
pub fn to_viewer_axis(p: Vec3) -> Vec3 {
  let half_pi: f32 = std::f32::consts::PI * 0.5;

  // let length = (p.0 * p.0 + p.1 * p.1 + p.2 * p.2).sqrt();
  let angle: f32 = get_viewer_angle();
  let project_distance = 20.0;
  let y_shift = get_y_shift();
  // vertical angle
  let v_angle = (y_shift / project_distance).atan();

  let from_x = (p.x * (angle - half_pi).cos(), 0.0, -1. * p.x * (angle - half_pi).sin());

  let from_y = (
    p.y * (v_angle + half_pi).cos() * angle.cos(),
    p.y * (v_angle + half_pi).sin(),
    -1. * p.y * (v_angle + half_pi).cos() * angle.sin(),
  );

  let from_z = (
    p.z * -1. * v_angle.cos() * angle.cos(),
    p.z * -1. * v_angle.sin(),
    p.z * v_angle.cos() * angle.sin(),
  );

  Vec3::new(
    from_x.0 + from_y.0 + from_z.0,
    from_x.1 + from_y.1 + from_z.1,
    from_x.2 + from_y.2 + from_z.2,
  )
}

/// get a vector at viewing position at length 600
pub fn new_lookat_point() -> Vec3 {
  let angle = get_viewer_angle();
  let y_shift = get_y_shift();
  let p: Vec3 = Vec3::new(angle.cos() * 400., y_shift * 20., angle.sin() * -400.);

  let l = p.length();
  let ratio = 600. / l;
  Vec3::new(p.x * ratio, p.y * ratio, p.z * ratio)
}

pub fn get_position() -> Vec3 {
  *VIEWER_POSITION.read().expect("to load viewer position")
}

pub fn mark_dirty() {
  *DIRTY_MARK.write().expect("to load dirty mark") = true;
  // web_sys::console::log_1(&"dirty".into());
}

pub fn requested_rendering() -> bool {
  let mut mark = DIRTY_MARK.write().expect("to load dirty mark");
  let ret = *mark;
  *mark = false;
  ret
}

pub fn get_y_shift() -> f32 {
  *VIEWER_Y_SHIFY.read().expect("to load viewer y shift")
}

pub fn get_viewer_angle() -> f32 {
  *VIEWER_ANGLE.read().expect("to load viewer angle")
}

pub fn reset_shift_y() {
  *VIEWER_Y_SHIFY.write().expect("to load viewer y shift") = 0.0;
  mark_dirty()
}

#[allow(dead_code)]
pub fn render_debug_text() -> String {
  use std::fmt::Write;
  let mut ret = String::new();
  writeln!(ret, "{:?}", get_position()).expect("write");
  ret.push_str(&get_viewer_angle().to_string());
  write!(ret, "\n{:?}", new_lookat_point()).expect("write");
  ret
}

pub fn get_view_upward() -> Vec3 {
  *VIEWER_UPWARD.read().expect("to load viewer upward")
}
