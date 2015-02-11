use result::Result;
use tree_node::TreeNode;

pub struct Action<T, F> {
    pub name: &'static str,
    pub callback: F
}

impl <T, F: FnMut(&mut T)> Action<T, F> {

    pub fn new(name: &'static str, callback: F) -> Action<T, F> {
        Action {
            name: name,
            callback: callback
        }
    }

}

impl <T, F: FnMut(&mut T)> TreeNode<T> for Action<T, F> {

    fn evaluate(&mut self, target: &mut T) -> Result {
        let args = (target,);
        self.callback.call_mut(args);
        Result::Success
    }

}
