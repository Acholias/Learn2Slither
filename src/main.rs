// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   main.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:09:13 by lumugot           #+#    #+#             //
//   Updated: 2026/04/12 16:47:07 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;
use ::rand::thread_rng;

mod board;
mod snake;
mod display;
mod state;
mod action;
mod rewards;
mod env;
mod agent;
mod train;

use board::{Board, Direction, StepResult};
use display::{draw_board, draw_game_over, window_size};
use crate::state::{compute_state, print_state};
use crate::train::train_basic;
use crate::agent::Agent;
use crate::rewards::compute_reward;

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
    let run_training = false;

    if run_training
    {
        let _agent = train_basic(10);
        return;
    }

    let mut board = Board::new();
    let speed: f64 = 0.03;
    let mut last_step = get_time();
    let mut queued_dir = board.snake.direction.clone();
    let mut started = false;
    let mut use_ai = false;
    let mut agent = Agent::new();
    let mut rng = thread_rng();
    let mut episode_count: u64 = 0;

    loop
    {
        if is_key_pressed(KeyCode::Escape) { break; }

        if is_key_pressed(KeyCode::Up)
        {
            queued_dir = Direction::Up;
            started = true;
        }
        if is_key_pressed(KeyCode::Down)
        {
            queued_dir = Direction::Down;
            started = true;
        }
        if is_key_pressed(KeyCode::Left)
        {
            queued_dir = Direction::Left;
            started = true;
        }
        if is_key_pressed(KeyCode::Right)
        {
            queued_dir = Direction::Right;
            started = true;
        }

        if is_key_pressed(KeyCode::Enter)
        {
            use_ai = !use_ai;
            started = true;
            println!("AI mode: {}", if use_ai { "ON" } else { "OFF" });
        }

        if is_key_pressed(KeyCode::Space)
        {
            let s = compute_state(&board);
            println!("State index = {}", s.to_index());
            print_state(&board);
        }

        if !board.snake.alive
        {
            if use_ai
            {
                episode_count += 1;
                agent.next_gen();
                println!(
                    "AI episode {} finished. Final length = {}",
                    episode_count,
                    board.snake.lenght()
                );
                board = Board::new();
                queued_dir = board.snake.direction.clone();
                last_step = get_time();
                started = true;
                continue;
            }
            else
            {
                draw_game_over(&board);
                if is_key_pressed(KeyCode::R)
                {
                    board = Board::new();
                    queued_dir = board.snake.direction.clone();
                    last_step = get_time();
                    started = false;
                }
                next_frame().await;
                continue;
            }
        }

        if started && get_time() - last_step >= speed
        {
            last_step = get_time();

            let result = if use_ai
            {
                let state = compute_state(&board);
                let action = agent.select_action(&state, false, &mut rng);
                let dir = action.to_direction();
                let step_res = board.step(dir);
                let reward = compute_reward(&step_res);
                let next_state = compute_state(&board);
                let done = matches!(step_res, StepResult::GameOver);
                agent.update(&state, &action, reward, &next_state, done);
                step_res
            }
            else
            {
                board.step(queued_dir.clone())
            };

            match result
            {
                StepResult::AteGreen => {
                    println!("Ate green apple! Length: {}", board.snake.lenght());
                }
                StepResult::AteRed => {
                    println!("Ate red apple! Length: {}", board.snake.lenght());
                }
                StepResult::GameOver => {
                    println!("Game over! Final length: {}", board.snake.lenght());
                }
                StepResult::Moved => {}
            }
        }

        if !started
        {
            let text = "Press arrows to play, ENTER = AI, SPACE = debug";
            draw_board(&board);
            draw_text(text, 20.0, screen_height() - 10.0, 20.0, YELLOW);
            if use_ai
            {
                let epi_text = format!("Episodes: {}", episode_count);
                draw_text(&epi_text, 20.0, 30.0, 24.0, WHITE);
            }
            next_frame().await;
            continue;
        }

        draw_board(&board);
        if use_ai
        {
            let epi_text = format!("Episodes: {}", episode_count);
            draw_text(&epi_text, 20.0, 30.0, 24.0, WHITE);
        }
        next_frame().await;
    }
}
