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
const SQUARE_SIZE: i32 = 50;
const WINDOW_WIDTH: i32 = BOARD_SIZE * SQUARE_SIZE;
const WINDOW_HEIGHT: i32 = BOARD_SIZE * SQUARE_SIZE;

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

fn init_chess_board(draw: &mut RaylibDrawHandle) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let x = i * SQUARE_SIZE;
            let y = j * SQUARE_SIZE;
            let color = if (i + j) % 2 == 0 {
                ChessColor::WHITE
            } else {
                ChessColor::BLACK
            };
            draw.draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, color);
        }
    }
}

fn render_chess_item(item: &ChessBoardItem, draw: &mut RaylibDrawHandle) {
    let texture_id = item.variant.clone() + &"-".to_string() + &item.team.clone();

    let binding = CHESS_TEXTURES_MAP.lock().unwrap();
    let texture = &binding.get(&texture_id).unwrap().texture;

    let x = item.x * SQUARE_SIZE;
    let y = item.y * SQUARE_SIZE;

    let is_white_field = (item.x + item.y) % 2 == 0;

    if item.state == "rest" {
        let rest_color = if is_white_field {
            ChessColor::WHITE
        } else {
            ChessColor::BLACK
        };
        draw.draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, rest_color);
    } else if item.state == "selected" {
        let selected_color = ChessColor::SELECTED;

        draw.draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, selected_color.clone());
    } else if item.state == "hover" {
        let hover_color = ChessColor::HOVERED;

        draw.draw_rectangle(x, y, SQUARE_SIZE, SQUARE_SIZE, hover_color);
    }
    draw.draw_texture(texture, x, y, Color::WHITE);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, World")
        .build();

    let chess_items = load_json();

    init_chess_items(chess_items, &thread, &mut rl);

    let mut chess_initialized = false;
    let mut chess_items_rendered = false;
    let mut x;
    let mut y;

    let mut selected = "selected".to_string();
    let mut position = "".to_string();
    print!("UPDATE\n");

    while !rl.window_should_close() {
        if chess_initialized && chess_items_rendered {
            let mut chess_map = CHESS_MAP.lock().unwrap();
            if chess_map.contains_key(&position) {
                let item = chess_map.get_mut(&position).unwrap();
                item.state = "rest".to_string();
            }
            x = rl.get_mouse_x() / SQUARE_SIZE;
            y = rl.get_mouse_y() / SQUARE_SIZE;

            if chess_map.contains_key(&position) {
                rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
                let item = chess_map.get_mut(&position).unwrap();
                item.state = "rest".to_string();
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
                let new_selected = on_mouse_down(chess_map, selected.clone(), position.clone());
                print!("Selected: {}\n", new_selected);
                print!("Prev selected: {}\n", selected);
                selected = new_selected;
                position = "".to_string();
            }
        }

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        init_chess_board(&mut d);
        if !chess_initialized {
            chess_initialized = true;
        }

        CHESS_MAP.lock().unwrap().iter().for_each(|(_, value)| {
            render_chess_item(value, &mut d); // Pass a mutable reference to ChessBoardItem
        });
        if !chess_items_rendered {
            chess_items_rendered = true;
        }

        d.clear_background(Color::WHITE);
    }
}
