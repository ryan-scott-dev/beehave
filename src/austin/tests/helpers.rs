use austin::result::Result;
use austin::node::Node;

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

struct SuccessNode;

impl <T> Node<T> for SuccessNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        Result::Success
    }
}

struct FailureNode;

impl <T> Node<T> for FailureNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        Result::Failure
    }
}

struct PendingNode;

impl <T> Node<T> for PendingNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        Result::Pending
    }
}

struct RaiseErrorNode;

impl <T> Node<T> for RaiseErrorNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        panic!("I should not have been called...");
    }
}

pub fn success() -> SuccessNode {
    SuccessNode
}

pub fn failure() -> FailureNode {
    FailureNode
}

pub fn pending() -> PendingNode {
    PendingNode
}

pub fn raise_error() -> RaiseErrorNode {
    RaiseErrorNode
}

pub fn result_methods() -> [Result;3] {
    [ Result::Failure, Result::Success, Result::Pending ]
}
