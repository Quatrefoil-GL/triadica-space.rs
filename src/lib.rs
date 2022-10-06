mod path;
mod viewer;

use std::cell::RefCell;
use std::include_str;
use std::rc::Rc;
use std::sync::RwLock;

use glam::Vec3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::console::{log_1, log_2};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

lazy_static::lazy_static! {
  static ref WINDOW_RATIO: RwLock<f32> = RwLock::new(1.0);
}

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

  let vert_shader = compile_shader(
    &context,
    WebGl2RenderingContext::VERTEX_SHADER,
    include_str!("../shaders/demo.vert"),
  )?;

  let frag_shader = compile_shader(
    &context,
    WebGl2RenderingContext::FRAGMENT_SHADER,
    include_str!("../shaders/demo.frag"),
  )?;

  let program = link_program(&context, &vert_shader, &frag_shader)?;
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

  bind_attributes(&context, &program, &vertices)?;

  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  let copied_context = Rc::new(context.to_owned());
  let copied_program = Rc::new(program);
  let vertices_count = (vertices.len() / 3) as i32;
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if viewer::requested_rendering() {
      bind_uniforms(&*copied_context, &*copied_program).expect("to bind uniforms");
      draw(&context, vertices_count);
      // document
      //   .query_selector(".debug")
      //   .expect("to get debug area")
      //   .expect("some debug area")
      //   .set_text_content(Some(&viewer::render_debug_text()));
    }

    // Schedule ourself for another requestAnimationFrame callback.
    request_animation_frame(f.borrow().as_ref().expect("building closure"));
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().ok_or("expect a closure")?);

  Ok(())
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

fn bind_attributes(context: &WebGl2RenderingContext, program: &WebGlProgram, vertices: &[f32]) -> Result<(), JsValue> {
  // web_sys::console::log_1(&format!("{:?}", vertices).into());

  let position_attribute_location = context.get_attrib_location(program, "a_position");
  let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
  context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

  // Note that `Float32Array::view` is somewhat dangerous (hence the
  // `unsafe`!). This is creating a raw view into our module's
  // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
  // (aka do a memory allocation in Rust) it'll cause the buffer to change,
  // causing the `Float32Array` to be invalid.
  //
  // As a result, after `Float32Array::view` we have to be very careful not to
  // do any memory allocations before it's dropped.
  unsafe {
    let positions_array_buf_view = js_sys::Float32Array::view(vertices);

    context.buffer_data_with_array_buffer_view(
      WebGl2RenderingContext::ARRAY_BUFFER,
      &positions_array_buf_view,
      WebGl2RenderingContext::STATIC_DRAW,
    );
  }

  let vao = context.create_vertex_array().ok_or("Could not create vertex array object")?;
  context.bind_vertex_array(Some(&vao));

  context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
  context.enable_vertex_attrib_array(position_attribute_location as u32);

  Ok(())
}

fn bind_uniforms(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(), JsValue> {
  let lookat = viewer::new_lookat_point();
  // look distance
  let look_distance = context.get_uniform_location(program, "lookDistance");
  context.uniform1f(look_distance.as_ref(), lookat.length());

  // forward
  let forward = context.get_uniform_location(program, "forward");
  let lookat_u = lookat.normalize();
  context.uniform3f(forward.as_ref(), lookat_u.x, lookat_u.y, lookat_u.z);

  // upward
  let upward = context.get_uniform_location(program, "upward");
  let upward_vector = viewer::get_view_upward();
  context.uniform3f(upward.as_ref(), upward_vector.x, upward_vector.y, upward_vector.z);

  // rightward
  let rightward = context.get_uniform_location(program, "rightward");
  let rightward_vector = lookat_u.cross(upward_vector);
  context.uniform3f(rightward.as_ref(), rightward_vector.x, rightward_vector.y, rightward_vector.z);

  // backcone scale
  let backcone_location = context.get_uniform_location(program, "coneBackScale");
  context.uniform1f(backcone_location.as_ref(), 2.);

  // viewportRatio
  let viewport_ratio_location = context.get_uniform_location(program, "viewportRatio");
  let window_ratio = *WINDOW_RATIO.read().expect("to get window ratio");
  context.uniform1f(viewport_ratio_location.as_ref(), window_ratio as f32);

  // cameraPosition
  let camera_position_location = context.get_uniform_location(program, "cameraPosition");
  let pos = viewer::get_position();
  // log_2(&"pos".into(), &format!("{:?}", pos).into());
  context.uniform3f(camera_position_location.as_ref(), pos.x, pos.y, pos.z);

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

  let mut r = WINDOW_RATIO.write().expect("write ratio");
  *r = (inner_height / inner_width) as f32;

  // web_sys::console::log_1(&format!("{} {}", inner_height, inner_width).into());

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

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
  // context.color_mask(false, false, false, false);
  context.clear_color(0.0, 0.0, 0.0, 1.0);
  context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

  context.draw_arrays(WebGl2RenderingContext::LINE_STRIP, 0, vert_count);
}

pub fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
    .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  context: &WebGl2RenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = context
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
    .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}

#[allow(clippy::too_many_arguments)]
#[wasm_bindgen(js_name = "onControl")]
pub fn on_control(
  elapsed: f32,
  left_move_x: f32,
  left_move_y: f32,
  right_move_x: f32,
  right_move_y: f32,
  right_delta_x: f32,
  right_delta_y: f32,
  right_a: bool,
) -> Result<(), JsValue> {
  if !is_zero(left_move_y) {
    viewer::move_viewer_by(Vec3::new(0., 0., -left_move_y * 2. * elapsed));
  }
  if !(is_zero(left_move_x)) {
    viewer::rotate_glance_by(-0.01 * elapsed * left_move_x, 0.0);
  }
  if !right_a && !is_zero(right_move_x) || !is_zero(right_move_y) {
    viewer::move_viewer_by(Vec3::new(right_move_x * 2. * elapsed, right_move_y * 2. * elapsed, 0.));
  }

  if right_a && !is_zero(right_delta_y) {
    viewer::rotate_glance_by(0., right_delta_y * 0.05 * elapsed);
  }

  if right_a && !is_zero(right_delta_x) {
    viewer::spin_glance_by(right_delta_x * -0.05 * elapsed);
  }

  Ok(())
}
