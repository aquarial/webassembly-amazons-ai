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
pub struct RequestedBoard {
  pub width:   f64,
  pub height:  f64,
  blocks:  Vec<(f64, f64)>,
  red_team: Vec<(f64, f64)>,
  blue_team: Vec<(f64, f64)>,
}

#[wasm_bindgen]
impl RequestedBoard {
  pub fn add_red_team(&mut self, y:f64, x:f64) {self.red_team.push((y,x));}
  pub fn add_blue_team(&mut self, y:f64, x:f64) {self.blue_team.push((y,x));}
  pub fn add_block(&mut self,  y:f64, x:f64) {self.blocks.push((y,x));}

  pub fn new() -> RequestedBoard {
    RequestedBoard {
      width: 0.0,
      height: 0.0,
      red_team: Vec::new(),
      blue_team: Vec::new(),
      blocks: Vec::new(),
    }
  }


  pub fn is_valid(&self) -> bool {
    // [1, width]  [1, height]
    if !is_int_in_range(self.width, (1.0, 10.0)) {
      return false;
    }
    if !is_int_in_range(self.height, (1.0, 10.0)) {
      return false;
    }

    for &(y,x) in &self.red_team {
      if !is_int_in_range(y, (1.0, self.height)) || !is_int_in_range(x, (1.0, self.width)) {
        return false;
      }
    }

    for &(y,x) in &self.blue_team {
      if !is_int_in_range(y, (1.0, self.height)) || !is_int_in_range(x, (1.0, self.width)) {
        return false;
      }
    }

    for &(y,x) in &self.blocks {
      if !is_int_in_range(y, (1.0, self.height)) || !is_int_in_range(x, (1.0, self.width)) {
        return false;
      }
    }

    return true;
  }
}

fn is_int_in_range(val: f64, range:(f64, f64)) -> bool {
  if !(range.0 <= val && val <= range.1) {
    return false;
  }
  if val.floor() != val {
    return false;
  }
  return true;
}
