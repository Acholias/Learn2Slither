// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   train.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/12 17:16:48 by lumugot           #+#    #+#             //
//   Updated: 2026/04/16 00:04:20 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use ::rand::thread_rng;
use crate::agent::Agent;
use crate::env::Env;


pub fn train_basic(sessions: u32) -> Agent
{
    train_from_agent(Agent::new(), sessions)
}

pub fn train_from_agent(mut agent: Agent, sessions: u32) -> Agent
{
    let mut env= Env::new();
    let mut rng = thread_rng();
    let max_steps: u32 = 200;

    for _episode in 0..sessions
    {
        let mut state = env.reset();
        let mut done = false;
        let mut steps = 0;
        let mut max_lenght = 0;

        while !done
        {
            let action = agent.select_action(&state, false, &mut rng);
            let (next_state, reward, next_done, _step_result) = env.step(action);

            agent.update(&state, &action, reward, &next_state, next_done);
        
            let len = env.board.snake.lenght();
            if len > max_lenght { max_lenght = len; }
        
            state = next_state;
            done = next_done;
            steps += 1;
        
            if steps >= max_steps { break ; }
        }
    }
    agent
}
