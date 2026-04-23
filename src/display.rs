// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   display.rs                                         :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 18:15:37 by lumugot           #+#    #+#             //
//   Updated: 2026/04/23 12:44:11 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use crate::board::{Board, Cell};

const	CELL_SIZE: f32			= 80.0;
const	MARGIN: f32				= 50.0;
const	MAX_BOARD_PX: f32		= 800.0;

const PANEL_WIDTH: f32			= 420.0;
const PANEL_PADDING: f32		= 20.0;

const COLOR_BG: Color           = Color::new(0.12, 0.12, 0.12, 1.0);
const COLOR_GRID: Color         = Color::new(0.2,  0.2,  0.2,  1.0);

const COLOR_CELL_LIGHT: Color	= Color::new(0.17, 0.5, 0.17, 1.0);
const COLOR_CELL_DARK: Color	= Color::new(0.13, 0.38, 0.13, 1.0);

const COLOR_SNAKE_HEAD: Color   = Color::new(0.2,  0.6,  1.0,  1.0);
const COLOR_SNAKE_BODY: Color   = Color::new(0.1,  0.4,  0.8,  1.0);
const COLOR_GREEN_APPLE: Color  = Color::new(0.2,  0.9,  0.3,  1.0);
const COLOR_RED_APPLE: Color    = Color::new(0.9,  0.2,  0.2,  1.0);

pub fn cell_size(board_size: usize) -> f32
{
	MAX_BOARD_PX / board_size as f32
}

pub fn window_size(board_size: usize) -> (f32, f32)
{
	let cs = cell_size(board_size);
	let board_px = board_size as f32 * cs;
	let board_area = board_px + MARGIN * 2.0;
	let w = board_area + PANEL_WIDTH;
	let h = board_area + 40.0;
	(w, h)
}

pub fn panel_left_x(board_size: usize) -> f32
{
    let board_px = board_size as f32 * CELL_SIZE;
    MARGIN + board_px + MARGIN
}

pub fn panel_padding() -> f32
{
    PANEL_PADDING
}

fn  cell_to_pixel(row: usize, col: usize) -> (f32, f32)
{
    let x = MARGIN + col as f32 * CELL_SIZE;
    let y = MARGIN + row as f32 * CELL_SIZE;
    (x, y)
}

fn	draw_apple(x: f32, y: f32, color: Color)
{
	let cx = x + CELL_SIZE / 2.0;
	let	cy = y + CELL_SIZE / 2.0 + 4.0;
	let	r = CELL_SIZE / 2.0 - 8.0;

	draw_circle(cx, cy, r, color);

	draw_circle(cx - r * 0.3, cy - r * 0.33, r * 0.2, Color::new(1.0, 1.0, 1.0, 0.4));

	draw_line(cx, y + 8.0, cx + 5.0, y + 3.0, 2.0, Color::new(0.4, 0.25, 0.1, 1.0));

	draw_circle(cx + 6.0, y + 4.0, 4.0, Color::new(0.2, 0.7, 0.2, 1.0));
}

pub fn draw_board(board: &Board)
{
    clear_background(COLOR_BG);
    
    for i in 0..=board.size
    {
        let i_f = i as f32;

        draw_line(
            MARGIN, 
            MARGIN + i_f * CELL_SIZE,
            MARGIN+ board.size as f32 * CELL_SIZE,
            MARGIN + i_f * CELL_SIZE,
            1.0,
            COLOR_GRID,
        );

        draw_line(
            MARGIN + i_f * CELL_SIZE,
            MARGIN,
            MARGIN + i_f * CELL_SIZE,
            MARGIN + board.size as f32 * CELL_SIZE,
            1.0,
            COLOR_GRID,
        );
    }

    for row in 0..board.size
    {
        for col in 0..board.size
        {
			let bg = if (row + col) % 2 == 0 { COLOR_CELL_LIGHT } else { COLOR_CELL_DARK };
			let (x, y) = cell_to_pixel(row, col);
			draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, bg);

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
				match cell
				{
					Cell::GreenApple | Cell::RedApple => { draw_apple(x, y, c); }
					
					_ =>
					{
						draw_rectangle(x + 2.0, y + 2.0, CELL_SIZE - 4.0, CELL_SIZE - 4.0, c);
					}
				}
			}
		}
    }
    let score_text = format!("Lenght: {}", board.snake.lenght());
    draw_text(&score_text, MARGIN, MARGIN + board.size as f32 * CELL_SIZE + 28.0, 24.0, WHITE);
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
