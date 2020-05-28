/*jshint esversion: 6 */
// @ts-check

import * as wasm from "amazons-ai-webassembly";


/**
 * @param {CanvasRenderingContext2D} c2d
 * @param {wasm.State} state
 * @param {number} tilesize
 */
export function drawWasmTiles(c2d, state, tilesize) {
  let checker_colors = ["#eae8ea", "#c1c1c1"]

  for (let y = 1; y <= state.size(); y++) {
    for (let x = 1; x <= state.size(); x++) {
      c2d.fillStyle = checker_colors[(x + y) % 2]
      c2d.fillRect((x - 1) * tilesize, (y - 1) * tilesize, tilesize, tilesize)

      let at = state.token(y, x);

      if (at.wall == true) {
        c2d.fillStyle = team_color('block', at.hover);
        drawOneShape(c2d, tilesize, y, x, "block")
      }

      if (at.piece != null) {
        if (at.piece == wasm.DrawableTeam.Gray) {
          c2d.fillStyle = team_color('block', at.hover);
          drawOneShape(c2d, tilesize, y, x, "circle")
        } else {

          c2d.fillStyle = 'white';
          drawOneShape(c2d, tilesize, y, x, "circle outline")

          if (at.piece == wasm.DrawableTeam.Red) {
            c2d.fillStyle = team_color('red', at.hover);
          }
          if (at.piece == wasm.DrawableTeam.Blue) {
            c2d.fillStyle = team_color('blue', at.hover);
          }

          drawOneShape(c2d, tilesize, y, x, "circle")


        }
      }

      at.free();
    }
  }
}


/**
 * @param {CanvasRenderingContext2D} c2d
 * @param {number} tilesize
 * @param {number} y
 * @param {number} x
 * @param {String} style
 */
function drawOneShape(c2d, tilesize, y, x, style) {
  switch (style) {
    case "circle":
      c2d.beginPath();
      c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
        (tilesize) * 0.3, (tilesize) * 0.3, 0, 0, 360);
      c2d.fill();
      break;
    case "circle outline":
      c2d.beginPath();
      c2d.ellipse((x - 1 + 0.5) * tilesize, (y - 1 + 0.5) * tilesize,
        (tilesize * 1.1) * 0.3, (tilesize * 1.1) * 0.3, 0, 0, 360);
      c2d.fill();
      break;
    case "block":
      c2d.fillRect((x - 1 + 0.2) * tilesize, (y - 1 + 0.2) * tilesize, tilesize * 0.6, tilesize * 0.6)
      break;
  }
}

/**
 * @param {string} team_name
 */
function team_color(team_name, faded = false) {
  if (team_name == "deselected") {
    return "#30303055"
  }
  if (team_name == "block") {
    if (faded) {
      return '#30303066'
    } else {
      return '#303030'
    }
  }
  if (team_name == "red") {
    if (faded) {
      return "#ff000066"
    } else {
      return "#ff0000"
    }
  }

  if (team_name == "blue") {
    if (faded) {
      return "#0000ff66"
    } else {
      return "#0000ff"
    }
  }
  throw new Error("Unkown team: " + team_name)
}
