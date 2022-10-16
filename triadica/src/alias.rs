use std::rc::Rc;

use crate::component::{ComponentOptions, PackedAttrs, TriadicaElement};
use crate::primes::{DrawMode, VertexData};

pub fn group(children: Vec<TriadicaElement>) -> TriadicaElement {
  TriadicaElement::Group(children)
}

pub fn object(
  draw_mode: DrawMode,
  vertex_shader: String,
  fragment_shader: String,
  packed_attrs: PackedAttrs,
  get_uniforms: Rc<dyn Fn() -> VertexData>,
) -> TriadicaElement {
  TriadicaElement::Object(ComponentOptions {
    draw_mode,
    vertex_shader,
    fragment_shader,
    packed_attrs,
    get_uniforms,
  })
}
