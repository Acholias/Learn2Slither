// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   main.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:09:13 by lumugot           #+#    #+#             //
//   Updated: 2026/04/22 09:53:52 by lumugot          ###   ########.fr       //
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

use board::{Board, Direction, StepResult, DEFAULT_BOARD_SIZE};
use display::{draw_board, draw_game_over, panel_left_x, panel_padding, window_size};
use crate::agent::Agent;
use crate::board::validate_board_size;
use crate::state::{compute_state, compute_vision};
use crate::train::{train_basic, train_from_agent};
use cli::{Cli, Mode};
use std::sync::OnceLock;

static BOARD_SIZE_GLOBAL: OnceLock<usize> = OnceLock::new();

pub const ANSI_RESET: &str	= "\x1b[0m";
pub const ANSI_GREEN: &str	= "\x1b[32m";
pub const ANSI_CYAN: &str	= "\x1b[36m";
pub const ANSI_YELLOW: &str	= "\x1b[33m";
pub const ANSI_RED: &str	= "\x1b[31m";

const SPEED_PLAYER: f64	= 0.1;
const SPEED_AI_MAX: f64	= 0.0001;
const SPEED_AI_MIN: f64	= 0.05;
const NO_FOOD_MAX: u32	= 100;

fn log_plain(message: impl std::fmt::Display)
{
	println!("{}", message);
}

fn log_err_plain(message: impl std::fmt::Display)
{
	eprintln!("{}", message);
}


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
			log_plain(format!("{}[RECORD]{} New length record: {}", ANSI_GREEN, ANSI_RESET, self.best_lenght));
		}
	}

	fn close_ai_episode(&mut self, final_length: usize)
	{
		self.episode_count += 1;
		self.total_length += final_length as u64;
		log_plain(format!("{}[EPISODE]{} #{} finished. Final lenght = {}", ANSI_CYAN, ANSI_RESET, self.episode_count, final_length));
	}
}

struct Runtime {
	board:	Board,
	board_size: usize,
	queued_dir:	Direction,
	started: bool,
	paused: bool,
	use_ai:	bool,
	speed: f64,
	speed_label: &'static str,
	last_step: f64,
	steps_since_food: u32,
	show_vision: bool,
	show_logs: bool,
	show_help: bool,
}

fn log(runtime: &Runtime, message: impl std::fmt::Display)
{
	if runtime.show_logs
	{
		log_plain(message);
	}
}

// Visual loop state for reset board after IA or PLAYER die
impl Runtime {
	fn new(mode: Mode, board_size: usize) -> Self
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
		};

		if use_ai { rt.set_ai_speed_min(); }
	
		else { rt.set_player_speed(); }
	
		rt
	}

	fn reset_board(&mut self, started: bool)
	{
		self.board= Board::new(self.board_size);
		self.queued_dir = self.board.snake.direction.clone();
		self.last_step= get_time();
		self.steps_since_food = 0;
		self.started = started;
		self.paused = false;
		self.show_help = true;
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

// Macroquad config create window with parameter setup
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

	if !save_if_requested(&agent, args.model.as_deref()) { return None; }

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
			log_err_plain(format!("{}[ERROR]{} --model is required in predict mode", ANSI_RED, ANSI_RESET));
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
			log_plain(format!("{}[LOAD]{} Model loaded from {}", ANSI_GREEN, ANSI_RESET, path));
			Some(agent)
		}
		Err(error) =>
		{
			log_err_plain(format!("{}[ERROR]{} Load model failed: {}", ANSI_RED, ANSI_RESET, error));
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
			log_plain(format!("{}[SAVE]{} Model saved to {}", ANSI_GREEN, ANSI_RESET, path));
			true
		}
		Err(error) =>
		{
			log_err_plain(format!("{}[ERROR]{} Save model failed: {}", ANSI_RED, ANSI_RESET, error));
			false
		}
	}
}

