use austin::result::Result;
use austin::node::Node;

pub struct ConditionalDecorator<'a, T, F> {
    pub name: &'static str,
    callback: F,
    child: Box<Node<T> + 'a>
}

impl <'a, T, F: FnMut(&mut T) -> bool + 'a> ConditionalDecorator<'a, T, F> {

    pub fn with_child(name: &'static str, callback: F, child: Box<Node<T> + 'a>) -> ConditionalDecorator<'a, T, F> {
        ConditionalDecorator {
            name: name,
            callback: callback,
            child: child
        }
    }

}

impl <'a, T: Clone, F: FnMut(&mut T) -> bool> Node<T> for ConditionalDecorator<'a, T, F> {

    fn evaluate(&mut self, target: &mut T) -> Result {
        if self.callback.call_mut((&mut target.clone(),)) {
            self.child.evaluate(target)
        } else {
            Result::Failure
        }
    }

}
