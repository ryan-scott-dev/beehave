use result::Result;
use tree_node::TreeNode;
use node_collection::TreeNodeCollection;

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

}

impl <'a, T> TreeNode<T> for Sequence<'a, T> {

    fn evaluate(&mut self, target: &mut T) -> Result {

        for child in self.children.iter_mut() {
            let result = child.evaluate(target);
            match result {
                Result::Success => { /* Null-Op */ },
                _ => { return result; }
            }
        }

        Result::Success
    }

}

impl <'a, T> TreeNodeCollection<'a, T> for Sequence<'a, T> {
    fn add(&mut self, new_child: Box<TreeNode<T> + 'a>) {
        self.children.push(new_child);
    }
}
