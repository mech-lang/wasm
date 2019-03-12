#![feature(alloc)]
#![feature(drain_filter)]
extern crate wasm_bindgen;
extern crate hashbrown;
//extern crate web_sys;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate serde_derive;
extern crate core;
extern crate web_sys;
extern crate mech_core;
extern crate mech_syntax;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use hashbrown::hash_set::HashSet;
use alloc::vec::Vec;
use core::fmt;
use mech_syntax::compiler::Compiler;
use mech_core::{Transaction, Hasher, Change, Index, Value, Table, Quantity, ToQuantity, QuantityMath};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Core {
  core: mech_core::Core,
  changes: Vec<Change>,
}

#[wasm_bindgen]
impl Core {

  pub fn new() -> Core {
    Core {
      core: mech_core::Core::new(100_000,100),
      changes: Vec::new(),
    }
  }

  pub fn compile_code(&mut self, code: String) {
    let mut compiler = Compiler::new();
    compiler.compile_string(code);
    self.core.register_blocks(compiler.blocks.clone());
    self.core.step();
    log!("Compiled {} blocks.", compiler.blocks.len());
  }


  pub fn clear(&mut self) {
    self.core.clear();
    log!("Core Cleared");
  }

  pub fn pause(&mut self) {
    self.core.pause();
    log!("Core Paused");
  }

  pub fn resume(&mut self) {
    self.core.resume();
    log!("Core Resumed");
  }

  pub fn step_back_one(&mut self) {
    self.core.step_back_one();
    log!("Core Time -{}", self.core.offset);
  }

  pub fn step_forward_one(&mut self) {
    self.core.step_forward_one();
    log!("Core Time -{}", self.core.offset);
  }

  pub fn display_core(&mut self) {
    log!("{:?}", self.core);
  }

  pub fn display_runtime(&mut self) {
    log!("{:?}", self.core.runtime);
  }

  pub fn queue_change(&mut self, table: String, row: u32, column: u32, value: u32) {
    let table_id = Hasher::hash_string(table);
    let change = Change::Set{table: table_id, 
                             row: Index::Index(row as u64), 
                             column: Index::Index(column as u64),
                             value: Value::from_u64(value as u64),
                            };
    self.changes.push(change);
  }

  pub fn process_transaction(&mut self) {
    if !self.core.paused {
        let txn = Transaction::from_changeset(self.changes.clone());
        //log!("{:?}", txn);
        self.core.process_transaction(&txn);
    }
    self.changes.clear();
  }

  pub fn get_mantissas(&mut self, table: u64, column: u64) -> Vec<i64> {
      let mut output: Vec<i64> = vec![];
      match self.core.store.get_column(table, Index::Index(column)) {
          Some(column) => {
              for row in column {
                  output.push(row.as_quantity().unwrap().mantissa());
              }
          }
          _ => log!("{} not found", table),
      }
      output
  }

  pub fn get_ranges(&mut self, table: u64, column: u64) -> Vec<i64> {
      let mut output: Vec<i64> = vec![];
      match self.core.store.get_column(table, Index::Index(column)) {
          Some(column) => {
              for row in column {
                  output.push(row.as_quantity().unwrap().range());
              }
          }
          _ => log!("{} not found", table),
      }
      output
  }

  pub fn get_column(&mut self, table: u64, column: u64) -> Vec<f64> {
      let mut output: Vec<f64> = vec![];
      match self.core.store.get_column(table, Index::Index(column)) {
          Some(column) => {
              for row in column {
                  output.push(row.as_quantity().unwrap().to_float());
              }
          }
          _ => log!("{} not found", table),
      }
      output
  }

