// use austin::result::Result;
// use austin::node::Node;

// pub struct ConditionalDecorator<'a, F> {
//     pub name: &'static str,
//     pub callback: F,
//     pub success_child: Option<Box<Node + 'a>>,
//     pub failure_child: Option<Box<Node + 'a>>
// }

// impl <'a, F: Fn() -> bool> ConditionalDecorator<'a, F> {

//     pub fn new(name: &'static str, callback: F) -> ConditionalDecorator<'a, F> {
//         ConditionalDecorator {
//             name: name,
//             callback: callback,
//             success_child: None,
//             failure_child: None
//         }
//     }

//     pub fn success(&mut self, new_child: Box<Node + 'a>) {
//         self.success_child = Some(new_child);
//     }

//     pub fn failure(&mut self, new_child: Box<Node + 'a>) {
//         self.failure_child = Some(new_child);
//     }

// }

// impl <'a, F: Fn() -> bool> Node for ConditionalDecorator<'a, F> {

//     fn evaluate(&mut self) -> Result {
//         let args = ();
//         let (child, default) = {
//             if self.callback.call(args) {
//                 (self.success_child.as_mut(), Result::Success)
//             } else {
//                 (self.failure_child.as_mut(), Result::Failure)
//             }
//         };

//         match child {
//             Some(child) => child.evaluate(),
//             None => default
//         }
//     }

// }
