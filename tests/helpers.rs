use beehave::result::Result;
use beehave::tree_node::TreeNode;

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

struct SuccessTreeNode;

impl <T> TreeNode<T> for SuccessTreeNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        Result::Success
    }
}

struct FailureTreeNode;

impl <T> TreeNode<T> for FailureTreeNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        Result::Failure
    }
}

struct PendingTreeNode;

impl <T> TreeNode<T> for PendingTreeNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        Result::Pending
    }
}

struct RaiseErrorTreeNode;

impl <T> TreeNode<T> for RaiseErrorTreeNode {
    fn evaluate(&mut self, _: &mut T) -> Result {
        panic!("I should not have been called...");
    }
}

pub fn success() -> SuccessTreeNode {
    SuccessTreeNode
}

pub fn failure() -> FailureTreeNode {
    FailureTreeNode
}

pub fn pending() -> PendingTreeNode {
    PendingTreeNode
}

pub fn raise_error() -> RaiseErrorTreeNode {
    RaiseErrorTreeNode
}

pub fn result_methods() -> [Result;3] {
    [ Result::Failure, Result::Success, Result::Pending ]
}
