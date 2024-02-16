use raylib::prelude::*;

use crate::{ChessBoardItem, ChessRenderData, CHESS_MAP, CHESS_TEXTURES_MAP};

pub fn add_to_chess_map(chess_items: ChessBoardItem, thread: &RaylibThread, rl: &mut RaylibHandle) {
    let name =
        chess_items.y.clone().to_string() + &"-".to_string() + &chess_items.x.clone().to_string();

    let texture_id = chess_items.variant.clone() + &"-".to_string() + &chess_items.team.clone();

    let textures_loaded = CHESS_TEXTURES_MAP.lock().unwrap().contains_key(&texture_id);
    let included = CHESS_MAP.lock().unwrap().contains_key(&name);

    if !textures_loaded {
        let texture = rl.load_texture(&thread, &chess_items.src).unwrap();

        CHESS_TEXTURES_MAP.lock().unwrap().insert(
            texture_id,
            ChessRenderData {
                team: chess_items.team.clone(),
                variant: chess_items.variant.clone(),
                texture,
            },
        );
    }

    if included {
        return;
    }
    CHESS_MAP.lock().unwrap().insert(name, chess_items);
}
