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
  attr_names: Vec<(String, i8)>,
  packed_attrs: PackedAttrs,
  get_uniforms: Rc<dyn Fn() -> VertexData>,
) -> TriadicaElement {
  TriadicaElement::Object(ComponentOptions {
    draw_mode,
    vertex_shader,
    fragment_shader,
    attr_names,
    packed_attrs,
    get_uniforms,
  })
}
