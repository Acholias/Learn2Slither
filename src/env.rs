// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   env.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/12 13:38:42 by lumugot           #+#    #+#             //
//   Updated: 2026/04/12 13:49:49 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use crate::action::Action;
use crate::board::{Board, StepResult};
use crate::rewards::compute_reward;
use crate::state::{compute_state, State};

pub struct Env {
    pub board: Board,
}

impl Env {
    pub fn new() -> Self
    {
        Env {board: Board::new(), }
    }

    pub fn reset(&mut self) -> State
    {
        self.board = Board::new();
        compute_state(&self.board)
    }

    pub fn step(&mut self, action: Action) -> (State, f32, bool, StepResult)
    {
        // Distance to nearest green apple before the move (for reward shaping)
        let prev_green_dist = closest_green_distance(&self.board);
        let dir = action.to_direction();
        let result = self.board.step(dir);
        let mut reward = compute_reward(&result);
        let done = matches!(result, StepResult::GameOver);
        let next_state = compute_state(&self.board);

        // Reward shaping: encourage moving closer to green apples, discourage moving away.
        if !done {
            let new_green_dist = closest_green_distance(&self.board);
            if new_green_dist < prev_green_dist {
                reward += 0.2;
            } else if new_green_dist > prev_green_dist {
                reward -= 0.2;
            }
        }

        (next_state, reward, done, result)
    }
}

fn closest_green_distance(board: &Board) -> f32
{
    let head = board.snake.head();
    let mut best: Option<i32> = None;

    for &(gr, gc) in &board.green_apples
    {
        let d = (head.0 as i32 - gr as i32).abs() + (head.1 as i32 - gc as i32).abs();
        best = match best {
            Some(cur) if cur <= d => Some(cur),
            _ => Some(d),
        };
    }

    best.unwrap_or(0) as f32
}
