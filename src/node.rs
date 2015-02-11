use result::Result;
use tree_node::TreeNode;

pub struct Node<T, F> {
    pub name: &'static str,
    pub callback: F
}

impl <T, F: FnMut(&mut T) -> Result> Node<T, F> {

    pub fn new(name: &'static str, callback: F) -> Node<T, F> {
        Node {
            name: name,
            callback: callback
        }
    }

}

impl <T, F: FnMut(&mut T) -> Result> TreeNode<T> for Node<T, F> {

    fn evaluate(&mut self, target: &mut T) -> Result {
        let args = (target,);
        self.callback.call_mut(args)
    }

}
