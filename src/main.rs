pub mod add_to_chess_map;
pub mod init_chess_items;
pub mod on_chess_click;
pub mod update_chess_map;

use crate::add_to_chess_map::add_to_chess_map;
use crate::init_chess_items::init_chess_items;
use crate::on_chess_click::on_mouse_down;
use crate::update_chess_map::update_chess_map;

use lazy_static::lazy_static;
use raylib::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

lazy_static! {
    static ref CHESS_MAP: Mutex<HashMap<String, ChessBoardItem>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    static ref CHESS_TEXTURES_MAP: Mutex<HashMap<String, ChessRenderData>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

const BOARD_SIZE: i32 = 8;
struct RectangleSize {
    width: i32,
    height: i32,
}

pub struct ChessColor;

impl ChessColor {
    pub const WHITE: Color = Color::new(255, 206, 158, 255);
    pub const BLACK: Color = Color::new(209, 139, 71, 255);

    // Define colors for selected squares
    pub const SELECTED: Color = Color::new(180, 180, 0, 255); // Darker yellow color for selected white square
    pub const HOVERED: Color = Color::new(180, 180, 180, 255); // Light grey color for hovered white square
}

#[derive(Deserialize, Debug)]
pub struct ChessItem {
    pub name: String,
    pub team: String,
    pub variant: String,
    pub src: String,
}

pub struct ChessBoardItem {
    pub name: String,
    pub team: String,

    pub variant: String,
    pub src: String,

    pub x: i32,
    pub y: i32,

    pub state: String,
    pub moved: bool,
}

pub struct ChessRenderData {
    pub team: String,
    pub variant: String,
    pub texture: Texture2D,
}

fn load_json() -> Vec<ChessItem> {
    let file = fs::read_to_string("json/chess.json").unwrap();
    let data: Vec<ChessItem> = serde_json::from_str(&file).unwrap();

    return data;
}

fn init_chess_board(draw: &mut RaylibDrawHandle, pice_size: RectangleSize, window_offset: RectangleSize) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let x = (i * pice_size.width) + window_offset.width;
            let y = (j * pice_size.height) + window_offset.height;
            let color = if (i + j) % 2 == 0 {
                ChessColor::WHITE
            } else {
                ChessColor::BLACK
            };
            draw.draw_rectangle(x, y, pice_size.width, pice_size.height, color);
        }
    }
}

fn render_chess_item(item: &ChessBoardItem, draw: &mut RaylibDrawHandle, pice_size: RectangleSize, window_offset: RectangleSize) {
    let texture_id = item.variant.clone() + &"-".to_string() + &item.team.clone();

    let binding = CHESS_TEXTURES_MAP.lock().unwrap();
    let texture = &binding.get(&texture_id).unwrap().texture;

    let x = item.x * pice_size.width + window_offset.width;
    let y = item.y * pice_size.height + window_offset.height;

    let is_white_field = (item.x + item.y) % 2 == 0;

    if item.state == "rest" {
        let rest_color = if is_white_field {
            ChessColor::WHITE
        } else {
            ChessColor::BLACK
        };
        draw.draw_rectangle(x, y, pice_size.width, pice_size.height, rest_color);
    } else if item.state == "selected" {
        draw.draw_rectangle(
            x,
            y,
            pice_size.width,
            pice_size.height,
            ChessColor::SELECTED,
        );
    } else if item.state == "hover" {
        let hover_color = ChessColor::HOVERED;

        draw.draw_rectangle(x, y, pice_size.width, pice_size.height, hover_color);
    }
    draw.draw_texture_pro(texture, Rectangle {
        x: 0.0,
        y: 0.0,
        width: texture.width as f32,
        height: texture.height as f32,
    }, Rectangle {
        x: x as f32,
        y: y as f32,
        width: pice_size.width as f32,
        height: pice_size.height as f32,
    }, Vector2 { x: 0.0, y: 0.0 }, 0.0, Color::WHITE);
}

fn main() {
    let mut window_size: RectangleSize = RectangleSize {
        width: BOARD_SIZE * 50,
        height: BOARD_SIZE * 50,
    };

    let (mut rl, thread) = raylib::init()
        .size(window_size.width, window_size.height)
        .resizable()
        .title("Hello, World")
        .build();

    let chess_items = load_json();

    init_chess_items(chess_items, &thread, &mut rl);

    let mut pice_size;

    let mut chess_initialized = false;
    let mut chess_items_rendered = false;
    let mut x: i32;
    let mut y: i32;

    let mut offset_left: i32;
    let mut offset_top: i32;

    let mut selected = "selected_item".to_string();
    let mut position = "current_pos".to_string();

    while !rl.window_should_close() {
        let win_width = rl.get_screen_width();
        let win_height = rl.get_screen_height();

        let _size = i32::min(win_width, win_height);
        let max_size = i32::max(win_width, win_height);
        window_size = RectangleSize {
            width: _size,
            height: _size,
        };

        let new_size = RectangleSize {
            width: (window_size.width as f32 / BOARD_SIZE as f32).round() as i32,
            height: (window_size.height as f32 / BOARD_SIZE as f32).round() as i32,
        };

        if max_size != win_height {
            offset_left = (max_size - (new_size.width * BOARD_SIZE)) / 2;
        } else {
            offset_left = 0;
        }

        if max_size != win_width {
            offset_top = (max_size - (new_size.height * BOARD_SIZE)) / 2;
        } else {
            offset_top = 0;
        }

        pice_size = new_size;

        if chess_initialized && chess_items_rendered {
            let mut chess_map = CHESS_MAP.lock().unwrap();
            if chess_map.contains_key(&position) {
                let item = chess_map.get_mut(&position).unwrap();

                if item.state == "hover" {
                    item.state = "rest".to_string();
                }
            }
            x = (rl.get_mouse_x() - offset_left) / pice_size.width;
            y = (rl.get_mouse_y() - offset_top) / pice_size.height;

            if chess_map.contains_key(&position) {
                rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
                let item: &mut ChessBoardItem = chess_map.get_mut(&position).unwrap();
                if item.state == "hover" {
                    item.state = "rest".to_string();
                }
            } else {
                rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_ARROW);
            }

            position = y.to_string() + &'-'.to_string() + &x.to_string();

            let item = chess_map.get_mut(&position);

            match item {
                Some(item) => {
                    if item.state == "rest" {
                        item.state = "hover".to_string();
                    }
                }
                None => {}
            }

            if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                selected = on_mouse_down(chess_map, selected.clone(), position.clone());
                position = "empty".to_string();
            }
        }

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        init_chess_board(
            &mut d,
            RectangleSize {
                width: pice_size.width,
                height: pice_size.height,
            },
            RectangleSize {
                width: offset_left,
                height: offset_top,
            },
        );

        if !chess_initialized {
            chess_initialized = true;
        }

        CHESS_MAP.lock().unwrap().iter().for_each(|(_, value)| {
            render_chess_item(
                value,
                &mut d,
                RectangleSize {
                    width: pice_size.width,
                    height: pice_size.height,
                },
                RectangleSize {
                    width: offset_left,
                    height: offset_top,
                },
            );
        });

        if !chess_items_rendered {
            chess_items_rendered = true;
        }

        d.clear_background(Color::WHITE);
    }
}
