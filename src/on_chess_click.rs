use std::collections::HashMap;

use crate::ChessBoardItem;

pub fn on_mouse_down<'a>(
    mut chess_map: std::sync::MutexGuard<'_, HashMap<String, ChessBoardItem>>,
    prev_position: String,
    curr_position: String,
) -> String {
    // Если предыдущая позиция равна текущей, то ничего не делаем
    if prev_position == curr_position {
        return prev_position;
    }
    // Если предыдущая позиция не равна текущей, то обновляем позицию фигуры

    // Удаляем предыдущую позицию
    let previous_item = chess_map.remove(&prev_position);
    if previous_item.is_none() {
        // Если предыдущая позиция не существует, то делаем curr_position как Selected
        match chess_map.get_mut(&curr_position) {
            Some(item) => {
                item.state = "selected".to_string();
                return curr_position;
            }
            None => {
                return "empty".to_string();
            }
        };
    }
    // Если предыдущая позиция существует, то обновляем ее координаты
    // и перемещяем ее на новую позицию
    // и обновляем позицию на rest
    // если в новой позиции уже есть фигура, то удаляем её
    let mut previous_item = previous_item.unwrap();
    let current_hovered_item = chess_map.get_mut(&curr_position);

    match current_hovered_item {
        Some(_) => {
            // print!("PREV CONDITION: {}\n", curr_position);
        }
        None => {
            // print!("LAS CONDITION: {}\n", curr_position);
        }
    }
    previous_item.x = curr_position.split("-").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    previous_item.y = curr_position.split("-").collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    previous_item.state = "rest".to_string();
    chess_map.insert(curr_position.clone(), previous_item);

    return "empty".to_string();
}
