mod viewer;

use std::cell::RefCell;
use std::fmt::format;
use std::fmt::Debug;
use std::include_str;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console::{log_1, log_2};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

lazy_static::lazy_static! {
  static ref WINDOW_RATIO: RwLock<f32> = RwLock::new(1.0);
}

#[wasm_bindgen(js_name = initApp)]
pub fn init_app() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  let window = web_sys::window().unwrap();

  let document = window.document().unwrap();
  let canvas = document.query_selector(".app").unwrap().unwrap();

  on_window_resize()?;

  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

  let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;

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

  let vertices = compute_vertices();

  bind_attributes(&context, &program, &vertices).unwrap();

  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  let copied_context = Rc::new(context.to_owned());
  let copied_program = Rc::new(program.to_owned());
  let vertices_count = (vertices.len() / 3) as i32;
  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if viewer::requested_rendering() {
      bind_uniforms(&*copied_context, &*copied_program).unwrap();
      draw(&context, vertices_count);
    }

    // Schedule ourself for another requestAnimationFrame callback.
    request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());

  // bind_uniforms(&*copied_context, &program).unwrap();

  // draw(&*copied_context, vertices_count);

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
  // backcone scale
  let backcone_location = context.get_uniform_location(program, "coneBackScale");
  context.uniform1f(backcone_location.as_ref(), 0.5);

  // viewportRatio
  let viewport_ratio_location = context.get_uniform_location(program, "viewportRatio");
  let window_ratio = *WINDOW_RATIO.read().unwrap();
  context.uniform1f(viewport_ratio_location.as_ref(), window_ratio as f32);

  // lookPoint
  let look_point_location = context.get_uniform_location(program, "lookPoint");
  let lookat = viewer::new_lookat_point();
  log_2(&"lookat".into(), &format!("{:?}", lookat).into());
  context.uniform3f(look_point_location.as_ref(), lookat.0, lookat.1, lookat.2);

  // cameraPosition
  let camera_position_location = context.get_uniform_location(program, "cameraPosition");
  let pos = viewer::get_position();
  log_2(&"pos".into(), &format!("{:?}", pos).into());
  context.uniform3f(camera_position_location.as_ref(), pos.0, pos.1, pos.2);

  Ok(())
}

pub fn compute_vertices() -> Vec<f32> {
  let geo: Vec<[f32; 3]> = vec![
    [-0.5, -0.5, 0.0],
    [-0.5, 0.5, 0.0],
    [0.5, 0.5, 0.0],
    [0.5, -0.5, 0.0],
    [-0.5, -0.5, -1.0],
    [-0.5, 0.5, -1.0],
    [0.5, 0.5, -1.0],
    [0.5, -0.5, -1.0],
  ];

  let indices = vec![0, 1, 1, 2, 2, 3, 3, 0, 0, 4, 1, 5, 2, 6, 3, 7, 4, 5, 5, 6, 6, 7, 7, 4];
  let mut points: Vec<[f32; 3]> = Vec::new();
  for i in 0..indices.len() {
    points.push(geo[indices[i]]);
  }

  let moved_points: Vec<_> = points.iter().map(|p| [p[0] * 400., p[1] * 400., p[2] * 400. - 1200.]).collect();
  let mut vertices: Vec<f32> = Vec::new();
  for p in moved_points {
    vertices.extend_from_slice(&p);
  }
  vertices
}

#[wasm_bindgen(js_name = onWindowResize)]
pub fn on_window_resize() -> Result<(), JsValue> {
  let window = web_sys::window().unwrap();
  let canvas = window.document().unwrap().query_selector(".app").unwrap().unwrap();

  let inner_width = window.inner_width().unwrap().as_f64().unwrap();
  let inner_height = window.inner_height().unwrap().as_f64().unwrap();

  let mut r = WINDOW_RATIO.write().unwrap();
  *r = (inner_height / inner_width) as f32;

  web_sys::console::log_1(&format!("{} {}", inner_height, inner_width).into());

  canvas.set_attribute("width", &format!("{}px", inner_width)).unwrap();
  canvas.set_attribute("height", &format!("{}px", inner_height)).unwrap();
  // canvas
  //   .set_attribute("style", &format!("width: {}px; height: {}px;", inner_width, inner_height))
  //   .unwrap();

  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;
  context.viewport(0, 0, inner_width as i32, inner_height as i32);

  viewer::mark_dirty();

  Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
  context.clear_color(0.0, 0.0, 0.0, 1.0);
  context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

  context.draw_arrays(WebGl2RenderingContext::LINES, 0, vert_count);
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

#[wasm_bindgen(js_name = "onControl")]
pub fn on_control(
  elapsed: f32,
  left_move_x: f32,
  left_move_y: f32,
  right_move_x: f32,
  right_move_y: f32,
  right_delta_x: f32,
  right_delta_y: f32,
  left_a: bool,
  resetting: bool,
) -> Result<(), JsValue> {
  if left_move_y.abs() > std::f32::EPSILON {
    viewer::move_viewer_by((0., 0., -left_move_y * 2. * elapsed));
  }
  if left_move_x.abs() > std::f32::EPSILON {
    viewer::rotate_view_by(-0.01 * elapsed * left_move_x);
  }
  if !left_a && (right_move_x.abs() > std::f32::EPSILON || right_move_y.abs() > std::f32::EPSILON) {
    viewer::move_viewer_by((right_move_x * 2. * elapsed, right_move_y * 2. * elapsed, 0.));
  }
  if left_a && right_delta_y.abs() > std::f32::EPSILON {
    viewer::shift_viewer_by(1. * elapsed * right_delta_y);
  }
  if left_a && right_delta_x.abs() > std::f32::EPSILON {
    viewer::rotate_view_by(-0.1 * elapsed * right_delta_x);
  }

  web_sys::console::log_1(&format!("resttin: {} {}", left_a, resetting).into());
  if resetting {
    let shift_y = viewer::get_shift_y();
    if shift_y < -0.06 {
      viewer::shift_viewer_by(2. * elapsed);
    } else if shift_y > 0.06 {
      viewer::shift_viewer_by(-2. * elapsed);
    } else {
      viewer::reset_shift_y();
    }
  }

  Ok(())
}
