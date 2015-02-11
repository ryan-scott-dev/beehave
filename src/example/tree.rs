use std::cell::RefCell;
use std::rc::Rc;

use world::World;

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
