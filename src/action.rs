use behaviour_result::BehaviourResult;
use behaviour_node::BehaviourNode;

/// A simple callback function performed on the actor. This always returns `BehaviourResult::Success`.
pub struct Action<F> {
    pub name: &'static str,
    callback: F
}

impl <F> Action<F> {

    pub fn new(name: &'static str, callback: F) -> Action<F> {
        Action {
            name: name,
            callback: callback
        }
    }

}

impl <T, F> BehaviourNode<T> for Action<F>
    where F: FnMut(&mut T) {

    fn evaluate(&mut self, target: &mut T) -> BehaviourResult {
        let args = (target,);
        self.callback.call_mut(args);
        BehaviourResult::Success
    }

}
