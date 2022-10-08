use triadica::path;
use triadica::viewer;
use web_sys::console::log_1;

use std::cell::RefCell;
use std::include_str;
use std::rc::Rc;

use glam::Vec3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::console::{log_1, log_2};
use web_sys::WebGl2RenderingContext;

use viewer::is_zero;

#[wasm_bindgen(js_name = initApp)]
pub fn init_app() -> Result<(), JsValue> {
  // console_error_panic_hook::set_once();

  let window = web_sys::window().ok_or("to get window")?;

  let document = window.document().ok_or("to get document")?;
  let canvas = document.query_selector(".app")?.ok_or("to get canvas")?;

  on_window_resize()?;

  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

  let context = canvas
    .get_context("webgl2")?
    .ok_or("to load context")?
    .dyn_into::<WebGl2RenderingContext>()?;

  let vert_shader = triadica::compile_shader(
    &context,
    WebGl2RenderingContext::VERTEX_SHADER,
    include_str!("../shaders/demo.vert"),
  )?;

  let frag_shader = triadica::compile_shader(
    &context,
    WebGl2RenderingContext::FRAGMENT_SHADER,
    include_str!("../shaders/demo.frag"),
  )?;

  let program = triadica::link_program(&context, &vert_shader, &frag_shader)?;
  context.use_program(Some(&program));

  context.enable(WebGl2RenderingContext::DEPTH_TEST);
  context.depth_func(WebGl2RenderingContext::LESS);
  // context.blend_func(WebGl2RenderingContext::ONE, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
  // context.depth_mask(false);

  // let mut vertices = vec![];
  // for i in path::compute_cube_vertices() {
  //   vertices.push(i);
  // }
  // for i in path::compute_lamp_tree_vertices() {
  //   vertices.push(i);
  // }
  // let vertices = path::compute_cube_vertices();
  let vertices = path::compute_lamp_tree_vertices();

  triadica::bind_attributes(&context, &program, &vertices)?;

  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  let copied_context = Rc::new(context.to_owned());
  let copied_program = Rc::new(program);
  let vertices_count = (vertices.len() / 3) as i32;
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if viewer::requested_rendering() {
      triadica::bind_uniforms(&*copied_context, &*copied_program).expect("to bind uniforms");
      triadica::draw(&context, vertices_count);
      // document
      //   .query_selector(".debug")
      //   .expect("to get debug area")
      //   .expect("some debug area")
      //   .set_text_content(Some(&viewer::render_debug_text()));
    }

    // Schedule ourself for another requestAnimationFrame callback.
    triadica::request_animation_frame(f.borrow().as_ref().expect("building closure"));
  }) as Box<dyn FnMut()>));

  triadica::request_animation_frame(g.borrow().as_ref().ok_or("expect a closure")?);

  Ok(())
}

#[wasm_bindgen(js_name = onWindowResize)]
pub fn on_window_resize() -> Result<(), JsValue> {
  let window = web_sys::window().ok_or("to get window")?;
  let canvas = window
    .document()
    .ok_or("to get document")?
    .query_selector(".app")?
    .ok_or("to get canvas")?;

  let inner_width = window.inner_width()?.as_f64().ok_or("to get window width")?;
  let inner_height = window.inner_height()?.as_f64().ok_or("to get window height")?;

  let mut r = triadica::WINDOW_RATIO.write().expect("write ratio");
  *r = (inner_height / inner_width) as f32;

  // log_1(&format!("{} {}", inner_height, inner_width).into());

  canvas.set_attribute("width", &format!("{}px", inner_width * 2.))?;
  canvas.set_attribute("height", &format!("{}px", inner_height * 2.))?;
  canvas.set_attribute("style", &format!("width: {}px; height: {}px;", inner_width, inner_height))?;

  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  let context = canvas
    .get_context("webgl2")?
    .ok_or("to get context")?
    .dyn_into::<WebGl2RenderingContext>()?;
  context.viewport(0, 0, inner_width as i32 * 2, inner_height as i32 * 2);

  viewer::mark_dirty();

  Ok(())
}

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen(js_name = "onControl")]
pub fn on_control(
  elapsed: f32,
  left_move_x: f32,
  left_move_y: f32,
  right_move_x: f32,
  right_move_y: f32,
  _right_delta_x: f32,
  _right_delta_y: f32,
  right_a: bool,
) -> Result<(), JsValue> {
  if !is_zero(left_move_y) {
    viewer::move_viewer_by(Vec3::new(0., 0., -left_move_y * 2. * elapsed));
  }
  if !(is_zero(left_move_x)) {
    viewer::rotate_glance_by(-0.01 * elapsed * left_move_x, 0.0);
  }

  // log_1(&JsValue::from_str(format!("shift? {}", right_a).as_str()));

  if right_a {
    if !is_zero(right_move_y) {
      viewer::rotate_glance_by(0., right_move_y * 0.05 * elapsed);
    }

    if !is_zero(right_move_x) {
      viewer::spin_glance_by(right_move_x * -0.05 * elapsed);
    }
  } else if !is_zero(right_move_x) || !is_zero(right_move_y) {
    viewer::move_viewer_by(Vec3::new(right_move_x * 2. * elapsed, right_move_y * 2. * elapsed, 0.));
  }

  Ok(())
}
