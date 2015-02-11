use result::Result;

pub trait TreeNode<T> {
    fn evaluate(&mut self, &mut T) -> Result;
}
