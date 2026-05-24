// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   main.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:09:13 by lumugot           #+#    #+#             //
//   Updated: 2026/05/24 09:35:20 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use std::sync::OnceLock;

mod	board;
mod snake;
mod display;
mod state;
mod action;
mod rewards;
mod env;
mod agent;
mod train;
mod cli;
mod logger;
mod game_stats;
mod runtime;
mod input;
mod hud;
mod agent_builder;
mod game_loop;

use board::{DEFAULT_BOARD_SIZE, validate_board_size};
use display::window_size;
use cli::Cli;
use agent_builder::build_agent;
use game_loop::run_visual_loop;

static BOARD_SIZE_GLOBAL: OnceLock<usize> = OnceLock::new();

fn	parse_board_size() -> usize
{
	let args: Vec<String> = std::env::args().collect();

	for i in 0..args.len()
	{
		if let Some(val) = args.get(i + 1)
		{
			if let Ok(n) = val.parse::<usize>() { return n; }
		}
	}
	DEFAULT_BOARD_SIZE
}

fn	window_conf() -> Conf
{
	let size = *BOARD_SIZE_GLOBAL.get_or_init(parse_board_size);

	let (w, h) = window_size(size);
	
	Conf
	{
		window_title: "Learn2Slither".to_string(),
		window_width: w as i32,
		window_height: h as i32,
		window_resizable: false,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main()
{
	let args = Cli::parse_args();
	let board_size = match validate_board_size(args.board_size)
	{
		Some(s) => s,
		None	=> return,
	};
	
	let agent = match build_agent(&args)
	{
		Some(agent) => agent,
		None		=> return,
	};

	if !args.visual { return ; }

	run_visual_loop(agent, &args, board_size).await;
}
