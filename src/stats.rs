// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   stats.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/22 14:36:46 by lumugot           #+#    #+#             //
//   Updated: 2026/04/29 20:48:11 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

#[derive(Default)]
pub struct Stats {
	pub episode_count: u64,
	pub best_lenght: usize,
	pub total_length: u64,
}

impl Stats {
	pub fn	average(&self) -> f32
	{
		if self.episode_count == 0 { 0.0 }
		else { self.total_length as f32 / self.episode_count as f32 }
	}

	pub fn	update_best(&mut self, current_length: usize)
	{
		if current_length > self.best_lenght
		{
			self.best_lenght= current_length;
		}
	}

	pub fn	close_ai_episode(&mut self, final_length: usize)
	{
		self.episode_count += 1;
		self.total_length += final_length as u64;
	}
}
