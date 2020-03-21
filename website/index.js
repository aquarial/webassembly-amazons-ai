// @ts-check
/* jshint -W069, esversion:6 */

import * as wasm from "amazons-ai-webassembly";
import { GameBoard, Player } from "./board.js";

/** @type {((dt: number, totaltime: number) => void)[]} */
let animations = [];

let gameboard = new GameBoard(8, 8);
gameboard.addPlayer(new Player(3, 3, "red"));
gameboard.addPlayer(new Player(3, 6, "red"));
gameboard.addPlayer(new Player(6, 3, "blue"));
gameboard.addPlayer(new Player(6, 6, "blue"));

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
