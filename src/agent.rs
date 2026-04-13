// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   agent.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/12 13:50:36 by lumugot           #+#    #+#             //
//   Updated: 2026/04/13 15:06:28 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::action::Action;
use crate::state::State;

const STATE_COUNT: usize = 256;
const ACTION_COUNT: usize = 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    pub q_table: Vec<[f32; ACTION_COUNT]>,
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon: f32,
    pub epsilon_min: f32,
    pub epsilon_decay: f32,
}

impl Agent {
    pub fn new() -> Self
    {
        Agent {
            q_table: vec![[0.0; ACTION_COUNT]; STATE_COUNT],
            alpha: 0.1,
            gamma: 0.95,
            epsilon: 1.0,
            epsilon_min: 0.05,
            epsilon_decay: 0.999,
        }
    }

    fn best_action_index(&self, state_idx: usize) -> usize
    {
        let q_vals = &self.q_table[state_idx];

        let (best_idx, _) = q_vals.iter().enumerate().max_by(|a, b|a.1  .partial_cmp(b.1).unwrap()).unwrap();

        best_idx
    }

    pub fn select_action<R: Rng>(&mut self, state: &State, dontlearn: bool, rng: &mut R) -> Action
    {
        let state_idx = state.to_index();

        let eps = if dontlearn { 0.0 } else { self.epsilon };

        if rng.gen::<f32>() < eps
        {
            let index = rng.gen_range(0..ACTION_COUNT);
            Action::ALL[index]
        }
        else
        {
            let index = self.best_action_index(state_idx);
            Action::ALL[index]
        }
    }

    pub fn update(&mut self, state: &State, action: &Action, reward: f32, next_state: &State, done: bool)
    {
        let s_index = state.to_index();
        let a_index = action.index();
        let next_index = next_state.to_index();

        let q_sa = self.q_table[s_index][a_index];

        let target = if done
        {
            reward
        }
        else
        {
            let max_next= self.q_table[next_index].iter().copied().fold(f32::NEG_INFINITY, f32::max);

            reward + self.gamma * max_next
        };
        self.q_table[s_index][a_index] = q_sa + self.alpha * (target - q_sa);

        if !done && self.epsilon > self.epsilon_min
        {
            self.epsilon *= self.epsilon_decay;
        }
    }
}
