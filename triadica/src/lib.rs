pub mod viewer;

use std::sync::RwLock;

use glam::Vec3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
// use web_sys::console::{log_1, log_2};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use viewer::is_zero;

lazy_static::lazy_static! {
  pub static ref WINDOW_RATIO: RwLock<f32> = RwLock::new(1.0);
}

/// load `globalThis.window`
pub fn global_window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  global_window()
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

/// bind float number to uniform
fn bind_uniform_location(context: &WebGl2RenderingContext, program: &WebGlProgram, variable: &str, value: f32) -> Result<(), JsValue> {
  let location = context.get_uniform_location(program, variable);
  context.uniform1f(location.as_ref(), value);
  Ok(())
}

/// bind vec3 to uniform
fn bind_uniform3f_location(
  context: &WebGl2RenderingContext,
  program: &WebGlProgram,
  variable: &str,
  value: Vec3,
) -> Result<(), JsValue> {
  let location = context.get_uniform_location(program, variable);
  context.uniform3f(location.as_ref(), value.x, value.y, value.z);
  Ok(())
}

fn bind_uniforms(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(), JsValue> {
  let (forward, upward, rightward) = viewer::get_directions();

  // directions
  bind_uniform3f_location(context, program, "forward", forward)?;
  bind_uniform3f_location(context, program, "upward", upward)?;
  bind_uniform3f_location(context, program, "rightward", rightward)?;

  // lookDistance, defaults to 600
  bind_uniform_location(context, program, "lookDistance", 600.0)?;

  // backcone scale
  bind_uniform_location(context, program, "coneBackScale", 0.5)?;

  // viewportRatio
  let window_ratio = *WINDOW_RATIO.read().expect("to get window ratio");
  bind_uniform_location(context, program, "viewportRatio", window_ratio)?;

  // cameraPosition
  let pos = viewer::get_camera_position();
  bind_uniform3f_location(context, program, "cameraPosition", pos)?;
  // log_2(&"pos".into(), &format!("{:?}", pos).into());

  Ok(())
}

pub enum DrawMode {
  Triangles,
  Lines,
  LineStrip,
  TriangleStrip,
}

impl From<DrawMode> for u32 {
  fn from(x: DrawMode) -> Self {
    match x {
      DrawMode::Triangles => WebGl2RenderingContext::TRIANGLES,
      DrawMode::Lines => WebGl2RenderingContext::LINES,
      DrawMode::LineStrip => WebGl2RenderingContext::LINE_STRIP,
      DrawMode::TriangleStrip => WebGl2RenderingContext::TRIANGLE_STRIP,
    }
  }
}

pub fn draw(context: &WebGl2RenderingContext, program: &WebGlProgram, draw_mode: DrawMode, vertices: &[f32], vert_size: i32) {
  // context.color_mask(false, false, false, false);
  bind_attributes(context, program, vertices).expect("bind attrs");
  bind_uniforms(context, program).expect("to bind uniforms");
  context.clear_color(0.0, 0.0, 0.0, 1.0);
  context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

  context.draw_arrays(draw_mode.into(), 0, vert_size);
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

pub fn context_setup(context: &WebGl2RenderingContext) {
  context.enable(WebGl2RenderingContext::DEPTH_TEST);
  context.depth_func(WebGl2RenderingContext::LESS);
  // context.blend_func(WebGl2RenderingContext::ONE, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
  // context.depth_mask(false);
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

/// handle events from touch control and move the camera
pub fn on_control_event(
  elapsed: f32,
  left_move_x: f32,
  left_move_y: f32,
  right_move_x: f32,
  right_move_y: f32,
  right_a: bool,
) -> Result<(), JsValue> {
  if !is_zero(left_move_y) {
    viewer::move_viewer_by(Vec3::new(0., 0., -left_move_y * 2. * elapsed));
  }
  if !(is_zero(left_move_x)) {
    viewer::rotate_glance_by(0.01 * elapsed * left_move_x, 0.0);
  }

  // log_1(&JsValue::from_str(format!("shift? {}", right_a).as_str()));

  if right_a {
    if !is_zero(right_move_y) {
      viewer::rotate_glance_by(0., right_move_y * 0.05 * elapsed);
    }

    if !is_zero(right_move_x) {
      viewer::spin_glance_by(right_move_x * 0.05 * elapsed);
    }
  } else if !is_zero(right_move_x) || !is_zero(right_move_y) {
    viewer::move_viewer_by(Vec3::new(right_move_x * 2. * elapsed, right_move_y * 2. * elapsed, 0.));
  }

  Ok(())
}

/// read window sizes and resize canvas
pub fn resize_canvas(canvas: Element) -> Result<(), JsValue> {
  let window = web_sys::window().ok_or("to get window")?;
  let inner_width = window.inner_width()?.as_f64().ok_or("to get window width")?;
  let inner_height = window.inner_height()?.as_f64().ok_or("to get window height")?;

  let mut r = WINDOW_RATIO.write().expect("write ratio");
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
