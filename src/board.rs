// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   board.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 16:43:57 by lumugot           #+#    #+#             //
//   Updated: 2026/04/10 17:36:14 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use rand::Rng;
use crate::snake::{Snake, spawn_snake};

pub const   BOARD_SIZE: usize = 10;
pub const   GREEN_APPLE_COUNT:  usize = 2;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    SnakeHead,
    SnakeBody,
    GreenApple,
    RedApple,
}

#[derive(Clone, Debug)]
pub enum StepResult {
    Moved,
    AteGreen,
    AteRed,
    GameOver,
}

pub struct Board {
    pub size: usize,
    pub snake: Snake,
    pub green_apples: Vec<(usize, usize)>,
    pub red_apples: (usize, usize),
}

impl Board {
    pub fn new() -> Self
    {
        let snake = spawn_snake();

        let mut board = Board {
            size: BOARD_SIZE,
            snake,
            green_apples: Vec::new(),
            red_apples: (0, 0),
        };

        for _ in 0..GREEN_APPLE_COUNT
        {
            let pos = board.randow_empty_cell();
            board.green_apples.push(pos);
        }
        board.red_apples= board.randow_empty_cell();

        board
    }

    fn randow_empty_cell(&self) -> (usize, usize)
    {
        let mut rng= rand::thread_rng();
        loop
        {
            let pos = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));

            if !self.is_occupied(pos) { return pos; }
        }
    }

    fn is_occupied(&self, pos: (usize, usize)) -> bool
    {
        self.snake.occupies(pos) || self.green_apples.contains(&pos) || self.red_apples == pos
    }
}
