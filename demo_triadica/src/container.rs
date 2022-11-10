use triadica::DrawMode;
use triadica::VertexDataValue;
use triadica::{group, object, PackedAttrs, TriadicaElement};
// use web_sys::console::log_1;

use std::collections::HashMap;
use std::rc::Rc;

use crate::shape::compute_lamp_tree_vertices;

pub fn container() -> TriadicaElement {
  let vert_shader = include_str!("../shaders/demo.vert");
  let frag_shader = include_str!("../shaders/demo.frag");

  let vertices = compute_lamp_tree_vertices();
  let mut data = vec![];
  for v in vertices {
    data.push(PackedAttrs::Item(HashMap::from_iter([(
      "a_position".to_string(),
      VertexDataValue::Vec3(v),
    )])));
  }
  let lamp_attrs = PackedAttrs::List(data);

  group(vec![
    object(
      DrawMode::LineStrip,
      vert_shader.to_owned(),
      frag_shader.to_owned(),
      // creating a list of points
      PackedAttrs::List(vec![
        PackedAttrs::Item(HashMap::from_iter([("a_position".to_owned(), VertexDataValue::Vec3([0., 0., 0.]))])),
        PackedAttrs::Item(HashMap::from_iter([(
          "a_position".to_owned(),
          VertexDataValue::Vec3([100., 0., 0.]),
        )])),
        PackedAttrs::Item(HashMap::from_iter([(
          "a_position".to_owned(),
          VertexDataValue::Vec3([0., 100., 0.]),
        )])),
        PackedAttrs::Item(HashMap::from_iter([("a_position".to_owned(), VertexDataValue::Vec3([0., 0., 0.]))])),
      ]),
      Rc::new(HashMap::new),
    ),
    object(
      DrawMode::LineStrip,
      vert_shader.to_owned(),
      frag_shader.to_owned(),
      lamp_attrs,
      Rc::new(HashMap::new),
    ),
  ])
}