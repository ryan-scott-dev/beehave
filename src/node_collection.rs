use node::Node;

pub trait NodeCollection<'a, T> {
    fn add(&mut self, Box<Node<T> + 'a>);
}
