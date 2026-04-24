// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   board.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 16:43:57 by lumugot           #+#    #+#             //
//   Updated: 2026/04/24 19:14:28 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use rand::Rng;
use crate::{snake::{Snake, spawn_snake}};
use crate::logger::{ANSI_RED, ANSI_RESET};

pub const	GREEN_APPLE_COUNT: usize = 2;
pub const	DEFAULT_BOARD_SIZE: usize = 10;

const		MIN_BOARD_SIZE: usize = 4;
const		MAX_BOARD_SIZE: usize = 10;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction
{
	Up,
	Down,
	Left,
	Right,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cell
{
	Empty,
	SnakeHead,
	SnakeBody,
	GreenApple,
	RedApple,
}

#[derive(Clone, Debug)]
pub enum StepResult
{
	Moved,
	AteGreen,
	AteRed,
	GameOver,
}

pub struct Board
{
	pub size: usize,
	pub snake: Snake,
	pub green_apples: Vec<(usize, usize)>,
	pub red_apples: (usize, usize),
}

impl Board
{
	pub fn new(size: usize) -> Self
	{
		let snake = spawn_snake(size);

		let mut board = Board {
			size,
			snake,
			green_apples: Vec::new(),
			red_apples: (usize::MAX, usize::MAX),
		};

		for _ in 0..GREEN_APPLE_COUNT
		{
			let Some(pos) = board.randow_empty_cell() else { break ; };
			board.green_apples.push(pos);
		}

		if let Some(pos) = board.randow_empty_cell()
		{
			board.red_apples = pos;
		}

		board
	}

	fn randow_empty_cell(&self) -> Option<(usize, usize)>
	{
		let mut empty_cells: Vec<(usize, usize)> = Vec::new();

		for row in 0..self.size
		{
			for col in 0..self.size
			{
				let pos = (row, col);
				if !self.is_occupied(pos)
				{
					empty_cells.push(pos);
				}
			}
		}

		if empty_cells.is_empty()
		{
			return None;
		}

		let mut rng = rand::thread_rng();
		let idx = rng.gen_range(0..empty_cells.len());
		Some(empty_cells[idx])
	}

	fn is_occupied(&self, pos: (usize, usize)) -> bool
	{
		self.snake.occupies(pos)
			|| self.green_apples.contains(&pos)
			|| self.red_apples == pos
	}

	pub fn get_cell(&self, row: usize, col: usize) -> Cell
	{
		let pos = (row, col);

		if pos == self.snake.head() { Cell::SnakeHead }
		else if self.snake.occupies_body(pos) { Cell::SnakeBody }
		else if self.green_apples.contains(&pos) { Cell::GreenApple }
		else if self.red_apples == pos { Cell::RedApple }
		else { Cell::Empty }
	}

	pub fn step(&mut self, dir: Direction) -> StepResult
	{
		let next = self.snake.next_head(&dir);

		if next.0 >= self.size || next.1 >= self.size
		{
			self.snake.alive = false;
			return StepResult::GameOver;
		}

		if self.snake.occupies_body(next)
		{
			self.snake.alive = false;
			return StepResult::GameOver;
		}

		if self.green_apples.contains(&next)
		{
			self.snake.advance(dir, true);
			self.green_apples.retain(|&p| p != next);

			let Some(new_apple) = self.randow_empty_cell() else
			{
				self.snake.alive = false;
				return StepResult::GameOver;
			};

			self.green_apples.push(new_apple);
			return StepResult::AteGreen;
		}

		if self.red_apples == next
		{
			if self.snake.lenght() <= 1
			{
				self.snake.alive = false;
				return StepResult::GameOver;
			}

			self.snake.advance(dir, false);
			self.snake.body.pop();

			if let Some(pos) = self.randow_empty_cell()
			{
				self.red_apples = pos;
			}
			else
			{
				self.red_apples = (usize::MAX, usize::MAX);
			}

			return StepResult::AteRed;
		}

		self.snake.advance(dir, false);
		StepResult::Moved
	}
}

pub fn validate_board_size(size: u32) -> Option<usize>
{
	if size < MIN_BOARD_SIZE as u32
	{
		eprintln!(
			"{}[ERROR]{} --board-size {} is too small (minimum: {})",
			ANSI_RED, ANSI_RESET, size, MIN_BOARD_SIZE
		);
		return None ;
	}

	if size > MAX_BOARD_SIZE as u32
	{
		eprintln!(
			"{}[ERROR]{} --board-size {} is too large (maximum: {})",
			ANSI_RED, ANSI_RESET, size, MAX_BOARD_SIZE
		);
		return None ;
	}
	Some(size as usize)
}