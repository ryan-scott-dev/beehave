use result::Result;
use node::Node;

pub struct Conditional<T, F> {
    pub name: &'static str,
    pub callback: F
}

impl <T, F: FnMut(&mut T) -> bool> Conditional<T, F> {

    pub fn new(name: &'static str, callback: F) -> Conditional<T, F> {
        Conditional {
            name: name,
            callback: callback
        }
    }

}

impl <T, F: FnMut(&mut T) -> bool> Node<T> for Conditional<T, F> {

    fn evaluate(&mut self, target: &mut T) -> Result {
        let args = (target,);
        if self.callback.call_mut(args) {
            Result::Success
        } else {
            Result::Failure
        }

    }

}
