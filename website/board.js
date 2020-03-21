/*jshint esversion: 6 */
// @ts-check


export class Player {
    /**
     * @param {number} x
     * @param {number} y
     * @param {string} team
     */
    constructor(y, x, team) {
        this.x = x
        this.y = y
        this.team = team
    }

    pos() {
        return `${this.y} ${this.x}`
    }
}

export class GameBoard {
    /**
     *
     * Board goes from (1,1) to (height,width) inclusive
     * @param {number} width
     * @param {number} height
     */
    constructor(width, height) {
        this.width = width
        this.height = height
        /** @type {Map<string,Boolean|Player>} */
        this.blocked = new Map()
    }

    /**
     * @param {number} y
     * @param {number} x
     */
    atPos(y, x) {
       return  this.blocked.get(`${y} ${x}`)
    }

    /**
     * @param {Player} player
     */
    addPlayer(player) {
        this.blocked.set(player.pos(), player)
    }
}
