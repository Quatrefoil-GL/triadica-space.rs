mod shape;
use triadica::global_window;
use triadica::viewer;
// use web_sys::console::log_1;

use std::cell::RefCell;
use std::include_str;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsCast};
// use web_sys::console::{log_1, log_2};
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen(js_name = initApp)]
pub fn init_app() -> Result<(), JsValue> {
  // console_error_panic_hook::set_once();

  let window = global_window();

  let document = window.document().ok_or("to get document")?;
  let canvas = document.query_selector(".app")?.ok_or("to get canvas")?;

  on_window_resize()?;

  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  let context = canvas
    .get_context("webgl2")?
    .ok_or("to load context")?
    .dyn_into::<WebGl2RenderingContext>()?;
  let copied_context = Rc::new(context.to_owned());

  triadica::context_setup(&context);

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
  let copied_program = Rc::new(program);

  // let mut vertices = vec![];
  // for i in shape::compute_cube_vertices() {
  //   vertices.push(i);
  // }
  // for i in shape::compute_lamp_tree_vertices() {
  //   vertices.push(i);
  // }
  // let vertices = shape::compute_cube_vertices();
  let vertices = shape::compute_lamp_tree_vertices();

  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if viewer::requested_rendering() {
      context.use_program(Some(&*copied_program));
      triadica::bind_attributes(&context, &*copied_program, &vertices).expect("bind attrs");
      let vertices_count = (vertices.len() / 3) as i32;
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
  let window = global_window();
  let canvas = window
    .document()
    .ok_or("to get document")?
    .query_selector(".app")?
    .ok_or("to get canvas")?;

  triadica::resize_canvas(canvas)
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
  triadica::on_control_event(elapsed, left_move_x, left_move_y, right_move_x, right_move_y, right_a)
}