// Visual loop
async fn run_visual_loop(mut agent: Agent, args: &Cli, board_size: usize)
{
	let mut runtime = Runtime::new(args.mode, board_size);
	let mut stats = Stats::default();
	let dontmlearn_now = matches!(args.mode, Mode::Predict) || args.dontlearn;
	let mut rng = thread_rng();

	loop
	{
		if is_key_pressed(KeyCode::Escape) { break ; }
	
		handle_speed_keys(&mut runtime);
		handle_mode_toggle_key(&mut runtime);
		handle_direction_keys(&mut runtime);
		handle_enter_key(&mut runtime);
		handle_debug_key(&mut runtime);

		if handle_dead_state(&mut runtime, &mut stats)
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

fn handle_speed_keys(runtime: &mut Runtime)
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

fn handle_direction_keys(runtime: &mut Runtime)
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

fn handle_mode_toggle_key(runtime: &mut Runtime)
{
	if runtime.started { return ; }
	if !is_key_pressed(KeyCode::Tab) { return ; }

	runtime.use_ai = !runtime.use_ai;

	if runtime.use_ai { runtime.set_ai_speed_min(); } 
	else { runtime.set_player_speed(); }

	log(runtime, format!("{}[MODE]{} AI {}", ANSI_CYAN, ANSI_RESET, if runtime.use_ai { "ON" } else { "OFF" }));
}

fn handle_enter_key(runtime: &mut Runtime)
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

fn handle_debug_key(runtime: &mut Runtime)
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

// Game state transitions
fn handle_dead_state(runtime: &mut Runtime, stats: &mut Stats) -> bool
{
	if runtime.board.snake.alive { return false; }

	if runtime.use_ai
	{
		let final_length = runtime.board.snake.lenght();
		stats.close_ai_episode(final_length);
		runtime.reset_board(true);
		return true;
	}

	draw_game_over(&runtime.board);

	if is_key_pressed(KeyCode::R) { runtime.reset_board(false); }

	true
}

fn can_tick(args: &Cli, runtime: &Runtime) -> bool
{
	if args.step && runtime.use_ai { is_key_pressed(KeyCode::N) }

	else { get_time() - runtime.last_step >= runtime.speed }
}

fn apply_tick(runtime: &mut Runtime, agent: &mut Agent, stats: &mut Stats, 
	dontlearn_now: bool, args: &Cli, rng: &mut impl ::rand::Rng)
{
	if !(args.step && runtime.use_ai) { runtime.last_step = get_time(); }
	let result = if runtime.use_ai { tick_ai(runtime, agent, dontlearn_now, rng) }
	else { tick_player(runtime) };

	update_after_step(runtime, stats, result);	
}

fn tick_ai(runtime: &mut Runtime, agent: &mut Agent, dontlearn_now: bool, rng: &mut impl ::rand::Rng) -> StepResult
{
	let state = compute_state(&runtime.board);
	let action= agent.select_action(&state, dontlearn_now, rng);
	
	runtime.board.step(action.to_direction())
}

fn tick_player(runtime: &mut Runtime) -> StepResult
{
	runtime.board.step(runtime.queued_dir.clone())
}

fn update_after_step(runtime: &mut Runtime, stats: &mut Stats, result: StepResult)
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

	stats.update_best(runtime.board.snake.lenght())
}

// Functions for draw
fn draw_frame(runtime: &Runtime, stats: &Stats)
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

fn draw_hud(runtime: &Runtime, stats: &Stats)
{
	let x = panel_left_x(runtime.board_size) + panel_padding();
	let mut y = 30.0;
	let fs = 24.0;
	let line = 28.0;

	draw_text(&format!("Ep   : {}", stats.episode_count), x, y, fs, WHITE); y += line;
	draw_text(&format!("PR   : {}", stats.best_lenght), x, y, fs, WHITE); y += line;
	draw_text(&format!("Avg  : {:.2}", stats.average()), x, y, fs, WHITE); y += line;
	draw_text(&format!("Speed: {}", runtime.speed_label), x, y, fs, WHITE);
}

fn draw_mode_status(runtime: &Runtime)
{
	let fs = 26.0;
	let pad = panel_padding();

	let text = if runtime.use_ai { "IA: ON" } else { "IA: OFF" };
	let dims= measure_text(text, None, fs as u16, 1.0);

	let x = screen_width() - pad - dims.width - 820.0;
	let y = screen_height() - pad;

	let color = if runtime.use_ai { GREEN } else { RED };
	draw_text(text, x, y, fs, color);
}

fn draw_help_menu(runtime: &Runtime)
{
	if !runtime.show_help { return ; }

	let fs = 22.0;
	let line = 26.0;
	let pad = panel_padding();

	let x = panel_left_x(runtime.board_size) + pad;
	let w = screen_width() - x - pad;

	let h = 7.0 * line + 8.0;
	let y = screen_height() - pad - h;

	draw_rectangle(x - 10.0, y - 22.0, w + 20.0, h + 70.0, Color::new(0.0, 0.0, 0.0, 0.75));

	let mut ty = y;
	draw_text("CONTROLS (H to close)", x, ty, fs, WHITE); ty += line;
	draw_text("TAB    : toggle AI (before start)", x, ty, fs, WHITE); ty += line;
	draw_text("ENTER  : start / pause", x, ty, fs, WHITE); ty += line;
	draw_text("ARROWS : start + move (Player)", x, ty, fs, WHITE); ty += line;
	draw_text("SPACE  : vision", x, ty, fs, WHITE); ty += line;
	draw_text("D      : logs", x, ty, fs, WHITE); ty += line;
	draw_text("ESC    : quit", x, ty, fs, WHITE);
}

fn draw_vision_overlay(board: &Board, runtime: &Runtime)
{
	let v = compute_vision(board);

	let x = panel_left_x(runtime.board_size) + panel_padding();
	let fs = 30.0;
	let line = 34.0;
	let lines = 5.0;

	let block_h = (lines - 1.0) * line;
	let mut y = screen_height() * 0.5 - block_h * 0.5;

	draw_text(&format!("Head : ({}, {})", v.head.0, v.head.1), x, y, fs, WHITE); y += line;
	draw_text(&format!("Up   : {}", v.up), x, y, fs, WHITE); y += line;
	draw_text(&format!("Down : {}", v.down), x, y, fs, WHITE); y += line;
	draw_text(&format!("Left : {}", v.left), x, y, fs, WHITE); y += line;
	draw_text(&format!("Right: {}", v.right), x, y, fs, WHITE);
}
