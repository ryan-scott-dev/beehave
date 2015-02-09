// use austin::result::Result;
// use austin::node::Node;

// pub struct Action<F> {
//     pub name: &'static str,
//     pub callback: F
// }

// impl <F: FnMut() -> Result> Action<F> {

//     pub fn new(name: &'static str, callback: F) -> Action<F> {
//         Action {
//             name: name,
//             callback: callback
//         }
//     }

// }

// impl <F: FnMut() -> Result> Node for Action<F> {

//     fn evaluate(&mut self) -> Result {
//         let args = ();
//         self.callback.call_mut(args)
//     }

// }
