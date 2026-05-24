// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   hud.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/22 14:46:43 by lumugot           #+#    #+#             //
//   Updated: 2026/05/24 09:41:07 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use crate::board::{Board, Direction};
use crate::runtime::Runtime;
use crate::game_stats::Stats;
use crate::logger::{ANSI_CYAN, ANSI_RESET};
use crate::state::compute_vision;
use crate::display::{draw_board, panel_left_x, panel_padding};

pub fn draw_frame(runtime: &Runtime, stats: &Stats)
{
	draw_board(&runtime.board);

	if runtime.show_vision
	{
		draw_vision_overlay(&runtime.board, runtime);
	}

	draw_hud(runtime, stats);
	draw_mode_status(runtime);
	draw_help_menu(runtime);
}

pub fn draw_hud(runtime: &Runtime, stats: &Stats)
{
	let x = panel_left_x(runtime.board_size) + panel_padding();
	let mut y = 30.0;
	let fs = 24.0;
	let line = 28.0;

	draw_text(&format!("Episode          : {}", stats.episode_count), x, y, fs, YELLOW); y += line;
	draw_text(&format!("Personnal record : {}", stats.best_lenght), x, y, fs, YELLOW); y += line;
	draw_text(&format!("Average length   : {:.2}", stats.average()), x, y, fs, YELLOW); y += line;
	draw_text(&format!("Speed: {}", runtime.speed_label), x, y, fs, YELLOW);
}

pub fn print_terminal_hud(board: &Board)
{
	let vision = compute_vision(board);

	let dir_str = match board.snake.direction
	{
		Direction::Up    => "Up",
		Direction::Down  => "Down",
		Direction::Left  => "Left",
		Direction::Right => "Right",
	};

	print!("\x1b[6A");
	println!("{}[HUD]{} Head : ({}, {})", ANSI_CYAN, ANSI_RESET, vision.head.0, vision.head.1);
	println!("{}[HUD]{} Up   : {}",       ANSI_CYAN, ANSI_RESET, vision.up);
	println!("{}[HUD]{} Down : {}",       ANSI_CYAN, ANSI_RESET, vision.down);
	println!("{}[HUD]{} Left : {}",       ANSI_CYAN, ANSI_RESET, vision.left);
	println!("{}[HUD]{} Right: {}",       ANSI_CYAN, ANSI_RESET, vision.right);
	println!("{}[HUD]{} Dir  : {}",       ANSI_CYAN, ANSI_RESET, dir_str);
}

pub fn	init_terminal_hud()
{
	for _ in 0..6 { println!(); }
}

pub fn draw_mode_status(runtime: &Runtime)
{
	let fs = 26.0;
	let pad = panel_padding();

	let text = if runtime.use_ai { "IA: ON" } else { "IA: OFF" };
	let dims= measure_text(text, None, fs as u16, 1.0);

	let x = screen_width() - pad - dims.width - 1600.0;
	let y = screen_height() - pad;

	let color = if runtime.use_ai { GREEN } else { RED };
	draw_text(text, x, y, fs, color);
}

pub fn draw_help_menu(runtime: &Runtime)
{
	let fs = 22.0;
	let line = 26.0;
	let pad = panel_padding();

	let x = panel_left_x(runtime.board_size) + pad;
	let w = screen_width() - x - pad;

	let h = 7.0 * line + 8.0;
	let y = screen_height() - pad - h;

	draw_rectangle(x - 10.0, y - 22.0, w + 20.0, h + 70.0, Color::new(0.0, 0.0, 0.0, 0.75));

	let mut ty = y;
	draw_text("TAB    : toggle AI (before start)", x, ty, fs, WHITE);	ty += line;
	draw_text("ENTER  : start / pause", x, ty, fs, WHITE);				ty += line;
	draw_text("ARROWS : start + move (Player)", x, ty, fs, WHITE);		ty += line;
	draw_text("SPACE  : print vision", x, ty, fs, WHITE);				ty += line;
	draw_text("D      : logs", x, ty, fs, WHITE);						ty += line;
	draw_text("ESC    : quit", x, ty, fs, WHITE);
}

pub fn	draw_vision_overlay(board: &Board, runtime: &Runtime)
{
	let v = compute_vision(board);

	let x = panel_left_x(runtime.board_size) + panel_padding();
	let fs = 30.0;
	let line = 34.0;
	let lines = 5.0;

	let block_h = (lines - 1.0) * line;
	let mut y = screen_height() * 0.5 - block_h * 0.5;

	draw_text(&format!("Head : ({}, {})", v.head.0, v.head.1), x, y, fs, WHITE);	y += line;
	draw_text(&format!("Up   : {}", v.up), x, y, fs, WHITE);						y += line;
	draw_text(&format!("Down : {}", v.down), x, y, fs, WHITE);						y += line;
	draw_text(&format!("Left : {}", v.left), x, y, fs, WHITE);						y += line;
	draw_text(&format!("Right: {}", v.right), x, y, fs, WHITE);						y += line;

	let snake_dir = match runtime.board.snake.direction
	{
		Direction::Up		=> "Up",
		Direction::Down		=> "Down",
		Direction::Left		=> "Left",
		Direction::Right	=> "Right",
	};

	draw_text(&format!("Direction: {}", snake_dir), x, y, fs, WHITE);
}

pub fn	draw_final_stats(stats: &Stats, sessions: u32)
{
	clear_background(BLACK);

	let fs_title = 48.0;
	let fs_text  = 28.0;
	let line     = 38.0;
	let cx       = screen_width() / 2.0;
	let mut y    = screen_height() / 2.0 - line * 3.0;

	let tittle = "SESSION COMPLETE";
	let dim = measure_text(tittle, None, fs_title as u16, 1.0);
	draw_text(tittle, cx - dim.width / 2.0, y, fs_title, GREEN);

	y += line * 2.0;

    let lines = [
		format!("Sessions  : {}", sessions),
		format!("Best length: {}", stats.best_lenght),
		format!("Average length  : {:.2}", stats.average()),
		format!("Total episodes  : {}", stats.episode_count),
	];

	for text in &lines
	{
		let dim = measure_text(text, None, fs_text as u16, 1.0);
		draw_text(text, cx - dim.width / 2.0, y, fs_text, WHITE);
		y += line;
	}

	let quit = "Press ESC to quit";
	let dim = measure_text(quit, None, fs_text as u16, 1.0);
	draw_text(quit, cx - dim.width / 2.0, y + line, fs_text, GRAY);
}
