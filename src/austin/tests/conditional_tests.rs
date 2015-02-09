// use austin::result::Result;
// use austin::node::Node;
// use austin::conditional::Conditional;

// #[test]
// fn constructor_new() {
//     let conditional = Conditional::new("Test Conditional", || { true });
//     assert!(conditional.name == "Test Conditional");
// }

// #[test]
// fn evalute_failure_with_false() {
//     let mut conditional = Conditional::new("Test Conditional", || { false });

//     let result = conditional.evaluate();
//     assert!(result == Result::Failure);
// }

// #[test]
// fn evalute_success_with_true() {
//     let mut conditional = Conditional::new("Test Conditional", || { true });

//     let result = conditional.evaluate();
//     assert!(result == Result::Success);
// }
