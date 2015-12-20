use beehave::{ BehaviourResult, BehaviourNode, Sequence };
use helpers;
use helpers::TestTarget;

#[test]
fn constructor_new() {
    let sequence = Sequence::<TestTarget>::new("Test Sequence");
    assert!(sequence.name == "Test Sequence");
}

#[test]
fn constructor_with_capacity() {
    let expected_capacity: usize = 10;
    let sequence = Sequence::<TestTarget>::with_capacity("Test Sequence", expected_capacity);
    assert!(sequence.name == "Test Sequence");
    assert!(sequence.children.capacity() == expected_capacity);
}

#[test]
fn evaluate_success_when_empty() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    let result = sequence.evaluate(&mut target);
    assert!(result == BehaviourResult::Success)
}

#[test]
fn evaluate_success_with_success() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::success()));

    let result = sequence.evaluate(&mut target);
    assert!(result == BehaviourResult::Success)
}

#[test]
fn evaluate_failure_with_failure() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::failure()));

    let result = sequence.evaluate(&mut target);
    assert!(result == BehaviourResult::Failure)
}

#[test]
fn evaluate_pending_with_pending() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::pending()));

    let result = sequence.evaluate(&mut target);
    assert!(result == BehaviourResult::Pending)
}

#[test]
fn evaluate_failure_short_circuits() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::success()));
    sequence.add(Box::new(helpers::failure()));
    sequence.add(Box::new(helpers::raise_error()));

    let result = sequence.evaluate(&mut target);
    assert!(result == BehaviourResult::Failure)
}

#[test]
fn evaluate_pending_short_circuits() {
    let mut sequence = Sequence::<TestTarget>::new("Test Sequence");
    let mut target = TestTarget::new();

    sequence.add(Box::new(helpers::success()));
    sequence.add(Box::new(helpers::pending()));
    sequence.add(Box::new(helpers::raise_error()));

    let result = sequence.evaluate(&mut target);
    assert!(result == BehaviourResult::Pending)
}
