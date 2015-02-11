use beehave::result::Result;
use beehave::tree_node::TreeNode;
use beehave::conditional::Conditional;
use helpers::TestTarget;

#[test]
fn constructor_new() {
    let conditional = Conditional::new("Test Conditional", |_: &mut TestTarget| { true });
    assert!(conditional.name == "Test Conditional");
}

#[test]
fn evalute_failure_with_false() {
    let mut conditional = Conditional::new("Test Conditional", |_: &mut TestTarget| { false });
    let mut target = TestTarget::new();

    let result = conditional.evaluate(&mut target);
    assert!(result == Result::Failure);
}

#[test]
fn evalute_success_with_true() {
    let mut conditional = Conditional::new("Test Conditional", |_: &mut TestTarget| { true });
    let mut target = TestTarget::new();

    let result = conditional.evaluate(&mut target);
    assert!(result == Result::Success);
}

#[test]
fn evalute_mutates_target() {
    let mut target = TestTarget::new();
    assert!(target.foo != true);
    assert!(target.bar != 32);

    let mut action = Conditional::new("Test Conditional", |target: &mut TestTarget| {
        target.foo = true;
        target.bar = 32;
        true
    });

    let _ = action.evaluate(&mut target);
    assert!(target.foo == true);
    assert!(target.bar == 32);
}
