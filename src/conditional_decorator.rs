use behaviour_result::BehaviourResult;
use behaviour_node::BehaviourNode;

/// A decorator which only evaluates the child node if the callback function returns `true`.
pub struct ConditionalDecorator<'a, T, F> {
    pub name: &'static str,
    callback: F,
    child: Box<BehaviourNode<T> + 'a>
}

impl <'a, T, F: FnMut(&mut T) -> bool + 'a> ConditionalDecorator<'a, T, F> {

    pub fn with_child(name: &'static str, callback: F, child: Box<BehaviourNode<T> + 'a>) -> ConditionalDecorator<'a, T, F> {
        ConditionalDecorator {
            name: name,
            callback: callback,
            child: child
        }
    }

}

impl <'a, T: Clone, F: FnMut(&mut T) -> bool> BehaviourNode<T> for ConditionalDecorator<'a, T, F> {

    fn evaluate(&mut self, target: &mut T) -> BehaviourResult {
        if self.callback.call_mut((&mut target.clone(),)) {
            self.child.evaluate(target)
        } else {
            BehaviourResult::Failure
        }
    }

}
