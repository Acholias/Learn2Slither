// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   board.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 16:43:57 by lumugot           #+#    #+#             //
//   Updated: 2026/04/10 19:11:19 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use rand::Rng;
use crate::snake::{Snake, spawn_snake};

pub const   BOARD_SIZE: usize = 15;
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

    pub fn get_cell(&self, row: usize, col: usize) -> Cell
    {
        let pos= (row, col);

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
            self.green_apples.retain(|&p| p != next); // Garde toutes les pommes vertes (sauf next)
            let new_apple = self.randow_empty_cell();
            self.green_apples.push(new_apple);
            return StepResult::AteGreen;
        }

        if self.red_apples == next
        {
            self.snake.advance(dir, false);
            self.snake.body.pop();
            if self.snake.lenght() == 0
            {
                self.snake.alive = false;
                return StepResult::GameOver;
            }
            self.red_apples = self.randow_empty_cell();
            return StepResult::AteRed;
        }
        self.snake.advance(dir, false);
        StepResult::Moved
    }
}
