use std::rand;
use std::rand::Rng;

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
        // rand::random::<f32>() > 0.5
        rand::thread_rng().gen::<f32>() > 0.5
    }

    pub fn can_shine(&self) -> bool {
        // rand::random::<f32>() > 0.5
        rand::thread_rng().gen::<f32>() > 0.5
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
