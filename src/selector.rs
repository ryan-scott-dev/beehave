use behaviour_result::BehaviourResult;
use behaviour_node::BehaviourNode;

/// A composite node which stops evaluating it's children and returns `BehaviourResult::Success` when a child returns `BehaviourResult::Success`.
/// Child nodes can also be added to this node.
/// A `BehaviourResult::Success` indicates that one of it's children return `BehaviourResult::Failure` when evaluated
/// A `BehaviourResult::Failure` indicates that none it's children returned `BehaviourResult::Success` when evaluated.
pub struct Selector<'a, T> {
    pub name: &'static str,
    pub children: Vec<Box<BehaviourNode<T> + 'a>>
}

impl <'a, T> Selector<'a, T> {

    pub fn new(name: &'static str) -> Selector<T> {
        Selector {
            name: name,
            children: vec![]
        }
    }

    pub fn with_capacity(name: &'static str, capacity: usize) -> Selector<'a, T> {
        Selector {
            name: name,
            children: Vec::with_capacity(capacity)
        }
    }

    pub fn with_children(name: &'static str, children: Vec<Box<BehaviourNode<T> + 'a>>) -> Selector<'a, T> {
        Selector {
            name: name,
            children: children
        }
    }

    pub fn children(&mut self, new_children: Vec<Box<BehaviourNode<T> + 'a>>) {
        self.children = new_children;
    }

    pub fn add(&mut self, new_child: Box<BehaviourNode<T> + 'a>) {
        self.children.push(new_child);
    }

}

impl <'a, T> BehaviourNode<T> for Selector<'a, T> {

    fn evaluate(&mut self, target: &mut T) -> BehaviourResult {

        for child in self.children.iter_mut() {
            let result = child.evaluate(target);
            match result {
                BehaviourResult::Failure => { /* Null-Op */ },
                _ => { return result; }
            }
        }

        BehaviourResult::Failure
    }

}
