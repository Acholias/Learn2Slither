// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   state.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:45:43 by lumugot           #+#    #+#             //
//   Updated: 2026/04/20 23:48:22 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use serde::{Deserialize, Serialize};
use crate::board::{Board, Cell, Direction};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct State {
    pub danger_up: bool,
    pub danger_down: bool,
    pub danger_left: bool,
    pub danger_right: bool,
    pub apple_up: bool,
    pub apple_down: bool,
    pub apple_left: bool,
    pub apple_right: bool,
}

#[derive(Clone, Debug)]
pub struct Vision {
	pub head: (usize, usize),
	pub up: String,
	pub down: String,
	pub left: String,
	pub right: String,
}

impl State {
    pub fn to_index(&self) -> usize
    {
        let mut idx = 0usize;
        if self.danger_up    { idx |= 1 << 0; }
        if self.danger_down  { idx |= 1 << 1; }
        if self.danger_left  { idx |= 1 << 2; }
        if self.danger_right { idx |= 1 << 3; }
        if self.apple_up     { idx |= 1 << 4; }
        if self.apple_down   { idx |= 1 << 5; }
        if self.apple_left   { idx |= 1 << 6; }
        if self.apple_right  { idx |= 1 << 7; }
        idx
    }
}

pub fn compute_state(board: &Board) -> State
{
    State {
        danger_up:    is_danger(board, &Direction::Up),
        danger_down:  is_danger(board, &Direction::Down),
        danger_left:  is_danger(board, &Direction::Left),
        danger_right: is_danger(board, &Direction::Right),
        apple_up:     apple_visible(board, &Direction::Up),
        apple_down:   apple_visible(board, &Direction::Down),
        apple_left:   apple_visible(board, &Direction::Left),
        apple_right:  apple_visible(board, &Direction::Right),
    }
}

fn is_danger(board: &Board, dir: &Direction) -> bool
{
    let next = board.snake.next_head(dir);
    if next.0 >= board.size || next.1 >= board.size { return true; }
    if board.snake.occupies_body(next) { return true; }
    false
}

fn apple_visible(board: &Board, dir: &Direction) -> bool
{
    let (hr, hc) = board.snake.head();
    match dir
    {
        Direction::Up => board.green_apples.iter().any(|&(r, c)| c == hc && r < hr),
        Direction::Down => board.green_apples.iter().any(|&(r, c)| c == hc && r > hr),
        Direction::Left => board.green_apples.iter().any(|&(r, c)| r == hr && c < hc),
        Direction::Right => board.green_apples.iter().any(|&(r, c)| r == hr && c > hc),
    }
}

fn ray_string(board: &Board, row: usize, col: usize, dir: Direction) -> String
{
    let size = board.size as isize;
    let mut r = row as isize;
    let mut c = col as isize;
	
    let mut out = String::new();

    loop
    {
        match dir
        {
            Direction::Up       => r -= 1,
            Direction::Down     => r += 1,
            Direction::Left     => c -= 1,
            Direction::Right    => c += 1,
        }
		
        if r < 0 || c < 0 || r >= size || c >= size
        {
            out.push('W');
            break ;
        }

        let ch = match board.get_cell(r as usize, c as usize)
        {
            Cell::SnakeHead     => 'H',
            Cell::SnakeBody     => 'S',
            Cell::GreenApple    => 'G',
            Cell::RedApple      => 'R',
            Cell::Empty         => '0'
        };
        out.push(ch);
    }
    out
}

pub fn compute_vision(board: &Board) -> Vision
{
	let (row, col) = board.snake.head();

	Vision
	{
		head: (row, col),
		up: ray_string(board, row, col, Direction::Up),
		down: ray_string(board, row, col, Direction::Down),
		left: ray_string(board, row, col, Direction::Left),
		right: ray_string(board, row, col, Direction::Right),
	}
}
