// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   main.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:09:13 by lumugot           #+#    #+#             //
//   Updated: 2026/04/16 00:16:17 by lumugot          ###   ########.fr       //
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

fn window_conf() -> Conf
{
	let (w, h) = window_size();
	Conf {
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

    let mut agent = train_basic(args.sessions);

    let mut board = Board::new();
    let mut speed: f64 = 0.1;
    let mut last_step = get_time();
    let mut queued_dir = board.snake.direction.clone();
    let mut started = false;
    let mut use_ai = false;
    let mut rng = thread_rng();
    let mut episode_count: u64 = 0;
    let mut best_length: usize = 0;
    let mut total_length: u64 = 0;
    let mut steps_since_food: u32 = 0;
    let no_food_max: u32 = 100;
    let mut speed_msg = String::from("");
        let agent = match build_agent(&args) {
            Some(agent) => agent,
            None => return,
        };

        if !args.visual {
            return;
        }

        run_visual_loop(agent, &args).await;
    }

    fn build_agent(args: &Cli) -> Option<Agent>
    {
        match args.mode {
            Mode::Train => {
                let agent = train_basic(args.sessions);

                if let Some(model_path) = args.model.as_deref() {
                    if let Err(error) = agent.save_to_file(model_path) {
                        eprintln!("Save model failed: {}", error);
                        return None;
                    }
                    println!("Model saved to {}", model_path);
                }

                Some(agent)
            }
            Mode::Predict => {
                let model_path = match args.model.as_deref() {
                    Some(path) => path,
                    None => {
                        eprintln!("--model is required in predict mode");
                        return None;
                    }
                };

                match Agent::load_from_file(model_path) {
                    Ok(agent) => {
                        println!("Model loaded from {}", model_path);
                        Some(agent)
                    }
                    Err(error) => {
                        eprintln!("Load model failed: {}", error);
                        None
                    }
                }
            }
            Mode::PredictTrain => {
                let mut base_agent = Agent::new();

                if let Some(model_path) = args.model.as_deref() {
                    if Path::new(model_path).exists() {
                        match Agent::load_from_file(model_path) {
                            Ok(agent) => {
                                println!("Model loaded from {}", model_path);
                                base_agent = agent;
                            }
                            Err(error) => {
                                eprintln!("Load model failed: {}", error);
                                return None;
                            }
                        }
                    }
                }

                let agent = train_from_agent(base_agent, args.sessions);

                if let Some(model_path) = args.model.as_deref() {
                    if let Err(error) = agent.save_to_file(model_path) {
                        eprintln!("Save model failed: {}", error);
                        return None;
                    }
                    println!("Model saved to {}", model_path);
                }

                Some(agent)
            }
        }
    }

    async fn run_visual_loop(mut agent: Agent, args: &Cli)
    {
        let mut board = Board::new();
        let mut speed: f64 = if matches!(args.mode, Mode::Predict | Mode::PredictTrain) { 0.0001 } else { 0.1 };
        let mut last_step = get_time();
        let mut queued_dir = board.snake.direction.clone();
        let mut started = matches!(args.mode, Mode::Predict | Mode::PredictTrain);
        let mut use_ai = matches!(args.mode, Mode::Predict | Mode::PredictTrain);
        let mut rng = thread_rng();
        let mut episode_count: u64 = 0;
        let mut best_length: usize = 0;
        let mut total_length: u64 = 0;
        let mut steps_since_food: u32 = 0;
        let no_food_max: u32 = 100;
        let mut speed_msg = if use_ai { String::from("MAX SPEED IA") } else { String::from("SPEED FOR PLAYER") };
        let dontlearn_now = matches!(args.mode, Mode::Predict) || args.dontlearn;

        loop {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            if is_key_pressed(KeyCode::KpSubtract) {
                speed = 0.05;
                speed_msg = String::from("MIN SPEED IA");
            }

            if is_key_pressed(KeyCode::KpAdd) {
                speed = 0.0001;
                speed_msg = String::from("MAX SPEED IA");
            }

            if is_key_pressed(KeyCode::Backspace) {
                speed = 0.1;
                speed_msg = String::from("SPEED FOR PLAYER");
            }

            if is_key_pressed(KeyCode::Up) {
                queued_dir = Direction::Up;
                started = true;
            }

            if is_key_pressed(KeyCode::Down) {
                queued_dir = Direction::Down;
                started = true;
            }

            if is_key_pressed(KeyCode::Left) {
                queued_dir = Direction::Left;
                started = true;
            }

            if is_key_pressed(KeyCode::Right) {
                queued_dir = Direction::Right;
                started = true;
            }

            if is_key_pressed(KeyCode::Enter) {
                use_ai = !use_ai;
                started = true;
                if use_ai {
                    speed = 0.0001;
                    speed_msg = String::from("MAX SPEED IA");
                } else {
                    speed = 0.1;
                    speed_msg = String::from("SPEED FOR PLAYER");
                }
                println!("AI mode: {}", if use_ai { "ON" } else { "OFF" });
            }

            if is_key_pressed(KeyCode::Space) {
                let state = compute_state(&board);
                println!("State index = {}", state.to_index());
                print_state(&board);
            }

            if !board.snake.alive {
                if use_ai {
                    episode_count += 1;
                    let final_len = board.snake.lenght() as u64;
                    total_length += final_len;
                    println!("AI episode {} finished. Final length = {}", episode_count, final_len);
                    board = Board::new();
                    queued_dir = board.snake.direction.clone();
                    last_step = get_time();
                    steps_since_food = 0;
                    started = true;
                    continue;
                }

                draw_game_over(&board);
                if is_key_pressed(KeyCode::R) {
                    board = Board::new();
                    queued_dir = board.snake.direction.clone();
                    last_step = get_time();
                    started = false;
                }
                next_frame().await;
                continue;
            }

            let can_tick = if args.step && use_ai {
                is_key_pressed(KeyCode::N)
            } else {
                get_time() - last_step >= speed
            };

            if started && can_tick {
                if !(args.step && use_ai) {
                    last_step = get_time();
                }

                let result = if use_ai {
                    let state = compute_state(&board);
                    let action = agent.select_action(&state, dontlearn_now, &mut rng);
                    let dir = action.to_direction();
                    board.step(dir)
                } else {
                    board.step(queued_dir.clone())
                };

                match result {
                    StepResult::GameOver => {
                        println!("Game over! Final length: {}", board.snake.lenght());
                    }
                    StepResult::Moved => {}
                    StepResult::AteGreen => {
                        steps_since_food = 0;
                    }
                    StepResult::AteRed => {
                        steps_since_food = 0;
                    }
                }

                if use_ai {
                    steps_since_food += 1;
                    if steps_since_food >= no_food_max {
                        board.snake.alive = false;
                    }
                }

                let current_len = board.snake.lenght();
                if current_len > best_length {
                    best_length = current_len;
                    println!("New length record: {}", best_length);
                }
            }

            if !started {
                let text = "Press arrows to play, ENTER = AI, SPACE = debug";
                draw_board(&board);
                let avg = if episode_count > 0 { total_length as f32 / episode_count as f32 } else { 0.0 };
                let hud = format!("Ep: {} | PR: {} | Avg: {:.2}  | Speed: {}", episode_count, best_length, avg, speed_msg);
                draw_text(&hud, 200.0, 30.0, 24.0, WHITE);
                draw_text(text, 20.0, screen_height() - 10.0, 25.0, RED);
                if use_ai {
                    let avg = if episode_count > 0 { total_length as f32 / episode_count as f32 } else { 0.0 };
                    let hud = format!("Ep: {} | PR: {} | Avg: {:.2}  | Speed: {}", episode_count, best_length, avg, speed_msg);
                    draw_text(&hud, 200.0, 30.0, 24.0, WHITE);
                }
                next_frame().await;
                continue;
            }

            draw_board(&board);
            if use_ai {
                let avg = if episode_count > 0 { total_length as f32 / episode_count as f32 } else { 0.0 };
                let hud = format!("Ep: {} | PR: {} | Avg: {:.2}  | Speed: {}", episode_count, best_length, avg, speed_msg);
                draw_text(&hud, 200.0, 30.0, 24.0, WHITE);
            }
            next_frame().await;
        }
    }
