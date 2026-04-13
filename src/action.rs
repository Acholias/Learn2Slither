// ************************************************************************** //
//                                                                            //
//                                                        :::      ::::::::   //
//   action.rs                                          :+:      :+:    :+:   //
//                                                    +:+ +:+         +:+     //
//   By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+        //
//                                                +#+#+#+#+#+   +#+           //
//   Created: 2026/04/12 13:21:27 by lumugot           #+#    #+#             //
//   Updated: 2026/04/13 16:45:03 by lumugot          ###   ########.fr       //
//                                                                            //
// ************************************************************************** //

use serde::{Deserialize, Serialize};
use crate::board::Direction;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Action {
    Up,
    Right,
    Down,
    Left,
}

impl Action {
    pub const ALL: [Action; 4] = [Action::Up, Action::Right, Action::Down, Action::Left];

    pub fn to_direction(self) -> Direction
    {
        match self
        {
            Action::Up      => Direction::Up,
            Action::Right   => Direction::Right,
            Action::Down    => Direction::Down,
            Action::Left    => Direction::Left,
        }
    }
    
    #[allow(dead_code)]
    pub fn from_direction(dir: &Direction) -> Self
    {
        match dir 
        {
            Direction::Up       => Action::Up,
            Direction::Right    => Action::Right,
            Direction::Down     => Action::Down,
            Direction::Left     => Action::Left,
        }
    }

    pub fn index(self) -> usize
    {
        match self
        {
            Action::Up      => 0,
            Action::Right   => 1,
            Action::Down    => 2,
            Action::Left    => 3,

        }
    }
}
