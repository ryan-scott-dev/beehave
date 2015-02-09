// use austin::result::Result;
// use austin::node::Node;
// use austin::conditional_decorator::ConditionalDecorator;
// use austin::tests::helpers;

// #[test]
// fn constructor_new() {
//     let conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { true });
//     assert!(conditional_decorator.name == "Test Conditional Decorator");
// }

// #[test]
// fn evalute_empty_failure_with_false() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { false });

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Failure);
// }

// #[test]
// fn evalute_empty_success_with_true() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { true });

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Success);
// }

// #[test]
// fn evalute_success_child_with_true_success() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { true });
//     conditional_decorator.success(Box::new(helpers::success()));
//     conditional_decorator.failure(Box::new(helpers::raise_error()));

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Success);
// }

// #[test]
// fn evalute_success_child_with_true_failure() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { true });
//     conditional_decorator.success(Box::new(helpers::failure()));
//     conditional_decorator.failure(Box::new(helpers::raise_error()));

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Failure);
// }

// #[test]
// fn evalute_success_child_with_true_pending() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { true });
//     conditional_decorator.success(Box::new(helpers::pending()));
//     conditional_decorator.failure(Box::new(helpers::raise_error()));

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Pending);
// }

// #[test]
// fn evalute_failure_child_with_true_success() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { false });
//     conditional_decorator.failure(Box::new(helpers::success()));
//     conditional_decorator.success(Box::new(helpers::raise_error()));

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Success);
// }

// #[test]
// fn evalute_failure_child_with_true_failure() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { false });
//     conditional_decorator.failure(Box::new(helpers::failure()));
//     conditional_decorator.success(Box::new(helpers::raise_error()));

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Failure);
// }

// #[test]
// fn evalute_failure_child_with_true_pending() {
//     let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", || { false });
//     conditional_decorator.failure(Box::new(helpers::pending()));
//     conditional_decorator.success(Box::new(helpers::raise_error()));

//     let result = conditional_decorator.evaluate();
//     assert!(result == Result::Pending);
// }
