extern crate demo_triadica_space;

use demo_triadica_space::container;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
// use web_sys::console::{log_1, log_2};
use container::container;

pub fn main() -> Result<(), JsValue> {
  println!("status ready");

  let tree = Rc::new(RefCell::new(container().compile_to_tree()?));
  println!("flatterned {}", (*tree.borrow_mut()).to_list().len());

  Ok(())
}
