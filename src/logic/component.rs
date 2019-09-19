use generational_arena::Index;

pub trait Component {
    fn update(&mut self);

    fn disconnect(&mut self, pin: usize);
    fn connect(&mut self, pin: usize, net: Index);
}
