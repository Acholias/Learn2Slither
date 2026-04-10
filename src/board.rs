// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   board.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 16:43:57 by lumugot           #+#    #+#             //
//   Updated: 2026/04/10 17:16:48 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

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
    pub green_apples: Vec<(usize, usize)>,
    pub red_apples: (usize, usize),
}
