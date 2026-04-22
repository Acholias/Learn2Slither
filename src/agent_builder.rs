// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   agent_builder.rs                                   :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/22 14:49:08 by lumugot           #+#    #+#             //
//   Updated: 2026/04/22 14:50:25 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use std::path::Path;
use crate::agent::Agent;
use crate::cli::{Cli, Mode};
use crate::train::{train_basic, train_from_agent};
use crate::logger::{ANSI_GREEN, ANSI_RED, ANSI_RESET, log_plain, log_err_plain};

pub fn build_agent(args: &Cli) -> Option<Agent>
{
	match args.mode
	{
		Mode::Train			=> build_train_agent(args),
		Mode::Predict		=> build_predict_agent(args),
		Mode::PredictTrain	=> build_predict_train_agent(args),
	}
}

pub fn build_train_agent(args: &Cli) -> Option<Agent>
{
	let agent = train_basic(args.sessions);

	if !save_if_requested(&agent, args.model.as_deref()) { return None; }

	Some(agent)
}

pub fn build_predict_agent(args: &Cli) -> Option<Agent>
{
	let model_path = required_model_path(args.model.as_deref())?;
	load_agent(model_path)
}

pub fn build_predict_train_agent(args: &Cli) -> Option<Agent>
{
	let base= load_optional_base_agent(args.model.as_deref())?;
	let agent= train_from_agent(base, args.sessions);

	if !save_if_requested(&agent, args.model.as_deref()) { return None; }

	Some(agent)
}

pub fn required_model_path(model: Option<&str>) -> Option<&str>
{
	match model
	{
		Some(path) => Some(path),
		None =>
		{
			log_err_plain(format!("{}[ERROR]{} --model is required in predict mode", ANSI_RED, ANSI_RESET));
			None
		}
	}
}

pub fn load_optional_base_agent(model: Option<&str>) -> Option<Agent>
{
	let Some(path) = model else
	{
		return Some(Agent::new());
	};

	if !Path::new(path).exists()
	{
		return Some(Agent::new());
	}

	load_agent(path)
}

pub fn load_agent(path: &str) -> Option<Agent>
{
	match Agent::load_from_file(path)
	{
		Ok(agent) =>
		{
			log_plain(format!("{}[LOAD]{} Model loaded from {}", ANSI_GREEN, ANSI_RESET, path));
			Some(agent)
		}
		Err(error) =>
		{
			log_err_plain(format!("{}[ERROR]{} Load model failed: {}", ANSI_RED, ANSI_RESET, error));
			None
		}
	}
}

pub fn save_if_requested(agent: &Agent, model: Option<&str>) -> bool
{
	let Some(path) = model else
	{
		return true;
	};

	match agent.save_to_file(path)
	{
		Ok(_) =>
		{
			log_plain(format!("{}[SAVE]{} Model saved to {}", ANSI_GREEN, ANSI_RESET, path));
			true
		}
		Err(error) =>
		{
			log_err_plain(format!("{}[ERROR]{} Save model failed: {}", ANSI_RED, ANSI_RESET, error));
			false
		}
	}
}
