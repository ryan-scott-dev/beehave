use austin::result::Result;
use austin::node::Node;
use austin::node_collection::NodeCollection;

pub struct Selector<'a, T> {
    pub name: &'static str,
    pub children: Vec<Box<Node<T> + 'a>>
}

impl <'a, T> Selector<'a, T> {

    pub fn new(name: &'static str) -> Selector<T> {
        Selector {
            name: name,
            children: vec![]
        }
    }

}

impl <'a, T> Node<T> for Selector<'a, T> {

    fn evaluate(&mut self, target: &mut T) -> Result {

        for child in self.children.iter_mut() {
            let result = child.evaluate(target);
            match result {
                Result::Failure => { /* Null-Op */ },
                _ => { return result; }
            }
        }

        Result::Failure
    }

}

impl <'a, T> NodeCollection<'a, T> for Selector<'a, T> {
    fn add(&mut self, new_child: Box<Node<T> + 'a>) {
        self.children.push(new_child);
    }
}
