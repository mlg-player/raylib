use raylib::{RaylibHandle, RaylibThread};

use crate::{update_chess_map, ChessItem};

pub fn init_chess_items(chess_items: Vec<ChessItem>, thread: &RaylibThread, rl: &mut RaylibHandle) {
    for index in 0..chess_items.len() {
        let item = &chess_items[index];

        let is_white = item.team == "white";

        let mut x = 0;
        let mut y = if is_white { 0 } else { 7 };

        if item.variant == "pawn" {
            if is_white {
                y = 1;
            } else {
                y = 6;
            }
            for field in 0..8 {
                let name = y.to_string() + &'-'.to_string() + &field.to_string();
                update_chess_map(name, item, field, y, thread, rl);
                x += 1;
            }
        } else if item.variant == "rook" {
            x = 0;

            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);

            x = 7;
            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);
        } else if item.variant == "knight" {
            x = 1;

            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);
            x = 6;
            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);
        } else if item.variant == "bishop" {
            x = 2;
            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);
            x = 5;
            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);
        } else if item.variant == "queen" {
            x = 3;
            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);
        } else if item.variant == "king" {
            x = 4;
            let name = y.to_string() + &'-'.to_string() + &x.to_string();
            update_chess_map(name, item, x, y, thread, rl);
        } else {
            print!("Invalid type!\n");
            print!("Type: {}\n", item.variant);
        }
    }
}
