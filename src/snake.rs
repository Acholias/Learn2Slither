// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   snake.rs                                           :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/10 16:50:10 by lumugot           #+#    #+#             //
//   Updated: 2026/04/10 17:20:26 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use crate::board::{Direction, BOARD_SIZE};

#[derive(Clone, Debug)]
pub struct Snake {
    pub body: Vec<(usize, usize)>,
    pub direction: Direction,
    pub alive: bool,
}

impl Snake {
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

    pub fn next_head(&self, dir: &Direction) -> (usize, usize)
    {
        let (c, r) = self.head();

        match dir
        {
            Direction::Up       => (r.wrapping_sub(1), c),
            Direction::Down     => (r + 1, c),
            Direction::Left     => (r, c.wrapping_sub(1)),
            Direction::Right    => (r, c + 1),
        }
    }

    pub fn occupies(&self, pos: (usize, usize)) -> bool
    {
        self.body.contains(&pos)
    }

    pub fn occupies_body(&self, pos: (usize, usize)) -> bool
    {
        self.body[1..].contains(&pos)
    }

    pub fn advance(&mut self, dir: Direction, grow: bool) -> (usize, usize)
    {
        self.direction = dir.clone();
        let new_head = self.next_head(&dir);

        self.body.insert(0, new_head);
        
        if grow
        { 
            *self.body.last().unwrap()
        }
        else
        { 
            self.body.pop().unwrap()
        }
    }

    pub fn spawn_snake() -> Snake
    {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let row = rng.gen_range(1..BOARD_SIZE - 1);
        let col = rng.gen_range(2..BOARD_SIZE - 1);

        let body = vec! [
            (row, col)
            (row, col - 1)
            (row, col - 2)
        ];
    
        Snake::new(body, Direction::Right)
    }
}
