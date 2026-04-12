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
        let dir= action.to_direction();
        let result = self.board.step(dir);
        let reward = compute_reward(&result);
        let done = matches!(result, StepResult::GameOver);
        let next_state = compute_state(&self.board);

        (next_state, reward, done, result)
    }
}
