import * as wasm from "amazons-ai-webassembly";



onmessage = (ev) => {
    console.log("run-ai.js received", ev, ev.data);
    postMessage("procceseeeeeeeeeee")
}


// let board = wasm.RequestedBoard.new();
// board.size = gameboard.width;
// for (let y = 1; y <= gameboard.height; y++) {
//     for (let x = 1; x <= gameboard.width; x++) {
//         let at = gameboard.atYX(y, x);
//         if (at instanceof Player) {
//             if (at.team == gamestate.next_to_go) {
//                 board.add_red_team(at.pos.y, at.pos.x);
//             } else {
//                 board.add_blue_team(at.pos.y, at.pos.x);
//             }
//         } else if (at != undefined) {
//             board.add_block(y, x)
//         }
//     }
// }

// if (board.is_valid()) {
//     let r = wasm.compute_ai_move(board)
//     console.log(r.piece_y, r.piece_x, r.move_y, r.move_x, r.stone_y, r.stone_x)
//     if (r.piece_y > 0 && r.piece_x > 0 &&
//         r.move_y > 0 && r.move_x > 0 &&
//         r.stone_y > 0 && r.stone_x > 0) {
//         let p0 = gameboard.atPos(new Pos(r.piece_y, r.piece_x))
//         let p1 = new Pos(r.move_y, r.move_x)
//         let p2 = new Pos(r.stone_y, r.stone_x)
//         gamestate.addMove(p0, p1, p2)
//         gameboard.makePlayerMove(p0, p1, p2)
//         drawstate.piece = null
//         drawstate.move = null;
//     }
//     r.free();
// } else {
//     alert("Error: invalid board??")
// }

// board.free();