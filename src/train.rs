use ::rand::thread_rng;

use crate::agent::Agent;
use crate::env::Env;

pub fn train_basic(sessions: u32) -> Agent
{
    let mut env = Env::new();
    let mut agent = Agent::new();
    let mut rng = thread_rng();

    for episode in 0..sessions {
        let mut state = env.reset();
        let mut done = false;
        let mut steps = 0;
        let mut max_length = 0;

        while !done {
            let action = agent.select_action(&state, false, &mut rng);
            let (next_state, reward, next_done, _step_result) = env.step(action);

            agent.update(&state, &action, reward, &next_state, next_done);

            let len = env.board.snake.lenght();
            if len > max_length {
                max_length = len;
            }

            state = next_state;
            done = next_done;
            steps += 1;

            if steps > 1000 {
                break;
            }
        }

        println!(
            "Episode {}: max_length = {}, steps = {}",
            episode + 1,
            max_length,
            steps
        );
    }

    agent
}
