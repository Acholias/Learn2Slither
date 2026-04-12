// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   rewards.rs                                         :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/12 13:30:41 by lumugot           #+#    #+#             //
//   Updated: 2026/04/12 13:35:09 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use crate::board::StepResult;

pub const REWARD_GREEN: f32     = 1.0;
pub const REWARD_RED: f32       = -1.0;
pub const REWARD_MOVE: f32      = -0.01;
pub const REWARD_GAME_OVER: f32 = -1.0;

pub fn compute_reward(result: &StepResult) -> f32
{
    match result
    {
        StepResult::AteGreen    => REWARD_GREEN,
        StepResult::AteRed      => REWARD_RED,
        StepResult::Moved       => REWARD_MOVE,
        StepResult::GameOver    => REWARD_GAME_OVER,
    }
}
