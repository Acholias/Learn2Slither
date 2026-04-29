// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   cli.rs                                             :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/15 11:08:00 by lumugot           #+#    #+#             //
//   Updated: 2026/04/29 20:22:03 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "snake", author, version, about = "Learn2Slither - snake with Reinforcement Learning")]
pub struct Cli {
    #[arg(long, value_enum)]
    pub mode: Mode,

    #[arg(long, short = 'n', default_value_t = 10_000)]
    pub sessions: u32,

    #[arg(long, short = 'm')]
    pub model: Option<String>,

    #[arg(long)]
    pub visual: bool,

    #[arg(long)]
    pub dontlearn: bool,

    #[arg(long)]
    pub step: bool,

	#[arg(long, default_value_t = 10)]
	pub board_size: u32,

	#[arg(long)]
	pub hud: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Mode {
    Train,
    Predict,
    PredictTrain,
}

impl Cli {
    pub fn parse_args() -> Self
    {
        Cli::parse()
    }
}
