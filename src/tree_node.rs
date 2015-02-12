use behaviour_result::BehaviourResult;

pub trait TreeNode<T> {
    fn evaluate(&mut self, &mut T) -> BehaviourResult;
}
