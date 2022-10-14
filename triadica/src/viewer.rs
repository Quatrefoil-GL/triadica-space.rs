use glam::f32::Vec3;
use std::sync::RwLock;

lazy_static::lazy_static! {
  static ref VIEWER_POSITION: RwLock<Vec3> = RwLock::new(Vec3::new(0.0, 0.0, 0.0));
  static ref DIRTY_MARK: RwLock<bool> = RwLock::new(true);


  static ref VIEWER_UPWARD: RwLock<Vec3> = RwLock::new(Vec3::new(0.0, 1.0, 0.0));
  static ref VIEWER_FORWARD: RwLock<Vec3> = RwLock::new(Vec3::new(0.0, 0.0, -1.0));
}

pub fn move_viewer_by(p: Vec3) {
  let p1 = to_viewer_axis(p);
  let mut v_position = VIEWER_POSITION.write().expect("to read viewer position");
  let p0 = *v_position;
  *v_position = p1 + p0;
  mark_dirty();
}

pub fn rotate_glance_by(x: f32, y: f32) {
  if !is_zero(x) {
    let da = x * 0.1;
    let (forward, _, rightward) = get_directions();
    *VIEWER_FORWARD.write().expect("to write") = forward * da.cos() + rightward * da.sin();
    mark_dirty();
  }

  if !is_zero(y) {
    let da = y * 0.1;
    let (forward, upward, _) = get_directions();
    *VIEWER_FORWARD.write().expect("to write") = forward * da.cos() + upward * da.sin();
    *VIEWER_UPWARD.write().expect("to write") = upward * da.cos() - forward * da.sin();
    mark_dirty();
  }
}

pub fn spin_glance_by(v: f32) {
  if !is_zero(v) {
    let da = v * 0.1;
    let (_, upward, rightward) = get_directions();
    *VIEWER_UPWARD.write().expect("to write viewer upward") = upward * da.cos() + rightward * da.sin();
    mark_dirty();
  }
}

/// get forward, uoward, rightward directions
pub fn get_directions() -> (Vec3, Vec3, Vec3) {
  let forward = *VIEWER_FORWARD.read().expect("to read viewer forward");
  let upward = *VIEWER_UPWARD.read().expect("to load viewer upward");
  let rightward = upward.cross(forward);
  (forward, upward, rightward)
}

/// compare the point to viewer's position and angle
pub fn to_viewer_axis(p: Vec3) -> Vec3 {
  let (forward, upward, rightward) = get_directions();
  rightward * p.x + upward * p.y + forward * -p.z
}

/// load camera position
pub fn get_camera_position() -> Vec3 {
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

#[allow(dead_code)]
pub fn render_debug_text() -> String {
  use std::fmt::Write;
  let mut ret = String::new();
  writeln!(ret, "{:?}", get_camera_position()).expect("write");
  ret
}

pub fn get_view_upward() -> Vec3 {
  *VIEWER_UPWARD.read().expect("to load viewer upward")
}

pub fn is_zero(x: f32) -> bool {
  x.abs() < std::f32::EPSILON
}
