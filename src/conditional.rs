use behaviour_result::BehaviourResult;
use behaviour_node::BehaviourNode;

/// A simple action performed on the actor.
///
/// This transforms the result into either `BehaviourResult::Success` or `BehaviourResult::Failure`,
/// depending on the result of the callback function.
///
pub struct Conditional<F> {
    pub name: &'static str,
    callback: F
}

impl <F> Conditional<F> {

    pub fn new(name: &'static str, callback: F) -> Conditional<F> {
        Conditional {
            name: name,
            callback: callback
        }
    }

}

impl <T, F> BehaviourNode<T> for Conditional<F>
    where F: FnMut(&mut T) -> bool {

    fn evaluate(&mut self, target: &mut T) -> BehaviourResult {
        let args = (target,);
        if self.callback.call_mut(args) {
            BehaviourResult::Success
        } else {
            BehaviourResult::Failure
        }

    }

}
