//! A simple library for defining and evaluating behaviour trees on an actor.
//!
//! ## Building A Behaviour Tree
//! ```rust
//!
//! ```
//! ## Evaluating A Behaviour Tree
//! ```rust
//! ```
//!

#![doc(html_logo_url = "http://rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "http://rust-lang.org/favicon.ico",
       html_root_url = "https://archytaus.github.io/beehave/beehave")]

#![feature(core, unboxed_closures)]

pub use behaviour_result::BehaviourResult;
pub use behaviour_node::BehaviourNode;
pub use sequence::Sequence;
pub use selector::Selector;
pub use node::Node;
pub use action::Action;
pub use conditional::Conditional;
pub use conditional_decorator::ConditionalDecorator;

mod behaviour_result;
mod behaviour_node;

mod sequence;
mod selector;
mod node;
mod action;
mod conditional;
mod conditional_decorator;

#[macro_use]
pub mod macros;
