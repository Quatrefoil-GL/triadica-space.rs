use crate::component::{Component, PackedAttrs, TriadicaElement};
use crate::primes::{DrawMode, VertexData};

pub fn group(children: Vec<TriadicaElement>) -> TriadicaElement {
  TriadicaElement::Group(children)
}

pub fn object(
  draw_mode: DrawMode,
  vertex_shader: String,
  fragment_shader: String,
  packed_attrs: PackedAttrs,
  get_uniforms: Box<dyn Fn() -> VertexData>,
) -> TriadicaElement {
  TriadicaElement::Object(Component {
    draw_mode,
    vertex_shader,
    fragment_shader,
    packed_attrs,
    get_uniforms,
  })
}
