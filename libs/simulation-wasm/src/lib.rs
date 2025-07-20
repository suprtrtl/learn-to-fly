use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>
}

impl From<&sim::World> for World {
    fn from(value: &sim::World) -> Self {
        let animals = value.animals().iter().map(Animal::from).collect();
        Self { animals }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::Animal> for Animal {
    fn from(value: &sim::Animal) -> Self {
        Self {
            x: value.position().x,
            y: value.position().y
        }
    }
}