  pub fn add_application(&self) -> Result<(), JsValue> {
    let table_id = Hasher::hash_str("app/main");
    match self.core.store.get_table(table_id) {
      Some(app_table) => {

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let drawing_area = document.get_element_by_id("drawing").unwrap();
        let mut app = document.create_element("div")?;
        let contents_id = app_table.data[1][0].as_u64().unwrap();
        let contents_table = self.core.store.get_table(contents_id).unwrap();
        for i in 0..contents_table.rows {
          let tag = &contents_table.data[0][i as usize].as_string().unwrap();
          match tag.as_ref() {
            "div" => {
              let value = &contents_table.data[1][i as usize].as_string().unwrap();
              let mut div = document.create_element(tag)?;
              div.set_inner_html(value);
              div.set_attribute("class","well");
              app.append_child(&div)?;
            },
            "img" => {
              let value = &contents_table.data[1][i as usize].as_string().unwrap();
              let mut img = web_sys::HtmlImageElement::new().unwrap();
              img.set_src(value);
              img.set_attribute("class","logo");
              app.append_child(&img)?;
            },
            _ => (),
          }
          
          

        }
        drawing_area.append_child(&app)?;
      }
      _ => (),
    }

    Ok(())
  }

  fn draw_contents(&self) -> Result<(), JsValue> {
      Ok(())

  }

  pub fn add_canvas(&self) -> Result<(), JsValue> {
    let table_id = Hasher::hash_str("html/canvas");
    match self.core.store.get_table(table_id) {
      Some(table) => {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        let drawing_area = document.get_element_by_id("drawing").unwrap();
        let canvas = document.create_element("canvas")?;
        canvas.set_attribute("id","drawing canvas");
        canvas.set_attribute("width", &format!("{}", table.data[1][0].as_float().unwrap()));
        canvas.set_attribute("height", &format!("{}", table.data[0][0].as_float().unwrap()));
        canvas.set_attribute("style", "background-color: rgb(255, 255, 255)");
        drawing_area.append_child(&canvas)?;
        self.render_balls();
      }
      _ => (),
    }

    Ok(())
  }

  pub fn render_balls(&self) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let canvas = document.get_element_by_id("drawing canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let radius = 10.0;

    let table_id = Hasher::hash_str("html/canvas");
    let table = self.core.store.get_table(table_id).unwrap();
    let elements_table_id = table.data[2][0].as_u64().unwrap();
    let elements_table = self.core.store.get_table(elements_table_id).unwrap();

    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    for i in 0..elements_table.rows {
      context.begin_path();
      match elements_table.data[0][i as usize] {
        Value::String(ref shape) => {
          match shape.as_ref() {
            "circle" => {
              let x = elements_table.data[1][i as usize].as_float().unwrap();
              let y = elements_table.data[2][i as usize].as_float().unwrap();
              let radius = elements_table.data[3][i as usize].as_float().unwrap();
              context.arc(x, y, radius, 0.0, 2.0 * 3.14);
              context.set_fill_style(&JsValue::from_str("#0B79CE"));
              context.fill();  
            },
            "line" => {
              let x1 = elements_table.data[1][i as usize].as_float().unwrap();
              let y1 = elements_table.data[2][i as usize].as_float().unwrap();
              let x2 = elements_table.data[4][i as usize].as_float().unwrap();
              let y2 = elements_table.data[5][i as usize].as_float().unwrap();
              
              context.begin_path();
              context.move_to(x1, y1);
              context.line_to(x2, y2);
              context.close_path();
              context.stroke();
            },
            _ => (),
          }
        },    
        _ => (),    
      }


    }
    Ok(())
  } 

  pub fn list_global_tables(&self) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let table_list_div = document.create_element("div")?;
    let table_list = document.create_element("ul")?;
    for (table_name, table) in self.core.store.tables.map.iter() {
      let table_list_item = document.create_element("li")?;
      table_list_item.set_inner_html(self.core.store.names.get(table_name).unwrap());
      table_list.append_child(&table_list_item)?;
    }
    table_list_div.append_child(&table_list)?;
    body.append_child(&table_list_div)?;
    Ok(())
  }

}

#[wasm_bindgen]
pub fn hash_string(input: String) -> u64 {
    Hasher::hash_string(input)
} 