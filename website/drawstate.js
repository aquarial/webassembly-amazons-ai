import { Player } from "./game";

/*jshint esversion: 6 */
// @ts-check

export class DrawState {
    constructor() {
        this.redrawboard = true

        this.tilex = -1;
        this.tiley = -1;

        /** @type {Player} */
        this.piece = null

        /** @type {Pos} */
        this.move = null
    }
}
