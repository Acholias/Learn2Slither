// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   logger.rs                                          :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/22 14:35:13 by lumugot           #+#    #+#             //
//   Updated: 2026/04/29 20:56:07 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use crate::runtime::Runtime;

pub const ANSI_RESET: &str	= "\x1b[0m";
pub const ANSI_GREEN: &str	= "\x1b[32m";
pub const ANSI_CYAN: &str	= "\x1b[36m";
pub const ANSI_YELLOW: &str	= "\x1b[33m";
pub const ANSI_RED: &str	= "\x1b[31m";

pub fn log_plain(message: impl std::fmt::Display)
{
	println!("{}", message);
}

pub fn log_err_plain(message: impl std::fmt::Display)
{
	eprintln!("{}", message);
}

pub fn log(runtime: &Runtime, message: impl std::fmt::Display)
{
	if runtime.show_logs
	{
		log_plain(message);
	}
}
