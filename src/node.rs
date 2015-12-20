use behaviour_result::BehaviourResult;
use behaviour_node::BehaviourNode;

/// A simple callback function performed on the actor.
///
/// The result of the callback function is used as the result of `evaluate`.
pub struct Node<F> {
    pub name: &'static str,
    callback: F
}

impl <F> Node<F> {

    pub fn new(name: &'static str, callback: F) -> Node<F> {
        Node {
            name: name,
            callback: callback
        }
    }

}

impl <T, F> BehaviourNode<T> for Node<F>
    where F: FnMut(&mut T) -> BehaviourResult {

    fn evaluate(&mut self, target: &mut T) -> BehaviourResult {
        let args = (target,);
        self.callback.call_mut(args)
    }

}
