use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum DrawMode {
  Triangles,
  Lines,
  LineStrip,
  TriangleStrip,
}

pub type VertexData = HashMap<String, VertexDataValue>;

#[derive(Debug, Clone)]
pub enum VertexDataValue {
  Float(f32),
  Vec2([f32; 2]),
  Vec3([f32; 3]),
  Vec4([f32; 4]),
}
