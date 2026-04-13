// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   display.rs                                         :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 18:15:37 by lumugot           #+#    #+#             //
//   Updated: 2026/04/13 16:51:01 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use crate::board::{Board, Cell, BOARD_SIZE};

const   CELL_SIZE: f32  = 80.0;
const   MARGIN: f32     = 50.0;

const COLOR_BG: Color           = Color::new(0.12, 0.12, 0.12, 1.0);
const COLOR_GRID: Color         = Color::new(0.2,  0.2,  0.2,  1.0);
const COLOR_SNAKE_HEAD: Color   = Color::new(0.2,  0.6,  1.0,  1.0);
const COLOR_SNAKE_BODY: Color   = Color::new(0.1,  0.4,  0.8,  1.0);
const COLOR_GREEN_APPLE: Color  = Color::new(0.2,  0.9,  0.3,  1.0);
const COLOR_RED_APPLE: Color    = Color::new(0.9,  0.2,  0.2,  1.0);

pub fn window_size() -> (f32, f32)
{
    let size = BOARD_SIZE as f32 * CELL_SIZE + MARGIN* 2.0;
    (size, size + 40.0)
}

fn  cell_to_pixel(row: usize, col: usize) -> (f32, f32)
{
    let x = MARGIN + col as f32 * CELL_SIZE;
    let y = MARGIN + row as f32 * CELL_SIZE;
    (x, y)
}

pub fn draw_board(board: &Board)
{
    clear_background(COLOR_BG);
    
    for i in 0..=BOARD_SIZE
    {
        let i_f = i as f32;

        draw_line(
            MARGIN, 
            MARGIN + i_f * CELL_SIZE,
            MARGIN+ BOARD_SIZE as f32 * CELL_SIZE,
            MARGIN + i_f * CELL_SIZE,
            1.0,
            COLOR_GRID,
        );

        draw_line(
            MARGIN + i_f * CELL_SIZE,
            MARGIN,
            MARGIN + i_f * CELL_SIZE,
            MARGIN + BOARD_SIZE as f32 * CELL_SIZE,
            1.0,
            COLOR_GRID,
        );
    }

    for row in 0..=BOARD_SIZE
    {
        for col in 0..=BOARD_SIZE
        {
            let cell = board.get_cell(row, col);
            let color = match cell
            {
                Cell::SnakeHead  => Some(COLOR_SNAKE_HEAD),
                Cell::SnakeBody  => Some(COLOR_SNAKE_BODY),
                Cell::GreenApple => Some(COLOR_GREEN_APPLE),
                Cell::RedApple   => Some(COLOR_RED_APPLE),
                Cell::Empty      => None,
            };

            if let Some(c) = color
            {
                let (x, y) = cell_to_pixel(row, col);

                draw_rectangle(x + 2.0, y + 2.0, CELL_SIZE - 4.0, CELL_SIZE - 4.0, c);
            }
        }
    }
    let score_text = format!("Lenght: {}", board.snake.lenght());
    draw_text(&score_text, MARGIN, MARGIN + BOARD_SIZE as f32 * CELL_SIZE + 28.0, 24.0, WHITE);
}

pub fn draw_game_over(board: &Board)
{
    draw_board(board);

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));
    
    let text = "GAME OVER";
    let font_size= 48.0;

    let x = screen_width() / 2.0 - font_size * 2.2;
    let y = screen_height() / 2.0;
    draw_text(text, x, y, font_size, RED);

    let sub = format!("Final lenght: {} | Press R to restart", board.snake.lenght());
    draw_text(&sub, MARGIN, y + 40.0, 20.0, WHITE);
}
