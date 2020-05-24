// @ts-check
/* jshint -W069, esversion:6 */

import * as wasm from "amazons-ai-webassembly";
import { GameBoard, Player, GameState, Pos } from "./game.js";
import { drawWasmTiles } from "./drawstate.js";

/** @type {((dt: number, totaltime: number) => void)[]} */
let animations = [];

{
  /** @type {HTMLCanvasElement} */
  let canvas = (document.getElementById("thecanvas"));
  let c2d = canvas.getContext("2d");

  /** @type {HTMLCanvasElement} */
  let next2go = (document.getElementById("next2go"));
  let next2go_2d = next2go.getContext("2d");

  let state = wasm.State.new();

  /** @type {HTMLButtonElement} */
  let undo = (document.getElementById("undo"));
  undo.onmousedown = function () {
    state.undo();
  }
  /** @type {HTMLButtonElement} */
  let newgame = (document.getElementById("newgame"));
  newgame.onmousedown = function () {
    state.new_game();
  }
  /** @type {HTMLButtonElement} */
  let makeai = (document.getElementById("makeai"));
  makeai.onmousedown = function (event) {
    state.ai_move();
  }

  canvas.onmouseleave = function (event) {
    state.mouse_leave();
  }

  // will be used for animations
  canvas.onmousemove = (function (event) {
    let tilesize = canvas.width / state.size();
    let tx = Math.floor(event.offsetX / tilesize) + 1;
    let ty = Math.floor(event.offsetY / tilesize) + 1;
    state.mouse_move(ty, tx);
  })

  // click handler
  canvas.onmousedown = function (event) {
    let tilesize = canvas.width / state.size();

    let tx = Math.floor(event.offsetX / tilesize) + 1;
    let ty = Math.floor(event.offsetY / tilesize) + 1;
    state.mouse_click(ty, tx);
  }

  animations.push((dt, totaltime) => {
    if (state.turn() == wasm.DrawableTeam.Red)
      next2go_2d.fillStyle = "red";
    else 
      next2go_2d.fillStyle = "blue";
    next2go_2d.fillRect(0, 0, next2go.width, next2go.height)

    drawWasmTiles(c2d, state, canvas.width / state.size());
  })
}


{
  let marker = 0;
  animations.push((dt, totaltime) => {
    if (totaltime > marker) {
      //console.log(marker, "seconds");
      marker += 5;
    }
  });
}



let previous = performance.now();
let totaltime = 0;

function animLoop() {
  let next = performance.now();
  let deltaTime = (next - previous) / 1000.0;
  totaltime += deltaTime;
  previous = next;

  animations.forEach(it => {
    it(deltaTime, totaltime);
  });

  window.requestAnimationFrame(animLoop);
}
animLoop();
