// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   main.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:09:13 by lumugot           #+#    #+#             //
//   Updated: 2026/04/20 22:45:43 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use ::rand::thread_rng;
use std::path::Path;

mod	board;
mod snake;
mod display;
mod state;
mod action;
mod rewards;
mod env;
mod agent;
mod train;
mod key_manager;
mod cli;

use board::{Board, Direction, StepResult};
use display::{draw_board, draw_game_over, window_size};
use crate::agent::Agent;
use crate::state::{compute_state, print_state};
use crate::train::{train_basic, train_from_agent};
use cli::{Cli, Mode};
use display::{draw_board, draw_game_over, window_size};

const SPEED_PLAYER: f64	= 0.1;
const SPEED_AI_MAX: f64	= 0.0001;
const SPEED_AI_MIN: f64	= 0.05;
const NO_FOOD_MAX: u32	= 100;

const ANSI_RESET: &str	= "\x1b[0m";
const ANSI_GREEN: &str	= "\x1b[32m";
const ANSI_CYAN: &str	= "\x1b[36m";
const ANSI_YELLOW: &str	= "\x1b[33m";
const ANSI_RED: &str	= "\x1b[31m";


// State board/snake and player
#[derive(Default)]
struct Stats {
	episode_count: u64,
	best_lenght: usize,
	total_length: u64,
}

impl Stats {
	fn average(&self) -> f32
	{
		if self.episode_count == 0 { 0.0 }
		else { self.total_length as f32 / self.episode_count as f32 }
	}

	fn update_best(&mut self, current_length: usize)
	{
		if current_length > self.best_lenght
		{
			self.best_lenght= current_length;
			println!("{}[RECORD]{} New length record: {}", ANSI_GREEN, ANSI_RESET, self.best_lenght);
		}
	}

	fn close_ai_episode(&mut self, final_length: usize)
	{
		self.episode_count += 1;
		self.total_length += final_length as u64;
		println!("{}[EPISODE]{} #{} finished. Final lenght = {}", ANSI_CYAN, ANSI_RESET, self.episode_count, final_length);
	}
}

struct Runtime {
	board:	Board,
	queued_dir:	Direction,
	started: bool,
	use_ai:	bool,
	speed: f64,
	speed_label: &'static str,
	last_step: f64,
	steps_since_food: u32,
}


// Visual loop state
impl Runtime {
	fn new(mode: Mode) -> Self
	{
		let board = Board::new();
		let queued_dir = board.snake.direction.clone();
		let use_ai = matches!(mode, Mode::Predict | Mode::PredictTrain);

		let mut rt = Self
		{
			board,
			queued_dir,
			started: use_ai,
			use_ai,
			speed: SPEED_PLAYER,
			speed_label: "SPEED FOR PLAYER",
			last_step: get_time(),
			steps_since_food: 0,
		};

		if use_ai { rt.set_ai_speed_max(); }
	
		rt
	}

	fn reset_board(&mut self, started: bool)
	{
		self.board= Board::new();
		self.queued_dir = self.board.snake.direction.clone();
		self.last_step= get_time();
		self.steps_since_food = 0;
		self.started = started;
	}

	fn set_ai_speed_max(&mut self)
	{
		self.speed = SPEED_AI_MAX;
		self.speed_label = "SPEED IA MAX";
	}
	
	fn set_ai_speed_min(&mut self)
	{
		self.speed = SPEED_AI_MIN;
		self.speed_label = "SPEED IA MIN";
	}

	fn set_player_speed(&mut self)
	{
		self.speed = SPEED_PLAYER;
		self.speed_label = "SPEED FOR PLAYER";
	}
}

// Macroquad config create
fn window_conf() -> Conf
{
	let (w, h) = window_size();
	
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

	let agent = match build_agent(&args)
	{
		Some(agent) => agent,
		None		=> return,
	};

	if !args.visual { return ; }

	run_visual_loop(agent, &args).await;
}

fn build_agent(args: &Cli) -> Option<Agent>
{
	match args.mode
	{
		Mode::Train			=> build_train_agent(args),
		Mode::Predict		=> build_predict_agent(args),
		Mode::PredictTrain	=> build_predict_train_agent(args),
	}
}

fn build_train_agent(args: &Cli) -> Option<Agent>
{
	let agent = train_basic(args.sessions);

	if save_if_requested(&agent, args.model.as_deref()) { return None; }

	Some(agent)
}

fn build_predict_agent(args: &Cli) -> Option<Agent>
{
	let model_path = required_model_path(args.model.as_deref())?;
	load_agent(model_path)
}

fn build_predict_train_agent(args: &Cli) -> Option<Agent>
{
	let base= load_optional_base_agent(args.model.as_deref())?;
	let agent= train_from_agent(base, args.sessions);

	if !save_if_requested(&agent, args.model.as_deref()) { return None; }

	Some(agent)
}

