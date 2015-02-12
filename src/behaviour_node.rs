use behaviour_result::BehaviourResult;

pub trait BehaviourNode<T> {
    fn evaluate(&mut self, &mut T) -> BehaviourResult;
}
