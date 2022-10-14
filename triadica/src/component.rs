use web_sys::WebGlProgram;

use crate::primes::{DrawMode, VertexData};

pub enum TriadicaElement {
  Group(Vec<TriadicaElement>),
  Object(Component),
}

/// definition of user land component
pub struct Component {
  pub draw_mode: DrawMode,
  pub vertex_shader: String,
  pub fragment_shader: String,
  pub packed_attrs: PackedAttrs,
  pub get_uniforms: Box<dyn Fn() -> VertexData>,
}

/// structure to hold nested attributes
pub enum PackedAttrs {
  List(Vec<PackedAttrs>),
  Item(VertexData),
}

/// collect vertext with mutable data for performance
fn flatten_attributes(packed_attrs: PackedAttrs) -> Vec<VertexData> {
  let mut attrs = Vec::new();
  iter_flatten_attributes(packed_attrs, &mut attrs);
  attrs
}

fn iter_flatten_attributes(packed_attrs: PackedAttrs, attrs: &mut Vec<VertexData>) {
  match packed_attrs {
    PackedAttrs::List(list) => {
      for item in list {
        iter_flatten_attributes(item, attrs);
      }
    }
    PackedAttrs::Item(item) => {
      attrs.push(item);
    }
  }
}

/// cached struct for compiled shaders
pub struct ComponentCache {
  pub draw_mode: DrawMode,
  pub program: WebGlProgram,
  pub arrays: Vec<VertexData>,
  pub get_uniforms: Box<dyn Fn() -> VertexData>,
}
