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
    let name = item.team.clone() + &item.variant.clone();
    let binding = CHESS_TEXTURES_MAP.lock().unwrap();
    let texture = &binding.get(&name).unwrap().texture;

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

fn init_chess_items(chess_items: Vec<ChessItem>, thread: &RaylibThread, rl: &mut RaylibHandle) {
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
                let name = y.to_string() + &'-'.to_string() + &x.to_string();
                CHESS_MAP.lock().unwrap().insert(
                    name,
                    ChessBoardItem {
                        name: item.name.clone(),
                        team: item.team.clone(),
                        variant: item.variant.clone(),
                        src: item.src.clone(),
                        x: field,
                        y,
                        state: "rest".to_string(),
                    },
                );
                x += 1;
            }
        } else if item.variant == "rook" {
            x = 0;

            let name = y.to_string() + &'-'.to_string() + &x.to_string();
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

            x = 7;
        } else if item.variant == "knight" {
            x = 1;

            let name = y.to_string() + &'-'.to_string() + &x.to_string();
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
            x = 6;
        } else if item.variant == "bishop" {
            x = 2;
            let name = y.to_string() + &'-'.to_string() + &x.to_string();
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
            x = 5;
        } else if item.variant == "queen" {
            x = 3;
        } else if item.variant == "king" {
            x = 4;
        } else {
            print!("Invalid type!\n");
            print!("Type: {}\n", item.variant);
        }

        let name = y.to_string() + &'-'.to_string() + &x.to_string();
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
}

fn add_to_chess_map(chess_items: ChessBoardItem, thread: &RaylibThread, rl: &mut RaylibHandle) {
    let name = chess_items.team.clone() + &chess_items.variant.clone();
    let textures_loaded = CHESS_TEXTURES_MAP.lock().unwrap().contains_key(&name);

    if !textures_loaded {
        let texture = rl.load_texture(&thread, &chess_items.src).unwrap();

        CHESS_TEXTURES_MAP.lock().unwrap().insert(
            name.clone(),
            ChessRenderData {
                team: chess_items.team.clone(),
                variant: chess_items.variant.clone(),
                texture,
            },
        );
    }

    CHESS_MAP.lock().unwrap().insert(name, chess_items);
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

    let mut selected = "".to_string();
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
                        selected = position.clone();
                    }
                }
                None => {}
            }

            if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                update_chess_square(chess_map, &mut selected, &position, "selected".to_string());
            }
        }

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

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

fn update_chess_square(
    mut chess_map: std::sync::MutexGuard<'_, HashMap<String, ChessBoardItem>>,
    previous: &mut String,
    position: &String,
    state: String,
) {
    if chess_map.contains_key(&*previous) {
        let item = chess_map.get_mut(&*previous).unwrap();
        item.state = "rest".to_string();
    }

    let item = chess_map.get_mut(position);
    match item {
        Some(item) => {
            if item.state == "rest" {
                item.state = state;
                *previous = position.clone();
            }
        }
        None => {}
    }
}
