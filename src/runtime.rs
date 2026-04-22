// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   runtime.rs                                         :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/22 14:39:05 by lumugot           #+#    #+#             //
//   Updated: 2026/04/22 15:12:20 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::get_time;
use crate::board::{Board, Direction};
use crate::cli::Mode;

pub const SPEED_PLAYER: f64	= 0.1;
pub const SPEED_AI_MAX: f64	= 0.0001;
pub const SPEED_AI_MIN: f64	= 0.05;

pub struct Runtime {
	pub board:	Board,
	pub board_size: usize,
	pub queued_dir:	Direction,
	pub started: bool,
	pub paused: bool,
	pub use_ai:	bool,
	pub speed: f64,
	pub speed_label: &'static str,
	pub last_step: f64,
	pub steps_since_food: u32,
	pub show_vision: bool,
	pub show_logs: bool,
	pub show_help: bool,
	pub should_quit: bool,
}

impl Runtime {
	pub fn new(mode: Mode, board_size: usize) -> Self
	{
		let board = Board::new(board_size);
		let queued_dir = board.snake.direction.clone();
		let use_ai = matches!(mode, Mode::Predict | Mode::PredictTrain);

		let mut rt = Self
		{
			board,
			board_size,
			queued_dir,
			started: false,
			paused: false,
			use_ai,
			speed: SPEED_PLAYER,
			speed_label: "SPEED FOR PLAYER",
			last_step: get_time(),
			steps_since_food: 0,
			show_vision: false,
			show_logs: false,
			show_help: true,
			should_quit: false,
		};

		if use_ai { rt.set_ai_speed_min(); }
	
		else { rt.set_player_speed(); }
	
		rt
	}

	pub fn reset_board(&mut self, started: bool)
	{
		self.board= Board::new(self.board_size);
		self.queued_dir = self.board.snake.direction.clone();
		self.last_step= get_time();
		self.steps_since_food = 0;
		self.started = started;
		self.paused = false;
		self.show_help = true;
	}

	pub fn set_ai_speed_max(&mut self)
	{
		self.speed = SPEED_AI_MAX;
		self.speed_label = "SPEED IA MAX";
	}
	
	pub fn set_ai_speed_min(&mut self)
	{
		self.speed = SPEED_AI_MIN;
		self.speed_label = "SPEED IA MIN";
	}

	pub fn set_player_speed(&mut self)
	{
		self.speed = SPEED_PLAYER;
		self.speed_label = "SPEED FOR PLAYER";
	}
}
