use raylib::{RaylibHandle, RaylibThread};

use crate::{add_to_chess_map, ChessBoardItem, ChessItem, CHESS_MAP};

pub fn update_chess_map(
    name: String,
    item: &ChessItem,
    x: i32,
    y: i32,
    thread: &RaylibThread,
    rl: &mut RaylibHandle,
) {
    CHESS_MAP.lock().unwrap().insert(
        name,
        ChessBoardItem {
            name: item.name.clone(),
            team: item.team.clone(),
            variant: item.variant.clone(),
            src: item.src.clone(),
            x,
            y,
            state: "rest".to_string(),
        },
    );

    add_to_chess_map(
        ChessBoardItem {
            name: item.name.clone(),
            team: item.team.clone(),
            variant: item.variant.clone(),
            src: item.src.clone(),
            x,
            y,
            state: "rest".to_string(),
        },
        thread,
        rl,
    );
}
