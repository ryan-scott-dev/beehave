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

#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "TODO FIX ME")]

#![feature(core, unboxed_closures)]

pub mod behaviour_result;
pub mod behaviour_node;

pub mod sequence;
pub mod selector;
pub mod node;
pub mod action;
pub mod conditional;
pub mod conditional_decorator;

#[macro_use]
pub mod macros;
