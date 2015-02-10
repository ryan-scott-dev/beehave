use austin::result::Result;
use austin::node::Node;
use austin::conditional_decorator::ConditionalDecorator;
use austin::tests::helpers;
use austin::tests::helpers::TestTarget;

#[test]
fn constructor_new() {
    let conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", |_: &mut TestTarget| { true });
    assert!(conditional_decorator.name == "Test Conditional Decorator");
}

#[test]
fn evalute_empty_failure_with_false() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", |_: &mut TestTarget| { false });

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Failure);
}

#[test]
fn evalute_empty_success_with_true() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", |_: &mut TestTarget| { true });

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Success);
}

#[test]
fn evalute_child_with_true_success() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", |_: &mut TestTarget| { true });
    conditional_decorator.child(Box::new(helpers::success()));

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Success);
}

#[test]
fn evalute_child_with_true_failure() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", |_: &mut TestTarget| { true });
    conditional_decorator.child(Box::new(helpers::failure()));

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Failure);
}

#[test]
fn evalute_child_with_true_pending() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", |_: &mut TestTarget| { true });
    conditional_decorator.child(Box::new(helpers::pending()));

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Pending);
}

#[test]
fn evalute_conditional_failure() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::new("Test Conditional Decorator", |_: &mut TestTarget| { false });
    conditional_decorator.child(Box::new(helpers::raise_error()));

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Failure);
}
