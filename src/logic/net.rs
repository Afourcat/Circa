use std::sync::{RwLock, Weak};
use std::collections::HashSet;
use generational_arena::{Index, Arena};
use super::{Tristate, Component};

pub struct Net {
    future_value: Tristate,
    active_value: Tristate,

    own_index: Option<Index>,
    arena: Weak<RwLock<Arena<Box<dyn Component>>>>,
    neighbors: HashSet<(Index, usize)>,
}

impl Net {
    pub fn new(component_arena: Weak<RwLock<Arena<Box<dyn Component>>>>) -> Net {
        Net {
            future_value: Tristate::Floating,
            active_value: Tristate::Floating,

            own_index: None,
            arena: component_arena,
            neighbors: HashSet::new(),
        }
    }

    pub fn set_index(&mut self, index: Index) {
        self.own_index = Some(index);
    }

    pub fn get(&self) -> Tristate {
        self.active_value
    }

    pub fn set(&mut self, value: Tristate) {
        self.future_value = value;
    }

    pub fn spy(&self) -> Tristate {
        self.future_value
    }

    pub fn overwrite(&mut self, value: Tristate) {
        self.active_value = value;
    }

    pub fn update(&mut self) {
        self.active_value = self.future_value;
        self.future_value = Tristate::Floating;
    }

    pub fn reset(&mut self) {
        self.active_value = Tristate::Floating;
        self.future_value = Tristate::Floating;
    }

    pub fn absorb(&mut self, net: &mut Net) {
        self.active_value = self.active_value.merge(net.active_value);
        self.future_value = self.future_value.merge(net.future_value);
        if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            for (component, pin) in net.neighbors.drain() {
                if let Some(neighbor) = arena.get_mut(component) {
                    neighbor.disconnect(pin);
                    // TODO: Error management on unwrap
                    neighbor.connect(pin, self.own_index.unwrap());
                    self.neighbors.insert((component, pin));
                }
            }
        }
    }

    pub fn connect(&mut self, component: Index, pin: usize) {
        if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            if let Some(neighbor) = arena.get_mut(component) {
                // TODO: Error management on unwrap
                neighbor.connect(pin, self.own_index.unwrap());
                self.neighbors.insert((component, pin));
            }
        }
    }

    pub fn disconnect(&mut self, component: Index, pin: usize) {
        if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            self.neighbors.remove(&(component, pin));
            if let Some(neighbor) = arena.get_mut(component) {
                neighbor.disconnect(pin);
            }
        }
    }

    pub fn clear(&mut self) {
        self.active_value = Tristate::Floating;
        self.future_value = Tristate::Floating;
        if let Some(mut arena) = self.arena.upgrade().as_ref().and_then(|l| l.try_write().ok()) {
            for (component, pin) in self.neighbors.drain() {
                if let Some(neighbor) = arena.get_mut(component) {
                    neighbor.disconnect(pin);
                }
            }
        }
    }
}
