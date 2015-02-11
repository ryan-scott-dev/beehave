use tree_node::TreeNode;

pub trait TreeNodeCollection<'a, T> {
    fn add(&mut self, Box<TreeNode<T> + 'a>);
}
