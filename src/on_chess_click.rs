use std::{borrow::Borrow, collections::HashMap};

use crate::ChessBoardItem;

pub fn on_mouse_down<'a>(
    mut chess_map: std::sync::MutexGuard<'_, HashMap<String, ChessBoardItem>>,
    prev_position: String,
    curr_position: String,
) -> String {
    // Если предыдущая позиция равна текущей, то ничего не делаем
    if prev_position == curr_position {
        print!("PREV CONDITION: {}\n", curr_position);
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

    let [y, x]: [i32; 2] = curr_position
        .split("-")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
        .as_slice()
        .try_into()
        .unwrap();

    if previous_item.x == x && previous_item.y == y {
        previous_item.state = "rest".to_string();
        chess_map.insert(curr_position.clone(), previous_item);
        return curr_position;
    }

    let clone = previous_item.borrow();

    let can_move = check_if_can_move(
        &clone.variant,
        &clone.team,
        clone.moved,
        clone.x,
        clone.y,
        x,
        y,
    );

    match can_move {
        true => {
            previous_item.x = x;
            previous_item.y = y;
            previous_item.moved = true;
            previous_item.state = "rest".to_string();
            chess_map.insert(
                previous_item.y.to_string() + "-" + &previous_item.x.to_string(),
                previous_item,
            );
        }
        false => {
            previous_item.state = "rest".to_string();
            chess_map.insert(prev_position.clone(), previous_item);
            return curr_position;
        }
    }

    "empty".to_string()
}

fn check_if_can_move(
    pice_type: &String,
    team: &String,
    moved: bool,
    curr_x: i32,
    curr_y: i32,
    new_x: i32,
    new_y: i32,
) -> bool {
    match pice_type.as_str() {
        "pawn" => {
            if team.as_str() == "white" {
                if curr_x == new_x && (curr_y + 1 == new_y) {
                    return true;
                }
                if curr_x == new_x && (curr_y + 2 == new_y) && !moved {
                    return true;
                }
            } else {
                if curr_x == new_x && (curr_y - 1 == new_y) {
                    return true;
                }
                if curr_x == new_x && (curr_y - 2 == new_y) && !moved {
                    return true;
                }
            }

            false
        }
        "rook" => {
            if curr_x == new_x || curr_y == new_y {
                return true;
            }
            false
        }
        "knight" => {
            if (curr_x - 2 == new_x && curr_y - 1 == new_y)
                || (curr_x - 2 == new_x && curr_y + 1 == new_y)
                || (curr_x + 2 == new_x && curr_y - 1 == new_y)
                || (curr_x + 2 == new_x && curr_y + 1 == new_y)
                || (curr_x - 1 == new_x && curr_y - 2 == new_y)
                || (curr_x - 1 == new_x && curr_y + 2 == new_y)
                || (curr_x + 1 == new_x && curr_y - 2 == new_y)
                || (curr_x + 1 == new_x && curr_y + 2 == new_y)
            {
                return true;
            }
            false
        }
        "bishop" => {
            if (curr_x - new_x).abs() == (curr_y - new_y).abs() {
                return true;
            }
            false
        }
        "queen" => {
            if (curr_x - new_x).abs() == (curr_y - new_y).abs() {
                return true;
            }
            if curr_x == new_x || curr_y == new_y {
                return true;
            }
            false
        }
        "king" => {
            if (curr_x - new_x).abs() <= 1 && (curr_y - new_y).abs() <= 1 {
                return true;
            }
            false
        }
        _ => false,
    }
}
