/// The result from a `BehaviourNode` evaluating an actor.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BehaviourResult {
    Success,
    Failure,
    Pending
}
