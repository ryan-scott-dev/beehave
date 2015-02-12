use behaviour_result::BehaviourResult;
use tree_node::TreeNode;

pub struct Sequence<'a, T> {
    pub name: &'static str,
    pub children: Vec<Box<TreeNode<T> + 'a>>
}

impl <'a, T> Sequence<'a, T> {

    pub fn new(name: &'static str) -> Sequence<'a, T> {
        Sequence {
            name: name,
            children: vec![]
        }
    }

    pub fn with_capacity(name: &'static str, capacity: usize) -> Sequence<'a, T> {
        Sequence {
            name: name,
            children: Vec::with_capacity(capacity)
        }
    }

    pub fn with_children(name: &'static str, children: Vec<Box<TreeNode<T> + 'a>>) -> Sequence<'a, T> {
        Sequence {
            name: name,
            children: children
        }
    }

    pub fn children(&mut self, new_children: Vec<Box<TreeNode<T> + 'a>>) {
        self.children = new_children;
    }

    pub fn add(&mut self, new_child: Box<TreeNode<T> + 'a>) {
        self.children.push(new_child);
    }

}

impl <'a, T> TreeNode<T> for Sequence<'a, T> {

    fn evaluate(&mut self, target: &mut T) -> BehaviourResult {

        for child in self.children.iter_mut() {
            let result = child.evaluate(target);
            match result {
                BehaviourResult::Success => { /* Null-Op */ },
                _ => { return result; }
            }
        }

        BehaviourResult::Success
    }

}