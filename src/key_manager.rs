// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   key_manager.rs                                     :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/13 16:09:27 by lumugot           #+#    #+#             //
//   Updated: 2026/04/16 00:55:54 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use crate::board::Direction;

pub struct KeySignals {
	pub quit:			bool,
	pub reset:			bool,
	pub debug_state:	bool,
	pub step_tick:		bool,
}

pub fn handle_keys(speed: &mut f64, speed_msg: &mut String,
	queued_dir: &mut Direction, started: &mut bool, use_ai: &mut bool) -> KeySignals
{
	let mut signal = KeySignals {
		quit:			false,
		reset:			false,
		debug_state:	false,
		step_tick:		false,
	};

	if is_key_pressed(KeyCode::Escape) { signal.quit= true; }

	if is_key_pressed(KeyCode::Space) { signal.debug_state = true; }
	
	if is_key_pressed(KeyCode::R) {signal.reset = true; }
	
	if is_key_pressed(KeyCode::N) {signal.step_tick = true; }

	if is_key_pressed(KeyCode::KpSubtract) 
	{
		*speed = 0.05;
		*speed_msg= String::from("MIN SPEED IA");
	}

	if is_key_pressed(KeyCode::KpAdd) 
	{
		*speed = 0.0001;
		*speed_msg= String::from("MAX SPEED IA");
	}

	if is_key_pressed(KeyCode::Backspace) 
	{
		*speed = 0.1;
		*speed_msg= String::from("SPEED FOR PLAYER");
	}

	if is_key_pressed(KeyCode::Up) 
	{
		*queued_dir = Direction::Up;
		*started = true;
	}

	if is_key_pressed(KeyCode::Down) 
	{
		*queued_dir = Direction::Down;
		*started = true;
	}

	if is_key_pressed(KeyCode::Left) 
	{
		*queued_dir = Direction::Left;
		*started = true;
	}

	if is_key_pressed(KeyCode::Right) 
	{
		*queued_dir = Direction::Right;
		*started = true;
	}

	if is_key_pressed(KeyCode::Enter)
	{
		*use_ai = !*use_ai;
		*started= true;
		println!("AI mode: {}", if *use_ai {" ON "} else {" OFF "});
	}

	signal
}
