use std::{cell::RefCell, fmt::Debug, rc::Rc};

use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{
  primes::{DrawMode, VertexData},
  program, ShaderProgramCaches,
};

/// structure in user markups
#[derive(Debug, Clone)]
pub enum TriadicaElement {
  Group(Vec<TriadicaElement>),
  Object(Component),
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

/// definition of user land component
#[derive(Clone)]
pub struct Component {
  pub draw_mode: DrawMode,
  pub vertex_shader: String,
  pub fragment_shader: String,
  pub packed_attrs: PackedAttrs,
  pub get_uniforms: Rc<dyn Fn() -> VertexData>,
}

impl Debug for Component {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("TODO Component: {:?}", self.draw_mode))
  }
}

impl Component {
  /// compile component into a webgl program that can be send to GPU
  pub fn compile_with_caches(&self, context: &WebGl2RenderingContext, caches: Rc<RefCell<ShaderProgramCaches>>) -> ComponentCache {
    ComponentCache {
      draw_mode: self.draw_mode,
      program: program::cached_link_program(context, &self.vertex_shader, &self.fragment_shader, caches).unwrap(),
      arrays: self.packed_attrs.flatten(),
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
  /// collect vertext with mutable data for performance
  pub fn flatten(&self) -> Vec<VertexData> {
    let mut attrs = Vec::new();
    iter_flatten_attributes(self, &mut attrs);
    attrs
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
  /// TODO need buffers
  pub arrays: Vec<VertexData>,
  pub get_uniforms: Rc<dyn Fn() -> VertexData>,
}

impl Debug for ComponentCache {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("TODO ComponentCache: {:?}", self.draw_mode))
  }
}