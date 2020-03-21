// @ts-check
/* jshint -W069, esversion:6 */

import * as wasm from "amazons-ai-webassembly";

let animations = [];

let canvas = /** @type {HTMLCanvasElement} */ (document.getElementById("thecanvas"));
let c2d = canvas.getContext("2d");

let time = 0;
let marker = 0;
/** @param {number} dt */
animations.push(dt => {
  time += dt;
  if (time > marker) {
    console.log(marker, "seconds");
    marker += 5;
  }
});


let previous = performance.now();
function animLoop() {
  let next = performance.now();
  let deltaTime = (next - previous) / 1000.0;
  previous = next;

  animations.forEach(it => {
    it(deltaTime);
  });

  window.requestAnimationFrame(animLoop);
}
animLoop();
