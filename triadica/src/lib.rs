pub mod path;
pub mod viewer;

use std::sync::RwLock;

use glam::Vec3;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::console::{log_1, log_2};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

lazy_static::lazy_static! {
  pub static ref WINDOW_RATIO: RwLock<f32> = RwLock::new(1.0);
}

pub fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

pub fn bind_attributes(context: &WebGl2RenderingContext, program: &WebGlProgram, vertices: &[f32]) -> Result<(), JsValue> {
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

pub fn bind_uniforms(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<(), JsValue> {
  let lookat = viewer::new_lookat_point();
  bind_uniform_location(context, program, "lookDistance", lookat.length())?;

  // forward
  let lookat_u = lookat.normalize();
  bind_uniform3f_location(context, program, "forward", lookat_u)?;

  // upward
  let upward_vector = viewer::get_view_upward();
  bind_uniform3f_location(context, program, "upward", upward_vector)?;

  // rightward
  let rightward_vector = lookat_u.cross(upward_vector);
  bind_uniform3f_location(context, program, "rightward", rightward_vector)?;

  // backcone scale
  bind_uniform_location(context, program, "coneBackScale", 0.5)?;

  // viewportRatio
  let window_ratio = *WINDOW_RATIO.read().expect("to get window ratio");
  bind_uniform_location(context, program, "viewportRatio", window_ratio)?;

  // cameraPosition
  let pos = viewer::get_position();
  bind_uniform3f_location(context, program, "cameraPosition", pos)?;
  // log_2(&"pos".into(), &format!("{:?}", pos).into());

  Ok(())
}

pub fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
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
