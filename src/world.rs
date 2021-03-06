// Copyright 2018 Steven Sheffey
// This file is part of heroesoftheswarm.
//
// heroesoftheswarm is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// heroesoftheswarm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with heroesoftheswarm.  If not, see <http://www.gnu.org/licenses/>.
use entity::{Bullet, Swarm};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Represents the state of the game's world
#[derive(Clone, Debug)]
pub struct World {
    /// The width of the world
    width: f32,
    /// The height of the world
    height: f32,
    /// Each swarm in the world
    /// Map of player ID to swarm
    swarms: HashMap<usize, Swarm>,
    /// Each bullet in the world
    /// Using a hashmap allows O(1) insertion and deletion
    /// The key is an increasing id
    /// TODO: vec and element swap
    bullets: HashMap<usize, Bullet>,
}
/// Functions for the world
impl World {
    /// Constructor
    /// width: the width of the world
    /// height: the height of the world
    pub fn new(width: f32, height: f32) -> Self {
        World {
            width: width,
            height: height,
            swarms: HashMap::new(),
            bullets: HashMap::new(),
        }
    }
    /// Capacity constructor
    /// width: the width of the world
    /// height: the height of the world
    /// capacity: the number players to allocate space for
    /// Space is allocated for 100x the number of bullets
    pub fn with_capacity(width: f32, height: f32, capacity: usize) -> Self {
        World {
            width: width,
            height: height,
            swarms: HashMap::with_capacity(capacity),
            bullets: HashMap::with_capacity(capacity * 10),
        }
    }
    /// Performs one "tick" of the world
    /// return: The amount of time elapsed during the tick
    /// Executes each swarm's program on itself
    /// Moves bullets
    /// Does bullet collision
    pub fn update(&mut self) -> Duration {
        // Record time at beginning of update
        let start_time = Instant::now();
        // Update each member of the swarm with its own program
        for (_id, swarm) in &mut self.swarms {
            swarm.update()
        }
        // Update each bullet
        // TODO: different logic for this as a bullet could be destroyed
        for (_id, bullet) in &mut self.bullets {
            bullet.update()
        }
        // Record time at end of update and return the time elapsed
        Instant::now().duration_since(start_time)
    }
    /// Returns the world in byte representation
    /// Used to render the world on a client
    pub fn serialize(&self) -> Vec<u8> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize_world() {
        let world = World::new(1000.0, 1000.0);
    }
}
