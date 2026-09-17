use macroquad::prelude::*;
use std::{thread, time::Duration};

const BOARD_HEIGHT: usize = 10;
const BOARD_WIDTH: usize = 10;
const CELL_SIZE: f32 = 75.;

const AROUND_POSITIONS: [[i8; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

const INITIAL_CELLS: [[usize; 2]; 5] = [[1, 0], [2, 1], [0, 2], [1, 2], [2, 2]];

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of life".to_owned(),
        window_width: (CELL_SIZE as i32 * BOARD_WIDTH as i32),
        window_height: (CELL_SIZE as i32 * BOARD_HEIGHT as i32),
        window_resizable: false,
        ..Default::default()
    }
}

type Matrix = [[u8; BOARD_WIDTH]; BOARD_HEIGHT];
type Row = usize;
type Column = usize;

fn get_alive_neighbors(matrix: Matrix, row: Row, col: Column) -> usize {
    let mut alive: usize = 0;

    for [diff_row, diff_col] in AROUND_POSITIONS {
        let current_row: i8 = row as i8 + diff_row;
        let current_col: i8 = col as i8 + diff_col;

        if (current_row < 0) || (current_row as usize >= BOARD_HEIGHT) {
            continue;
        };

        if (current_col < 0) || (current_col as usize >= BOARD_WIDTH) {
            continue;
        };

        if matrix[current_row as usize][current_col as usize] == 1 {
            alive += 1;
        };
    }

    return alive;
}

fn draw_board(board: Matrix) {
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            if board[row][col] == 1 {
                draw_rectangle(
                    row as f32 * CELL_SIZE,
                    col as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    BLACK,
                )
            }
        }
    }
}

fn update_board(board: &mut Matrix) {
    let mut new_board = board.clone();

    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            let alive_neighbors: usize = get_alive_neighbors(*board, row, col);
            let current_is_alive = board[row][col] == 1;

            if alive_neighbors > 0 {
                let text_color = if current_is_alive { WHITE } else { BLACK };
                draw_text(
                    alive_neighbors.to_string().as_str(),
                    (row as f32 * CELL_SIZE) + (CELL_SIZE / 3.),
                    (col as f32 * CELL_SIZE) + (CELL_SIZE / 1.5),
                    CELL_SIZE * 0.5,
                    text_color,
                );
            }

            if current_is_alive {
                new_board[row][col] = (alive_neighbors == 2 || alive_neighbors == 3) as u8;
            } else {
                new_board[row][col] = (alive_neighbors == 3) as u8;
            }
        }
    }
    *board = new_board;
}

fn set_fps_limit(frame_time: f32, fps: f32) {
    let minimum_frame_time = 1. / fps;

    if frame_time < minimum_frame_time {
        let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board: Matrix = [[0; BOARD_WIDTH]; BOARD_HEIGHT];

    for [row, col] in INITIAL_CELLS {
        board[row][col] = 1;
        // board[BOARD_HEIGHT / 2 + row][BOARD_WIDTH / 2 + col] = 1;
    }

    let frame_time = get_frame_time();

    loop {
        clear_background(WHITE);
        draw_board(board);
        update_board(&mut board);
        next_frame().await;

        set_fps_limit(frame_time, 0.3);
    }
}
