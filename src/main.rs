// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   main.rs                                            :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:09:13 by lumugot           #+#    #+#             //
//   Updated: 2026/04/10 20:19:14 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use macroquad::prelude::*;

mod board;
mod snake;
mod display;
mod state;

use board::{Board, Direction, StepResult};
use display::{draw_board, draw_game_over, window_size};
use crate::state::{compute_state, print_state};

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
    let mut board = Board::new();
    let speed: f64 = 0.15;
    let mut last_step = get_time();
    let mut queued_dir = board.snake.direction.clone();

    let mut started = false;

    loop
    {
        if is_key_pressed(KeyCode::Escape) { break ; }

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

        if is_key_pressed(KeyCode::Space)
        {
            let s = compute_state(&board);
            println!("State index = {}", s.to_index());
            print_state(&board);
        }

        if !board.snake.alive
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

        if started && get_time() - last_step >= speed
        {
            let result = board.step(queued_dir.clone());
            last_step = get_time();

            match result
            {
                StepResult::AteGreen => println!("Ate green apple! Length: {}", board.snake.lenght()),
                StepResult::AteRed   => println!("Ate red apple! Length: {}", board.snake.lenght()),
                StepResult::GameOver => println!("Game over! Final length: {}", board.snake.lenght()),
                StepResult::Moved    => {},
            }
        }

        if !started
        {
            let text = "Press any arrow key to start";
            draw_board(&board);
            draw_text(text, 20.0, screen_height() - 10.0, 20.0, YELLOW);
            next_frame().await;
            continue;
        }
        draw_board(&board);
        next_frame().await;
    }
}
