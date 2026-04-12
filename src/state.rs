// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   state.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 19:45:43 by lumugot           #+#    #+#             //
//   Updated: 2026/04/12 13:20:25 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use serde::{Deserialize, Serialize};
use crate::board::{Board, Cell, Direction};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum VisionCell {
    Wall,
    Body,
    GreenApple,
    RedApple,
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct State {
    pub up: VisionCell,
    pub down: VisionCell,
    pub left: VisionCell,
    pub right: VisionCell,
}

impl State {
    pub fn to_index(&self) -> usize
    {
        fn encode(c: VisionCell) -> usize
        {
            match c
            {
                VisionCell::Wall        => 0,
                VisionCell::Body        => 1,
                VisionCell::GreenApple  => 2,
                VisionCell::RedApple    => 3,
                VisionCell::Empty       => 4,
            }
        }
        let b0 = encode(self.up);
        let b1 = encode(self.down);
        let b2 = encode(self.left);
        let b3 = encode(self.right);
        
        (((b0 * 5 + b1) * 5 + b2) * 5) + b3
    }
}

pub fn compute_state(board: &Board) -> State
{
    let (row, col) = board.snake.head();
    let up = first_visible_in_direction(board, row, col, Direction::Up);
    let down = first_visible_in_direction(board, row, col, Direction::Down);
    let left = first_visible_in_direction(board, row, col, Direction::Left);
    let right = first_visible_in_direction(board, row, col, Direction::Right);

    State { up, down, left, right }
}

fn first_visible_in_direction(board: &Board, row: usize, col: usize, dir: Direction) -> VisionCell
{
    let size = board.size as isize;
    let mut r = row as isize;
    let mut c = col as isize;

    loop
    {
        match dir
        {
            Direction::Up       => r -= 1,
            Direction::Down     => r += 1,
            Direction::Left     => c -= 1,
            Direction::Right    => c += 1,
        }
        
        if r < 0 || c < 0 || r >= size || c >= size { return VisionCell::Wall; }
        let cell = board.get_cell(r as usize, c as usize);
        
        match cell
        {
            Cell::SnakeHead | Cell::SnakeBody   => return VisionCell::Body,
            Cell::GreenApple                    => return VisionCell::GreenApple,
            Cell::RedApple                      => return VisionCell::RedApple,
            Cell::Empty                         => {}
        }
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

pub fn print_state(board: &Board)
{
    let (row, col) = board.snake.head();

    let up = ray_string(board, row, col, Direction::Up);
    let down = ray_string(board, row, col, Direction::Down);
    let left = ray_string(board, row, col, Direction::Left);
    let right = ray_string(board, row, col, Direction::Right);

    println!("Snake head at ({}, {})", row, col);
    println!("Vision (W/H/S/G/R/0):");
    println!("  Up   : {}", up);
    println!("  Down : {}", down);
    println!("  Left : {}", left);
    println!("  Right: {}", right);
}
