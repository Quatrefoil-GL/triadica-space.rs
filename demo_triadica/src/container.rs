use triadica::DrawMode;
use triadica::VertexDataValue;
use triadica::{group, object, PackedAttrs, TriadicaElement};
use web_sys::console::log_1;

use std::rc::Rc;

use crate::shape::compute_lamp_tree_vertices;

pub fn container() -> TriadicaElement {
  let vert_shader = include_str!("../shaders/demo.vert");
  let frag_shader = include_str!("../shaders/demo.frag");

  log_1(&"building".into());

  let lamp_attrs = compute_lamp_tree_vertices();

  log_1(&"finished building".into());

  group(vec![
    object(
      DrawMode::LineStrip,
      vert_shader.to_owned(),
      frag_shader.to_owned(),
      // creating a list of points
      vec![("a_position".to_owned(), 3)],
      PackedAttrs::List(vec![
        PackedAttrs::Item(vec![VertexDataValue::Vec3([0., 0., 0.])]),
        PackedAttrs::Item(vec![VertexDataValue::Vec3([100., 0., 0.])]),
        PackedAttrs::Item(vec![VertexDataValue::Vec3([0., 100., 0.])]),
        PackedAttrs::Item(vec![VertexDataValue::Vec3([0., 0., 0.])]),
      ]),
      Rc::new(Vec::new),
    ),
    object(
      DrawMode::LineStrip,
      vert_shader.to_owned(),
      frag_shader.to_owned(),
      vec![("a_position".to_owned(), 3)],
      lamp_attrs,
      Rc::new(Vec::new),
    ),
  ])
}
