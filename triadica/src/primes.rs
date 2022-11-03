use std::{cell::RefCell, collections::HashMap, rc::Rc};

use web_sys::WebGl2RenderingContext;

#[derive(Clone, Copy, Debug)]
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

pub type VertexData = HashMap<String, VertexDataValue>;

#[derive(Debug, Clone)]
pub enum VertexDataValue {
  Float(f32),
  Vec2([f32; 2]),
  Vec3([f32; 3]),
  Vec4([f32; 4]),
}

impl VertexDataValue {
  pub fn is_empty(&self) -> bool {
    false
  }

  pub fn len(&self) -> usize {
    match self {
      VertexDataValue::Float(_) => 1,
      VertexDataValue::Vec2(_) => 2,
      VertexDataValue::Vec3(_) => 3,
      VertexDataValue::Vec4(_) => 4,
    }
  }

  pub fn push_to(&self, xs: Rc<RefCell<Vec<f32>>>) {
    match self {
      VertexDataValue::Float(x) => xs.borrow_mut().push(*x),
      VertexDataValue::Vec2(x) => xs.borrow_mut().extend_from_slice(x),
      VertexDataValue::Vec3(x) => xs.borrow_mut().extend_from_slice(x),
      VertexDataValue::Vec4(x) => xs.borrow_mut().extend_from_slice(x),
    }
  }
}
