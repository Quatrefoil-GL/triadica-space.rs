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

/// collection of key/value pairs
pub type VertexData = Vec<VertexDataValue>;

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
}
