// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   snake.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 16:50:10 by lumugot           #+#    #+#             //
//   Updated: 2026/04/10 16:57:31 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use crate::board::{Direction, BOARD_SIZE};

#[derive(Clone, Debug)]
pub struct Snake {
    pub body: Vec<(usize, usize)>,
    pub direction: Direction,
    pub alive: bool,
}

impl Snake
{
    pub fn new(body: Vec<(usize, usize)>, direction: Direction) -> Self
    {
        Snake { body, direction, alive: true, }
    }

    pub fn head(&self) -> (usize, usize)
    {
        self.body[0]
    }

    pub fn lenght(&self) -> usize
    {
        self.body.len()
    }
}
