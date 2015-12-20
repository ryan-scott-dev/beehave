#[macro_use]
extern crate beehave;

extern crate rand;

// Example based on the example provided by http://machinejs.maryrosecook.com

use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use beehave::{ Sequence, Selector, ConditionalDecorator, Action, BehaviourNode };

#[derive(Clone)]
pub struct World {
  pub groundwater: u32,
  pub oxygen: u32,
  pub sunny: bool
}

impl World {

    pub fn new() -> World {
        World {
            groundwater: 0,
            oxygen: 0,
            sunny: false
        }
    }

    pub fn is_sunny(&self) -> bool {
        self.sunny
    }

    pub fn has_water(&self) -> bool {
        self.groundwater > 0
    }

    pub fn can_rain(&self) -> bool {
        rand::random::<f32>() > 0.5
    }

    pub fn can_shine(&self) -> bool {
        rand::random::<f32>() > 0.5
    }

    pub fn give_water(&mut self) -> u32 {
        self.groundwater -= 1;
        1
    }

    pub fn oxygenate(&mut self) {
        self.oxygen += 1;
    }

    pub fn rain(&mut self) {
        self.groundwater += 1;
    }

    pub fn toggle_sun(&mut self) {
        self.sunny = !self.sunny;
    }

}

#[derive(Clone)]
pub struct Tree {
    pub energy: bool,
    pub oxygen: bool,
    pub height: u32,
    pub water: u32,
    pub sun: u32,
    pub world: Rc<RefCell<World>>
}

impl Tree {

    pub fn new(world: Rc<RefCell<World>>) -> Tree {
        Tree {
            world: world,
            height: 1,
            energy: false,
            oxygen: false,
            water: 0,
            sun: 0,
        }
    }

    pub fn has_water(&self) -> bool {
        self.water > 0
    }

    pub fn has_sun(&self) -> bool {
        self.sun > 0
    }

    pub fn can_grow(&self) -> bool {
        self.energy
    }

    pub fn can_make_energy(&self) -> bool {
          return self.has_water() && self.has_sun();
    }

    pub fn can_emit_oxygen(&self) -> bool {
        self.oxygen
    }

    pub fn can_gather_sun(&self) -> bool {
        let world = self.world.borrow();
        world.is_sunny()
    }

    pub fn can_gather_water(&self) -> bool {
        let world = self.world.borrow();
        world.has_water()
    }

    pub fn grow(&mut self) {
        self.energy = false;
        self.height += 1;
    }

    pub fn make_energy(&mut self) {
        self.sun -= 1;
        self.water -= 1;
        self.energy = true;
    }

    pub fn emit_oxygen(&mut self) {
        self.oxygen = false;
        let mut world = self.world.borrow_mut();
        world.oxygenate();
    }

    pub fn gather_sun(&mut self) {
        self.sun += 1;
    }

    pub fn gather_water(&mut self) {
        let mut world = self.world.borrow_mut();
        self.water += world.give_water();
    }
}

fn build_behaviour_trees() -> (Selector<'static, World>, Selector<'static, Tree>) {
    let world_behaviour: Selector<World> = behaviour_selector!("World Root", [
        condition_decorator!("Ensure Can Shine",
            |world: &mut World| {
                world.can_shine()
            },
            action!("Cycle Day/Night", |world: &mut World| {
                world.toggle_sun()
            })
        ),
        condition_decorator!("Ensure Can Rain",
            |world: &mut World| {
                world.can_rain()
            },
            action!("Rain", |world: &mut World| {
                world.rain()
            })
        )
    ]);

    let tree_behaviour: Selector<Tree> = behaviour_selector!("Tree Root", [
        behaviour_sequence!("Photosynthesise", [
            condition_decorator!("Ensure Can Make Energy",
                |tree: &mut Tree| {
                    tree.can_make_energy()
                },
                action!("Make Energy", |tree: &mut Tree| {
                    tree.make_energy()
                })
            ),
            condition_decorator!("Ensure Can Grow",
                |tree: &mut Tree| {
                    tree.can_grow()
                },
                action!("Grow", |tree: &mut Tree| {
                    tree.grow()
                })
            ),
            condition_decorator!("Ensure Can Emit Oxygen",
                |tree: &mut Tree| {
                    tree.can_emit_oxygen()
                },
                action!("Emit Oxygen", |tree: &mut Tree| {
                    tree.emit_oxygen()
                })
            )
        ]),
        condition_decorator!("Ensure Can Gather Sun",
            |tree: &mut Tree| {
                tree.can_gather_sun()
            },
            action!("Emit Oxygen", |tree: &mut Tree| {
                tree.gather_sun()
            })
        ),
        condition_decorator!("Ensure Can Gather Water",
            |tree: &mut Tree| {
                tree.can_gather_water()
            },
            action!("Emit Oxygen", |tree: &mut Tree| {
                tree.gather_water()
            })
        )
    ]);

    (world_behaviour, tree_behaviour)
}

fn main() {
    let world = Rc::new(RefCell::new(World::new()));
    let mut tree = Tree::new(world.clone());

    let (mut world_behaviour, mut tree_behaviour) = build_behaviour_trees();

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

        thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(test)]
extern crate test;

#[cfg(test)]
use test::Bencher;

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
