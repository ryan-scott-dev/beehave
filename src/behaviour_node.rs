use behaviour_result::BehaviourResult;

/// A type which is used to represent a node on the behaviour tree.
///
pub trait BehaviourNode<T> {
    fn evaluate(&mut self, &mut T) -> BehaviourResult;
}