fn required_model_path(model: Option<&str>) -> Option<&str>
{
	match model
	{
		Some(path) => Some(path),
		None =>
		{
			eprintln!("{}[ERROR]{} --model is required in predict mode", ANSI_RED, ANSI_RESET);
			None
		}
	}
}

fn load_optional_base_agent(model: Option<&str>) -> Option<Agent>
{
	let Some(path) = model else
	{
		return Some(Agent::new());
	};

	if !Path::new(path).exists()
	{
		return Some(Agent::new());
	}

	load_agent(path)
}

fn load_agent(path: &str) -> Option<Agent>
{
	match Agent::load_from_file(path)
	{
		Ok(agent) =>
		{
			println!("{}[LOAD]{} Model loaded from {}", ANSI_GREEN, ANSI_RESET, path);
			Some(agent)
		}
		Err(error) =>
		{
			eprintln!("{}[ERROR]{} Load model failed: {}", ANSI_RED, ANSI_RESET, error);
			None
		}
	}
}

fn save_if_requested(agent: &Agent, model: Option<&str>) -> bool
{
	let Some(path) = model else
	{
		return true;
	};

	match agent.save_to_file(path)
	{
		Ok(_) =>
		{
			println!("{}[SAVE]{} Model saved to {}", ANSI_GREEN, ANSI_RESET, path);
			true
		}
		Err(error) =>
		{
			eprintln!("{}[ERROR]{} Save model failed: {}", ANSI_RED, ANSI_RESET, error);
			false
		}
	}
}

// Visual loop
async fn run_visual_loop(mut agent: Agent, args: &Cli)
{
	let mut runtime = Runtime::new(args.mode);
	let mut stats = Stats::default();
	let dontmlearn_now = matches!(args.mode, Mode::Predict) || args.dontlearn;
	let mut rng = thread_rng();

	loop
	{
		if	is_key_pressed(KeyCode::Escape) { break ; }
	
		handle_speed_keys(&mut runtime);
		handle_direction_keys(&mut runtime);
		handle_ai_toggle_key(&mut runtime);
		handle_debug_key(&runtime.board);

		if handle_dead_state(&mut runtime, &mut stats)
		{
			next_frame().await;
			continue ;
		}

		if runtime.started && can_tick(args, &runtime)
		{
			apply_tick(&mut runtime, &mut agent, &mut stats, dontmlearn_now, args, &mut rng);
		}

		draw_frame(&runtime, &stats);
		next_frame().await;
	}
}

fn handle_speed_keys(runtime: &mut Runtime)
{
	if is_key_pressed(KeyCode::KpSubtract)
	{	
		runtime.set_ai_speed_min();
		println!("{}[SPEED]{} {}", ANSI_YELLOW, ANSI_RESET, runtime.speed_label);
	}

	if is_key_pressed(KeyCode::KpAdd)
	{
		runtime.set_ai_speed_max();
		println!("{}[SPEED]{} {}", ANSI_YELLOW, ANSI_RESET, runtime.speed_label);
	}

	if is_key_pressed(KeyCode::Backspace)
	{
	runtime.set_player_speed();
        println!("{}[SPEED]{} {}", ANSI_YELLOW, ANSI_RESET, runtime.speed_label);
	}
}

fn handle_direction_keys(runtime: &mut Runtime)
{
	if is_key_pressed(KeyCode::Up)
	{
		runtime.queued_dir = Direction::Up;
		runtime.started = true;
	}

	if is_key_pressed(KeyCode::Down)
	{
		runtime.queued_dir = Direction::Down;
		runtime.started = true;
	}

	if is_key_pressed(KeyCode::Left)
	{
		runtime.queued_dir = Direction::Left;
		runtime.started = true;
	}

	if is_key_pressed(KeyCode::Right)
	{
		runtime.queued_dir = Direction::Right;
		runtime.started = true;
	}
}

fn handle_ai_toggle_key(runtime: &mut Runtime)
{
	if !is_key_pressed(KeyCode::Enter) { return ; }

	runtime.use_ai = !runtime.use_ai;
	runtime.started = true;

	if runtime.use_ai { runtime.set_ai_speed_max(); } 
	else { runtime.set_player_speed(); }

    println!("{}[MODE]{} AI {}", ANSI_CYAN, ANSI_RESET, if runtime.use_ai { "ON" } else { "OFF" });
}

fn handle_debug_key(board: &Board)
{
	if !is_key_pressed(KeyCode::Space) { return ; }

	let state = compute_state(board);
	println!("{}[DEBUG]{} State index = {}", ANSI_CYAN, ANSI_RESET, state.to_index());
	print_state(board);
}
