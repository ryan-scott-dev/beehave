#![feature(unboxed_closures)]
#![feature(core)]

// Demo depends on:
#![feature(io)]
#![feature(std_misc)]
#![feature(rand)]

extern crate test;
extern crate rand;

// Example based on the example provided by http://machinejs.maryrosecook.com

use std::cell::RefCell;
use std::rc::Rc;
use std::old_io::Timer;
use std::time::Duration;

#[macro_use]
mod austin;

use austin::sequence::Sequence;
use austin::selector::Selector;
use austin::conditional_decorator::ConditionalDecorator;
use austin::action::Action;
use austin::node::Node;
use austin::result::Result;

#[allow(dead_code)]
mod example;

use example::world::World;
use example::tree::Tree;

use test::Bencher;

fn build_behaviour_trees() -> (Selector<'static, World>, Selector<'static, Tree>) {
    let world_behaviour: Selector<World> = behaviour_selector!("World Root", [
        condition!("Ensure Can Shine",
            |world: &mut World| {
                world.can_shine()
            },
            action!("Cycle Day/Night", |world: &mut World| {
                world.toggle_sun();
                Result::Success
            })
        ),
        condition!("Ensure Can Rain",
            |world: &mut World| {
                world.can_rain()
            },
            action!("Rain", |world: &mut World| {
                world.rain();
                Result::Success
            })
        )
    ]);

    let photosynthesise: Sequence<Tree> = behaviour_sequence!("Photosynthesise", [
        condition!("Ensure Can Make Energy",
            |tree: &mut Tree| {
                tree.can_make_energy()
            },
            action!("Make Energy", |tree: &mut Tree| {
                tree.make_energy();
                Result::Success
            })
        ),
        condition!("Ensure Can Grow",
            |tree: &mut Tree| {
                tree.can_grow()
            },
            action!("Grow", |tree: &mut Tree| {
                tree.grow();
                Result::Success
            })
        ),
        condition!("Ensure Can Emit Oxygen",
            |tree: &mut Tree| {
                tree.can_emit_oxygen()
            },
            action!("Emit Oxygen", |tree: &mut Tree| {
                tree.emit_oxygen();
                Result::Success
            })
        )
    ]);

    let tree_behaviour: Selector<Tree> = behaviour_selector!("Tree Root", [
        photosynthesise,
        condition!("Ensure Can Gather Sun",
            |tree: &mut Tree| {
                tree.can_gather_sun()
            },
            action!("Emit Oxygen", |tree: &mut Tree| {
                tree.gather_sun();
                Result::Success
            })
        ),
        condition!("Ensure Can Gather Water",
            |tree: &mut Tree| {
                tree.can_gather_water()
            },
            action!("Emit Oxygen", |tree: &mut Tree| {
                tree.gather_water();
                Result::Success
            })
        )
    ]);

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
