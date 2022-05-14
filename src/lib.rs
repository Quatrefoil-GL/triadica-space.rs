use std::include_str;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

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

  // web_sys::console::log_1(&format!("{:?}", vertices).into());

  let position_attribute_location = context.get_attrib_location(&program, "a_position");
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
    let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

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

  // backcone scale
  let backcone_location = context.get_uniform_location(&program, "coneBackScale");
  context.uniform1f(backcone_location.as_ref(), 0.0);

  // viewportRatio
  let viewport_ratio_location = context.get_uniform_location(&program, "viewportRatio");
  context.uniform1f(viewport_ratio_location.as_ref(), get_window_ratio() as f32);

  // lookPoint
  let look_point_location = context.get_uniform_location(&program, "lookPoint");
  context.uniform3f(look_point_location.as_ref(), 20., 30., -800.0);

  // cameraPosition
  let camera_position_location = context.get_uniform_location(&program, "cameraPosition");
  context.uniform3f(camera_position_location.as_ref(), 300.0, 0.0, 0.0);

  context.bind_vertex_array(Some(&vao));

  let vert_count = (vertices.len() / 3) as i32;
  draw(&context, vert_count);

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

  canvas.set_attribute("width", &inner_width.to_string()).unwrap();
  canvas.set_attribute("height", &inner_height.to_string()).unwrap();

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

pub fn get_window_ratio() -> f64 {
  let window = web_sys::window().unwrap();
  let inner_width = window.inner_width().unwrap().as_f64().unwrap();
  let inner_height = window.inner_height().unwrap().as_f64().unwrap();
  inner_height / inner_width
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
