// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   input.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/22 14:42:46 by lumugot           #+#    #+#             //
//   Updated: 2026/04/22 15:35:50 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use crate::runtime::Runtime;
use crate::logger::{ANSI_CYAN, ANSI_YELLOW, ANSI_RESET};
use crate::board::Direction;
use crate::logger::log;

pub fn	handle_speed_keys(runtime: &mut Runtime)
{
	if is_key_pressed(KeyCode::KpSubtract)
	{	
		runtime.set_ai_speed_min();
		log(runtime, format!("{}[SPEED]{} {}", ANSI_YELLOW, ANSI_RESET, runtime.speed_label));
	}

	if is_key_pressed(KeyCode::KpAdd)
	{
		runtime.set_ai_speed_max();
		log(runtime, format!("{}[SPEED]{} {}", ANSI_YELLOW, ANSI_RESET, runtime.speed_label));
	}

	if is_key_pressed(KeyCode::Backspace)
	{
		runtime.set_player_speed();
		log(runtime, format!("{}[SPEED]{} {}", ANSI_YELLOW, ANSI_RESET, runtime.speed_label));
	}
}

pub fn	handle_direction_keys(runtime: &mut Runtime)
{
	if runtime.use_ai { return ; }

	if is_key_pressed(KeyCode::Up)
	{
		runtime.queued_dir = Direction::Up;
		if !runtime.started
		{
			runtime.started = true;
			runtime.paused = false;
		}
	}

	if is_key_pressed(KeyCode::Down)
	{
		runtime.queued_dir = Direction::Down;
		if !runtime.started
		{
			runtime.started = true;
			runtime.paused = false;
		}
	}

	if is_key_pressed(KeyCode::Left)
	{
		runtime.queued_dir = Direction::Left;
		if !runtime.started
		{
			runtime.started = true;
			runtime.paused = false;
		}
	}

	if is_key_pressed(KeyCode::Right)
	{
		runtime.queued_dir = Direction::Right;
		if !runtime.started
		{
			runtime.started = true;
			runtime.paused = false;
		}
	}
}

pub fn	handle_mode_toggle_key(runtime: &mut Runtime)
{
	if runtime.started { return ; }
	if !is_key_pressed(KeyCode::Tab) { return ; }

	runtime.use_ai = !runtime.use_ai;

	if runtime.use_ai { runtime.set_ai_speed_min(); } 
	else { runtime.set_player_speed(); }

	log(runtime, format!("{}[MODE]{} AI {}", ANSI_CYAN, ANSI_RESET, if runtime.use_ai { "ON" } else { "OFF" }));
}

pub fn	handle_enter_key(runtime: &mut Runtime)
{
	if !is_key_pressed(KeyCode::Enter) { return ; }

	if !runtime.started
	{
		runtime.started = true;
		runtime.paused = false;
		return ;
	}

	runtime.paused = !runtime.paused;
}

pub fn	handle_debug_key(runtime: &mut Runtime)
{
	if is_key_pressed(KeyCode::Space)
	{
		runtime.show_vision = !runtime.show_vision;
	}

	if is_key_pressed(KeyCode::D)
	{
		runtime.show_logs = !runtime.show_logs;
	}

	if is_key_pressed(KeyCode::H)
	{
		runtime.show_help = !runtime.show_help;
	}
}
