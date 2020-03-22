mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
struct RequestedBoard {
  width:   f64,
  height:  f64,
  players: Vec<(f64, f64)>,
  blocks:  Vec<(f64, f64)>,
}

#[wasm_bindgen]
impl RequestedBoard {
  pub fn board_greet(&mut self) {
    alert(&format!("{} {} {:?} {:?}",
                   self.width, self.height, self.players, self.blocks
    ));
  }
  pub fn set_width(&mut self,  x:f64) {self.width = x;}
  pub fn set_height(&mut self, x:f64) {self.height = x;}
  pub fn add_player(&mut self, y:f64, x:f64) {self.players.push((y,x));}
  pub fn add_block(&mut self,  y:f64, x:f64) {self.blocks.push((y,x));}

  pub fn new() -> RequestedBoard {
    RequestedBoard {
      width: 0.0,
      height: 0.0,
      players: Vec::new(),
      blocks: Vec::new(),
    }
  }
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello, {{project-name}}!");
}
