// @ts-check
/* jshint -W069, esversion:6 */

import * as wasm from "amazons-ai-webassembly";
import { GameBoard, Player } from "./board.js";

/** @type {((dt: number, totaltime: number) => void)[]} */
let animations = [];


{

  /** @type {HTMLCanvasElement} */
  let canvas = (document.getElementById("thecanvas"));
  let c2d = canvas.getContext("2d");

  let gameboard = new GameBoard(8, 8);
  gameboard.addPlayer(new Player(3, 3, "red"));
  gameboard.addPlayer(new Player(3, 6, "red"));
  gameboard.addPlayer(new Player(6, 3, "blue"));
  gameboard.addPlayer(new Player(6, 6, "blue"));

  let tilesize = canvas.width / gameboard.width;
  let checker_colors = ["brown", "gray"]

  for (let y = 1; y <= gameboard.height; y++) {
    for (let x = 1; x <= gameboard.width; x++) {
      c2d.fillStyle = checker_colors[(x + y) % 2]
      c2d.fillRect((x - 1) * tilesize, (y - 1) * tilesize, tilesize, tilesize)
    }
  }

  animations.push((dt, totaltime) => {

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
