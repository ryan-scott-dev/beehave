use austin::result::Result;
use austin::node::Node;
use austin::selector::{ Selector };
use austin::node_collection::NodeCollection;
use austin::tests::helpers;
use austin::tests::helpers::TestTarget;

#[test]
fn constructor_new() {
    let selector = Selector::<TestTarget>::new("Test Selector");
    assert!(selector.name == "Test Selector");
}

#[test]
fn constructor_with_capacity() {
    let expected_capacity: usize = 10us;
    let selector = Selector::<TestTarget>::with_capacity("Test Selector", expected_capacity);
    assert!(selector.name == "Test Selector");
    assert!(selector.children.capacity() == expected_capacity);
}

#[test]
fn evaluate_failure_when_empty() {
    let mut selector = Selector::<TestTarget>::new("Test Selector");
    let mut target = TestTarget::new();

    let result = selector.evaluate(&mut target);
    assert!(result == Result::Failure)
}

#[test]
fn evaluate_success_with_success() {
    let mut selector = Selector::<TestTarget>::new("Test Selector");
    let mut target = TestTarget::new();

    selector.add(Box::new(helpers::success()));

    let result = selector.evaluate(&mut target);
    assert!(result == Result::Success)
}

#[test]
fn evaluate_failure_with_failure() {
    let mut selector = Selector::<TestTarget>::new("Test Selector");
    let mut target = TestTarget::new();

    selector.add(Box::new(helpers::failure()));

    let result = selector.evaluate(&mut target);
    assert!(result == Result::Failure)
}

#[test]
fn evaluate_pending_with_pending() {
    let mut selector = Selector::<TestTarget>::new("Test Selector");
    let mut target = TestTarget::new();

    selector.add(Box::new(helpers::pending()));

    let result = selector.evaluate(&mut target);
    assert!(result == Result::Pending)
}

#[test]
fn evaluate_success_short_circuits() {
    let mut selector = Selector::<TestTarget>::new("Test Selector");
    let mut target = TestTarget::new();

    selector.add(Box::new(helpers::failure()));
    selector.add(Box::new(helpers::success()));
    selector.add(Box::new(helpers::raise_error()));

    let result = selector.evaluate(&mut target);
    assert!(result == Result::Success)
}

#[test]
fn evaluate_pending_short_circuits() {
    let mut selector = Selector::<TestTarget>::new("Test Selector");
    let mut target = TestTarget::new();

    selector.add(Box::new(helpers::failure()));
    selector.add(Box::new(helpers::pending()));
    selector.add(Box::new(helpers::raise_error()));

    let result = selector.evaluate(&mut target);
    assert!(result == Result::Pending)
}
