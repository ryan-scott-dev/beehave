use austin::result::Result;
use austin::node::Node;
use austin::action::Action;
use austin::tests::helpers;
use austin::tests::helpers::TestTarget;

#[test]
fn constructor_new() {
    let action = Action::new("Test Action", |_: &mut TestTarget| { Result::Success });
    assert!(action.name == "Test Action");
}

#[test]
fn evalute_returns_method_result() {
    let expected_results = helpers::result_methods();
    let mut target = TestTarget::new();

    for expected_result in expected_results.iter() {
        let mut action = Action::new("Test Action", |_: &mut TestTarget| { expected_result.clone() });

        let result = action.evaluate(&mut target);
        assert!(result == expected_result.clone());
    }

}

#[test]
fn evalute_mutates_the_target() {
    let mut target = TestTarget::new();
    assert!(target.foo != true);
    assert!(target.bar != 32);

    let mut action = Action::new("Test Action", |target: &mut TestTarget| {
        target.foo = true;
        target.bar = 32;
        Result::Success
    });

    let _ = action.evaluate(&mut target);
    assert!(target.foo == true);
    assert!(target.bar == 32);
}
