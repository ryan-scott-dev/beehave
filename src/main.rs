#![feature(unboxed_closures)]
#![feature(core)]

// Demo depends on:
#![feature(io)]
#![feature(std_misc)]
#![feature(rand)]

extern crate test;

// Example based on the example provided by http://machinejs.maryrosecook.com

use std::cell::RefCell;
use std::rc::Rc;
use std::old_io::Timer;
use std::time::Duration;

mod austin;

use austin::sequence::Sequence;
use austin::selector::Selector;
use austin::action::Action;
use austin::node::Node;
use austin::node_collection::NodeCollection;
use austin::result::Result;

#[allow(dead_code)]
mod example;

use example::world::World;
use example::tree::Tree;

use test::Bencher;

fn build_behaviour_trees() -> (Selector<'static, World>, Selector<'static, Tree>) {
    let mut world_behaviour: Selector<'static, World> = Selector::new("World Root");
    world_behaviour.add(Box::new(Action::new("Day/Night Cycle", |world: &mut World| {
            if world.can_shine() {
                world.toggle_sun();
                Result::Success
            } else {
                Result::Failure
            }
        }
    )));

    world_behaviour.add(Box::new(Action::new("Rain", |world: &mut World| {
            if world.can_rain() {
                world.rain();
                Result::Success
            } else {
                Result::Failure
            }
        }
    )));

    let mut tree_behaviour: Selector<'static, Tree> = Selector::new("Tree Root");

    let mut photosynthesise: Sequence<Tree> = Sequence::new("Photosynthesise");
    photosynthesise.add(Box::new(Action::new("Make Energy", |tree: &mut Tree| {
            if tree.can_make_energy() {
                tree.make_energy();
                Result::Success
            } else {
                Result::Failure
            }
        }
    )));
    photosynthesise.add(Box::new(Action::new("Grow", |tree: &mut Tree| {
            if tree.can_grow() {
                tree.grow();
                Result::Success
            } else {
                Result::Failure
            }
        }
    )));
    photosynthesise.add(Box::new(Action::new("Emit Oxygen", |tree: &mut Tree| {
            if tree.can_emit_oxygen() {
                tree.emit_oxygen();
                Result::Success
            } else {
                Result::Failure
            }
        }
    )));

    tree_behaviour.add(Box::new(photosynthesise));
    tree_behaviour.add(Box::new(Action::new("Gather Sun", |tree: &mut Tree| {
            if tree.can_gather_sun() {
                tree.gather_sun();
                Result::Success
            } else {
                Result::Failure
            }
        }
    )));
    tree_behaviour.add(Box::new(Action::new("Gather Water", |tree: &mut Tree| {
            if tree.can_gather_water() {
                tree.gather_water();
                Result::Success
            } else {
                Result::Failure
            }
        }
    )));

    (world_behaviour, tree_behaviour)
}

#[allow(dead_code)]
fn main() {
    let world = Rc::new(RefCell::new(World::new()));
    let mut tree = Tree::new(world.clone());

    let (mut world_behaviour, mut tree_behaviour) = build_behaviour_trees();

    let mut timer = Timer::new().unwrap();
    loop {
        {
            tree_behaviour.evaluate(&mut tree);
        }

        {
            let mut local_world = world.borrow_mut();
            world_behaviour.evaluate(&mut local_world);
        }

        {
            let local_world = world.borrow();
            println!("Tree:  {:2?}m | {:?} water | {:?} sun", tree.height, tree.water, tree.sun);
            println!("World:     | {:?} water | {:?} oxygen | {:5}", local_world.groundwater, local_world.oxygen, if local_world.sunny { "day" } else { "night" } );
            println!("");
        }

        timer.sleep(Duration::milliseconds(100));
    }
}

#[bench]
fn bench_evaluate_trees_and_world(b: &mut test::Bencher) {
    let world = Rc::new(RefCell::new(World::new()));
    let mut tree = Tree::new(world.clone());

    let (mut world_behaviour, mut tree_behaviour) = build_behaviour_trees();

    b.iter(|| {
        tree_behaviour.evaluate(&mut tree);

        let mut local_world = world.borrow_mut();
        world_behaviour.evaluate(&mut local_world)
    })
}

#[bench]
fn bench_build_behaviour_trees(b: &mut test::Bencher) {
    b.iter(|| {
        build_behaviour_trees()
    })
}
