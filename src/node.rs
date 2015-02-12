use behaviour_result::BehaviourResult;
use behaviour_node::BehaviourNode;

pub struct Node<T, F> {
    pub name: &'static str,
    pub callback: F
}

impl <T, F: FnMut(&mut T) -> BehaviourResult> Node<T, F> {

    pub fn new(name: &'static str, callback: F) -> Node<T, F> {
        Node {
            name: name,
            callback: callback
        }
    }

}

impl <T, F: FnMut(&mut T) -> BehaviourResult> BehaviourNode<T> for Node<T, F> {

    fn evaluate(&mut self, target: &mut T) -> BehaviourResult {
        let args = (target,);
        self.callback.call_mut(args)
    }

}
