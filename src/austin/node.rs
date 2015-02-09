use austin::result::Result;

pub trait Node<T> {
    fn evaluate(&mut self, &mut T) -> Result;
}
