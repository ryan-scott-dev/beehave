use beehave::{ BehaviourResult, BehaviourNode };

#[derive(Clone, PartialEq, Eq)]
pub struct TestTarget {
    pub foo: bool,
    pub bar: u32
}

impl TestTarget {
    pub fn new() -> TestTarget {
        TestTarget {
            foo: false,
            bar: 1
        }
    }
}

pub struct SuccessBehaviourNode;

impl <T> BehaviourNode<T> for SuccessBehaviourNode {
    fn evaluate(&mut self, _: &mut T) -> BehaviourResult {
        BehaviourResult::Success
    }
}

pub struct FailureBehaviourNode;

impl <T> BehaviourNode<T> for FailureBehaviourNode {
    fn evaluate(&mut self, _: &mut T) -> BehaviourResult {
        BehaviourResult::Failure
    }
}

pub struct PendingBehaviourNode;

impl <T> BehaviourNode<T> for PendingBehaviourNode {
    fn evaluate(&mut self, _: &mut T) -> BehaviourResult {
        BehaviourResult::Pending
    }
}

pub struct RaiseErrorBehaviourNode;

impl <T> BehaviourNode<T> for RaiseErrorBehaviourNode {
    fn evaluate(&mut self, _: &mut T) -> BehaviourResult {
        panic!("I should not have been called...");
    }
}

pub fn success() -> SuccessBehaviourNode {
    SuccessBehaviourNode
}

pub fn failure() -> FailureBehaviourNode {
    FailureBehaviourNode
}

pub fn pending() -> PendingBehaviourNode {
    PendingBehaviourNode
}

pub fn raise_error() -> RaiseErrorBehaviourNode {
    RaiseErrorBehaviourNode
}

pub fn result_methods() -> [BehaviourResult;3] {
    [ BehaviourResult::Failure, BehaviourResult::Success, BehaviourResult::Pending ]
}
