use austin::result::Result;
use austin::node::Node;
use austin::conditional_decorator::ConditionalDecorator;
use austin::tests::helpers;
use austin::tests::helpers::TestTarget;

#[test]
fn evalute_child_with_true_success() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::with_child(
        "Test Conditional Decorator",
        |_: &mut TestTarget| { true },
        Box::new(helpers::success())
    );

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Success);
}

#[test]
fn evalute_child_with_true_failure() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::with_child(
        "Test Conditional Decorator",
        |_: &mut TestTarget| { true },
        Box::new(helpers::failure())
    );

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Failure);
}

#[test]
fn evalute_child_with_true_pending() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::with_child(
        "Test Conditional Decorator",
        |_: &mut TestTarget| { true },
        Box::new(helpers::pending())
    );

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Pending);
}

#[test]
fn evalute_conditional_failure() {
    let mut target = TestTarget::new();
    let mut conditional_decorator = ConditionalDecorator::with_child(
        "Test Conditional Decorator",
        |_: &mut TestTarget| { false },
        Box::new(helpers::raise_error())
    );

    let result = conditional_decorator.evaluate(&mut target);
    assert!(result == Result::Failure);
}
