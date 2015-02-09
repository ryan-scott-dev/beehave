// use austin::result::Result;
// use austin::node::Node;

// pub struct Conditional<F> {
//     pub name: &'static str,
//     pub callback: F
// }

// impl <F: Fn() -> bool> Conditional<F> {

//     pub fn new(name: &'static str, callback: F) -> Conditional<F> {
//         Conditional {
//             name: name,
//             callback: callback
//         }
//     }

// }

// impl <F: Fn() -> bool> Node for Conditional<F> {

//     fn evaluate(&mut self) -> Result {
//         let args = ();
//         if self.callback.call(args) {
//             Result::Success
//         } else {
//             Result::Failure
//         }

//     }

// }
