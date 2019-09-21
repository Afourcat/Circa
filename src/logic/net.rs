// Circa, efficient logic simulator
// Copyright (C) 2019 Lorenzo Cecchini
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::{RwLock, Weak};
use std::collections::HashSet;
use generational_arena::{Index, Arena};
use super::{Tristate, Component, LogicResult, LogicError};

pub struct Net {
    future_value: Vec<Tristate>,
    active_value: Vec<Tristate>,
    bit_width: usize,

    own_index: Option<Index>,
    arena: Weak<RwLock<Arena<Box<dyn Component>>>>,
    neighbors: HashSet<(Index, usize)>,
}

impl Net {
    pub fn new(bit_width: usize, component_arena: Weak<RwLock<Arena<Box<dyn Component>>>>) -> Net {
        Net {
            future_value: vec![Tristate::Floating; bit_width],
            active_value: vec![Tristate::Floating; bit_width],
            bit_width: bit_width,

            own_index: None,
            arena: component_arena,
            neighbors: HashSet::new(),
        }
    }

    pub fn set_index(&mut self, index: Index) {
        self.own_index = Some(index);
    }

    pub fn get(&self, bit: usize) -> Tristate {
        if bit < self.active_value.len() {
            self.active_value[bit]
        } else {
            Tristate::Floating
        }
    }

    pub fn set(&mut self, bit: usize, value: Tristate) {
        if bit < self.future_value.len() {
            self.future_value[bit] = value;
        }
    }

    pub fn spy(&self, bit: usize) -> Tristate {
        if bit < self.future_value.len() {
            self.future_value[bit]
        } else {
            Tristate::Floating
        }
    }

    pub fn overwrite(&mut self, bit: usize, value: Tristate) {
        if bit < self.active_value.len() {
            self.active_value[bit] = value;
        }
    }

    pub fn update(&mut self) {
        let active = &mut self.active_value;
        let future = &mut self.future_value;

        std::mem::swap(active, future);
        self.future_value = vec![Tristate::Floating; self.bit_width];
    }

    pub fn reset(&mut self) {
        self.active_value = vec![Tristate::Floating; self.bit_width];
        self.future_value = vec![Tristate::Floating; self.bit_width];
    }

    pub fn resize(&mut self, new_size: usize) {
        if self.bit_width < new_size {
            for _ in 0..(new_size - self.bit_width) {
                self.active_value.push(Tristate::Floating);
                self.future_value.push(Tristate::Floating);
            }
        } else {
            for _ in 0..(self.bit_width - new_size) {
                self.active_value.pop();
                self.future_value.pop();
            }
        }
        self.bit_width = new_size;
    }

    pub fn absorb(&mut self, net: &mut Net) -> LogicResult<()> {
        self.active_value = self.active_value.iter().zip(net.active_value.iter()).map(|(l, r)| l.merge(*r)).collect();
        self.future_value = self.future_value.iter().zip(net.future_value.iter()).map(|(l, r)| l.merge(*r)).collect();
        if self.own_index.is_none() {
            Err(LogicError::InvalidNet)
        } else if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            for (component, pin) in net.neighbors.drain() {
                if let Some(neighbor) = arena.get_mut(component) {
                    neighbor.disconnect(pin);
                    neighbor.connect(pin, self.own_index.unwrap());
                    self.neighbors.insert((component, pin));
                }
            }
            Ok(())
        } else {
            Err(LogicError::InvalidArena)
        }
    }

    pub fn read_u64(&self) -> Option<u64> {
        let mut weight = 1;
        let mut result = 0;

        for i in (0..self.bit_width).rev() {
            match self.active_value[i] {
                Tristate::Floating => return None,
                Tristate::Error    => return None,
                Tristate::High     => result = result | weight,
                Tristate::Low      => {},
            }
            weight = weight << 1;
        }
        Some(result)
    }

    pub fn write_u64(&mut self, value: u64) {
        let mut weight = 1;

        for i in (0..self.bit_width).rev() {
            self.future_value[i] = if value & weight != 0 {
                Tristate::High
            } else {
                Tristate::Low
            };
            weight = weight << 1;
        }
    }

    pub fn connect(&mut self, component: Index, pin: usize) -> LogicResult<()> {
        if self.own_index.is_none() {
            Err(LogicError::InvalidNet)
        } else if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            if let Some(neighbor) = arena.get_mut(component) {
                neighbor.connect(pin, self.own_index.unwrap());
                self.neighbors.insert((component, pin));
                Ok(())
            } else {
                Err(LogicError::InvalidComponent)
            }
        } else {
            Err(LogicError::InvalidArena)
        }
    }

    pub fn disconnect(&mut self, component: Index, pin: usize) -> LogicResult<()> {
        if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            self.neighbors.remove(&(component, pin));
            if let Some(neighbor) = arena.get_mut(component) {
                neighbor.disconnect(pin);
                Ok(())
            } else {
                Err(LogicError::InvalidComponent)
            }
        } else {
            Err(LogicError::InvalidArena)
        }
    }

    pub fn clear(&mut self) -> LogicResult<()> {
        self.reset();
        if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            for (component, pin) in self.neighbors.drain() {
                if let Some(neighbor) = arena.get_mut(component) {
                    neighbor.disconnect(pin);
                }
            }
            Ok(())
        } else {
            Err(LogicError::InvalidArena)
        }
    }
}
