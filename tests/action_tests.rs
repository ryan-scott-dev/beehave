use beehave::behaviour_result::BehaviourResult;
use beehave::tree_action::TreeAction;
use beehave::action::Action;
use helpers;
use helpers::TestTarget;

#[test]
fn constructor_new() {
    let action = Action::new("Test Action", |_: &mut TestTarget| { });
    assert!(action.name == "Test Action");
}

#[test]
fn evalute_returns_success() {
    let mut target = TestTarget::new();
    let mut action = Action::new("Test Action", |_: &mut TestTarget| { });

    let result = action.evaluate(&mut target);
    assert!(result == BehaviourResult::Success);
}

#[test]
fn evalute_mutates_target() {
    let mut target = TestTarget::new();
    assert!(target.foo != true);
    assert!(target.bar != 32);

    let mut action = Action::new("Test Action", |target: &mut TestTarget| {
        target.foo = true;
        target.bar = 32;
    });

    let _ = action.evaluate(&mut target);
    assert!(target.foo == true);
    assert!(target.bar == 32);
}
