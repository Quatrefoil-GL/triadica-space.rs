use std::{cell::RefCell, fmt::Debug, rc::Rc};

use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{
  primes::{DrawMode, VertexData},
  program, ShaderProgramCaches, VertexDataValue,
};

/// structure in user markups
#[derive(Debug, Clone)]
pub enum TriadicaElement {
  Group(Vec<TriadicaElement>),
  Object(ComponentOptions),
}

impl TriadicaElement {
  /// compile from markup to data for webgl program
  pub fn compile_to_tree(
    &self,
    context: &WebGl2RenderingContext,
    caches: Rc<RefCell<ShaderProgramCaches>>,
  ) -> Result<TriadicaElementTree, String> {
    match self {
      TriadicaElement::Group(children) => {
        let children = children
          .iter()
          .map(|child| child.compile_to_tree(context, caches.clone()))
          .collect::<Result<Vec<_>, _>>()?;
        Ok(TriadicaElementTree::Group(children))
      }
      TriadicaElement::Object(component) => Ok(TriadicaElementTree::Object(component.compile_with_caches(context, caches))),
    }
  }
}

/// structure after compilation
#[derive(Debug, Clone)]
pub enum TriadicaElementTree {
  Group(Vec<TriadicaElementTree>),
  Object(ComponentCache),
}

impl TriadicaElementTree {
  /// TODO need iter for better performance, reduce cloning
  pub fn to_list(&self) -> Vec<ComponentCache> {
    let mut result: Vec<ComponentCache> = Vec::new();
    match self {
      TriadicaElementTree::Group(xs) => {
        for x in xs {
          result.extend_from_slice(&x.to_list())
        }
      }
      TriadicaElementTree::Object(x) => result.push(x.to_owned()),
    }
    result
  }
}

/// definition of user land component
#[derive(Clone)]
pub struct ComponentOptions {
  pub draw_mode: DrawMode,
  pub vertex_shader: String,
  pub fragment_shader: String,
  pub attr_names: Vec<(String, i8)>,
  pub packed_attrs: PackedAttrs,
  pub get_uniforms: Rc<dyn Fn() -> VertexData>,
}

impl Debug for ComponentOptions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("TODO Component options: {:?}", self.draw_mode))
  }
}

impl ComponentOptions {
  /// compile component into a webgl program that can be send to GPU
  pub fn compile_with_caches(&self, context: &WebGl2RenderingContext, caches: Rc<RefCell<ShaderProgramCaches>>) -> ComponentCache {
    ComponentCache {
      draw_mode: self.draw_mode,
      program: program::cached_link_program(context, &self.vertex_shader, &self.fragment_shader, caches).unwrap(),
      attr_names: self.attr_names.clone(),
      arrays: self.packed_attrs.flatten(),
      size: self.packed_attrs.len(),
      get_uniforms: self.get_uniforms.clone(),
    }
  }
}

#[derive(Debug, Clone)]
/// structure to hold nested attributes
pub enum PackedAttrs {
  List(Vec<PackedAttrs>),
  Item(VertexData),
}

impl PackedAttrs {
  pub fn is_empty(&self) -> bool {
    false
  }
  pub fn len(&self) -> usize {
    match self {
      Self::Item(_) => 1,
      Self::List(xs) => {
        let mut x = 0;
        for i in xs {
          x += i.len();
        }
        x
      }
    }
  }

  /// collect vertext with mutable data for performance
  pub fn flatten(&self) -> Vec<Vec<f32>> {
    let mut attrs = Vec::new();
    iter_flatten_attributes(self, &mut attrs);

    if attrs.is_empty() {
      Vec::new()
    } else {
      // TODO for performance, need to reduce allocation
      let a0 = &attrs[0];
      let mut result = Vec::new();
      for (idx, _record) in a0.iter().enumerate() {
        let mut values: Vec<f32> = vec![];
        for attr in attrs.iter() {
          if attr.len() == a0.len() {
            match &attr[idx] {
              VertexDataValue::Float(f) => values.push(*f),
              VertexDataValue::Vec2(v) => values.extend_from_slice(v),
              VertexDataValue::Vec3(v) => values.extend_from_slice(v),
              VertexDataValue::Vec4(v) => values.extend_from_slice(v),
            }
          } else {
            unreachable!("unexpected length")
          }
        }
        result.push(values.to_owned());
      }
      result
    }
  }

  /// get a sample of vertex data
  pub fn peek(&self) -> Option<VertexData> {
    match self {
      PackedAttrs::Item(x) => Some(x.to_owned()),
      PackedAttrs::List(xs) => {
        if xs.is_empty() {
          None
        } else {
          xs[0].peek()
        }
      }
    }
  }
}

fn iter_flatten_attributes(packed_attrs: &PackedAttrs, attrs: &mut Vec<VertexData>) {
  match packed_attrs {
    PackedAttrs::List(list) => {
      for item in list {
        iter_flatten_attributes(item, attrs);
      }
    }
    PackedAttrs::Item(item) => {
      attrs.push(item.to_owned());
    }
  }
}

/// cached struct for compiled shaders
#[derive(Clone)]
pub struct ComponentCache {
  pub draw_mode: DrawMode,
  pub program: WebGlProgram,
  pub attr_names: Vec<(String, i8)>,
  /// TODO need buffers
  pub arrays: Vec<Vec<f32>>,
  pub size: usize,
  pub get_uniforms: Rc<dyn Fn() -> VertexData>,
}

impl Debug for ComponentCache {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("TODO ComponentCache: {:?}", self.draw_mode))
  }
}
