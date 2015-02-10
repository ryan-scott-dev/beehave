use austin::result::Result;
use austin::node::Node;
use austin::sequence::Sequence;
use austin::conditional::Conditional;
use austin::node_collection::NodeCollection;

pub struct ConditionalDecorator<'a, T, F> {
    pub name: &'static str,
    internal: Sequence<'a, T>
}

impl <'a, T, F: FnMut(&mut T) -> bool + 'a> ConditionalDecorator<'a, T, F> {

    pub fn new(name: &'static str, callback: F) -> ConditionalDecorator<'a, T, F> {
        let mut internal = Sequence::new("Internal Hidden Sequence");
        let internal_conditional = Conditional::new("Internal Hidden Conditional", callback);
        internal.add(Box::new(internal_conditional));
        ConditionalDecorator {
            name: name,
            internal: internal
        }
    }

    pub fn child(&mut self, new_child: Box<Node<T> + 'a>) {
        self.internal.add(new_child);
    }

}

impl <'a, T, F: FnMut(&mut T) -> bool> Node<T> for ConditionalDecorator<'a, T, F> {

    fn evaluate(&mut self, target: &mut T) -> Result {
        self.internal.evaluate(target)
    }

}
