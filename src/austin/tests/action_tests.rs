// use austin::result::Result;
// use austin::node::Node;
// use austin::action::Action;
// use austin::tests::helpers;

// #[test]
// fn constructor_new() {
//     let action = Action::new("Test Action", || { Result::Success });
//     assert!(action.name == "Test Action");
// }

// #[test]
// fn evalute_returns_method_result() {
//     let results = helpers::result_methods();
//     for expected_result in results.iter() {
//         let mut action = Action::new("Test Action", || { expected_result.clone() });

//         let result = action.evaluate();
//         assert!(result == expected_result.clone());
//     }

// }
