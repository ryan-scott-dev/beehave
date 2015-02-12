use beehave::behaviour_result::BehaviourResult;
use beehave::tree_node::TreeNode;
use beehave::node::Node;
use helpers;
use helpers::TestTarget;

#[test]
fn constructor_new() {
    let node = Node::new("Test Node", |_: &mut TestTarget| { BehaviourResult::Success });
    assert!(node.name == "Test Node");
}

#[test]
fn evalute_returns_method_result() {
    let expected_results = helpers::result_methods();
    let mut target = TestTarget::new();

    for expected_result in expected_results.iter() {
        let mut node = Node::new("Test Node", |_: &mut TestTarget| { expected_result.clone() });

        let result = node.evaluate(&mut target);
        assert!(result == expected_result.clone());
    }

}

#[test]
fn evalute_mutates_target() {
    let mut target = TestTarget::new();
    assert!(target.foo != true);
    assert!(target.bar != 32);

    let mut node = Node::new("Test Node", |target: &mut TestTarget| {
        target.foo = true;
        target.bar = 32;
        BehaviourResult::Success
    });

    let _ = node.evaluate(&mut target);
    assert!(target.foo == true);
    assert!(target.bar == 32);
}
