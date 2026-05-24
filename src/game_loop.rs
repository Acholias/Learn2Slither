// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   game_loop.rs                                       :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/22 14:50:38 by lumugot           #+#    #+#             //
//   Updated: 2026/05/24 09:35:24 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use ::rand::thread_rng;
use crate::agent::Agent;
use crate::board::StepResult;
use crate::cli::Cli;
use crate::runtime::Runtime;
use crate::game_stats::Stats;
use crate::input::*;
use crate::cli::Mode;
use crate::state::compute_state;
use crate::hud::{draw_final_stats, draw_frame, init_terminal_hud, print_terminal_hud};
use crate::display::draw_game_over;
use crate::logger::{ANSI_GREEN, ANSI_RED, ANSI_RESET, ANSI_CYAN, log};

const NO_FOOD_MAX: u32	= 100;

pub async fn run_visual_loop(mut agent: Agent, args: &Cli, board_size: usize)
{
	let mut runtime = Runtime::new(args.mode, board_size);
	let mut stats = Stats::default();
	let dontmlearn_now = matches!(args.mode, Mode::Predict) || args.dontlearn;
	let mut rng = thread_rng();

	if args.hud { init_terminal_hud(); }

	loop
	{
		if runtime.should_quit == true
		{
			draw_final_stats(&stats, args.sessions);
			next_frame().await;
			if is_key_pressed(KeyCode::Escape) { break ; }

			continue ;
		}

		if is_key_pressed(KeyCode::Escape) { runtime.should_quit = true; }
	
		handle_speed_keys(&mut runtime);
		handle_mode_toggle_key(&mut runtime);
		handle_direction_keys(&mut runtime);
		handle_enter_key(&mut runtime);
		handle_debug_key(&mut runtime);

		if handle_dead_state(&mut runtime, &mut stats, args.sessions as u64)
		{
			next_frame().await;
			continue ;
		}

		if runtime.started && !runtime.paused && can_tick(args, &runtime)
		{
			apply_tick(&mut runtime, &mut agent, &mut stats, dontmlearn_now, args, &mut rng);
		}

		draw_frame(&runtime, &stats);
		next_frame().await;
	}
}

pub fn handle_dead_state(runtime: &mut Runtime, stats: &mut Stats, max_sessions: u64) -> bool
{
	if runtime.board.snake.alive { return false; }

	if runtime.use_ai
	{
		let final_length = runtime.board.snake.lenght();
		stats.close_ai_episode(final_length);
	
		runtime.print_log(format!("{}[EPISODE]{} #{} finished. Final lenght = {}",
            ANSI_CYAN, ANSI_RESET, stats.episode_count, final_length));

		if stats.episode_count >= max_sessions as u64
		{	
			runtime.should_quit = true;

			runtime.print_log(format!("{}[DONE]{} {} sessions completed. Best: {} Avg: {:.2}",
                ANSI_GREEN, ANSI_RESET,
                max_sessions,
                stats.best_lenght,
                stats.average()));
			
			return true;
		}
		runtime.reset_board(true);
		return true;
	}

	draw_game_over(&runtime.board);

	if is_key_pressed(KeyCode::R) { runtime.reset_board(false); }

	true
}

pub fn can_tick(args: &Cli, runtime: &Runtime) -> bool
{
	if args.step && runtime.use_ai { is_key_pressed(KeyCode::N) }

	else { get_time() - runtime.last_step >= runtime.speed }
}

pub fn apply_tick(runtime: &mut Runtime, agent: &mut Agent, stats: &mut Stats, 
	dontlearn_now: bool, args: &Cli, rng: &mut impl ::rand::Rng)
{
	if !(args.step && runtime.use_ai) { runtime.last_step = get_time(); }
	let result = if runtime.use_ai { tick_ai(runtime, agent, dontlearn_now, rng) }
	else { tick_player(runtime) };

	update_after_step(runtime, stats, result);	

	if args.hud
	{
		print_terminal_hud(&runtime.board);
	}
}

pub fn tick_ai(runtime: &mut Runtime, agent: &mut Agent, dontlearn_now: bool, rng: &mut impl ::rand::Rng) -> StepResult
{
	let state = compute_state(&runtime.board);
	let action= agent.select_action(&state, dontlearn_now, rng);
	
	runtime.board.step(action.to_direction())
}

pub fn tick_player(runtime: &mut Runtime) -> StepResult
{
	runtime.board.step(runtime.queued_dir.clone())
}

pub fn update_after_step(runtime: &mut Runtime, stats: &mut Stats, result: StepResult)
{
	match result
	{
		StepResult::GameOver =>
		{
			log(runtime, format!("{}[GAME OVER]{} Final length: {}", ANSI_RED, ANSI_RESET, runtime.board.snake.lenght()));
		}

		StepResult::AteGreen | StepResult::AteRed =>
		{
			runtime.steps_since_food = 0;
		}

		StepResult::Moved => {}
	}

	if runtime.use_ai
	{
		runtime.steps_since_food += 1;
		if runtime.steps_since_food >= NO_FOOD_MAX
		{
			runtime.board.snake.alive = false;
		}
	}
	let prev_best_length = stats.best_lenght;
	stats.update_best(runtime.board.snake.lenght());

	if stats.best_lenght > prev_best_length
	{
        runtime.print_log(format!("{}[RECORD]{} New length record: {}",
			ANSI_GREEN, ANSI_RESET, stats.best_lenght));
	}
}
