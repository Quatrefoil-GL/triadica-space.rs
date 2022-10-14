//! this file builds shader program, and it uses caches to skip duplications

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct ShaderProgramCaches {
  v: HashMap<String, WebGlProgram>,
}

impl Default for ShaderProgramCaches {
  fn default() -> Self {
    Self { v: HashMap::new() }
  }
}

fn link_program(context: &WebGl2RenderingContext, vert_shader: &str, frag_shader: &str) -> Result<WebGlProgram, String> {
  let program = context
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  let vert_shader = compile_shader(context, WebGl2RenderingContext::VERTEX_SHADER, vert_shader)?;

  let frag_shader = compile_shader(context, WebGl2RenderingContext::FRAGMENT_SHADER, frag_shader)?;

  context.attach_shader(&program, &vert_shader);
  context.attach_shader(&program, &frag_shader);
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

pub fn cached_link_program(
  context: &WebGl2RenderingContext,
  vert_shader: &str,
  frag_shader: &str,
  caches: Rc<RefCell<ShaderProgramCaches>>,
) -> Result<WebGlProgram, String> {
  let key = format!("{}\n@@@@\n{}", vert_shader, frag_shader);

  let mut p = (*caches).borrow_mut();
  if let Some(program) = p.v.get(&key) {
    Ok(program.clone())
  } else {
    let program = link_program(context, vert_shader, frag_shader)?;
    p.v.insert(key, program.clone());
    Ok(program)
  }
}

fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
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
