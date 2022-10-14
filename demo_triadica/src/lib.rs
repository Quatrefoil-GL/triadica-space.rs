mod shape;
use triadica::global_window;
use triadica::viewer;
use triadica::ShaderProgramCaches;
use web_sys::Element;
// use web_sys::console::log_1;

use std::cell::RefCell;
use std::include_str;
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsCast};
// use web_sys::console::{log_1, log_2};
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen(js_name = initApp)]
pub fn init_app() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  on_window_resize()?;

  let canvas = get_canvas();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  let context = canvas
    .get_context("webgl2")?
    .ok_or("to load context")?
    .dyn_into::<WebGl2RenderingContext>()?;

  triadica::context_setup(&context);

  let vert_shader = include_str!("../shaders/demo.vert");
  let frag_shader = include_str!("../shaders/demo.frag");

  let program_caches = Rc::new(RefCell::new(ShaderProgramCaches::default()));

  let program = Rc::new(triadica::cached_link_program(&context, vert_shader, frag_shader, program_caches)?);

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
      context.use_program(Some(&program));
      triadica::draw(
        &context,
        &program,
        triadica::DrawMode::Lines,
        &vertices,
        (vertices.len() / 3) as i32,
      );
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

fn get_canvas() -> Element {
  let window = global_window();
  let document = window.document().expect("to get document");
  let canvas = document.query_selector(".app").expect("to get canvas").expect("some canvas");
  canvas.dyn_into::<Element>().expect("to cast to canvas")
}

#[wasm_bindgen(js_name = onWindowResize)]
pub fn on_window_resize() -> Result<(), JsValue> {
  let canvas = get_canvas();
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
