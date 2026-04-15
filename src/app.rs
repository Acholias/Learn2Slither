// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   app.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/16 00:56:05 by lumugot           #+#    #+#             //
//   Updated: 2026/04/16 01:00:35 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use crate::agent::Agent;
use crate::cli::{Cli, Mode};
use crate::game_loop::run_visual_loop;
use crate::train::{train_basic, train_from_agent};
use std::path::Path;

pub async fn run(args: Cli)
{
	let agent = match build_agent(&args) {
		Some(agent)	=> agent,
		None		=> return,
	};

	if !args.visual { return ; }

	run_visual_loop(agent, &args).await;
}

fn build_agent(args: &Cli) -> Option<Agent>
{

}
