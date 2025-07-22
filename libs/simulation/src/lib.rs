mod animal;
mod animal_individual;
mod eye;
mod food;
mod world;

pub use self::{animal::*, animal_individual::*, eye::*, food::*, world::*};
use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use nalgebra as na;
use rand::{Rng, RngCore};
// FRAC_PI_2 = PI / 2.0
use std::f32::consts::FRAC_PI_2;

/// Minimum speed of a bird.
///
/// Keeping it above zero prevents bird from getting stuck
const SPEED_MIN: f32 = 0.001;

/// Maximum speed of a bird
///
/// prevents unrealistic behaviour
const SPEED_MAX: f32 = 0.005;

/// Speed acceleration, determains how quickly the bird can change speed
///
/// assuming bird is travelling with speed=5.0 how long it takes to stop
/// - 0.1 = 5 steps to stop
/// - 0.5 = 1 step to stop
const SPEED_ACCEL: f32 = 0.2;

/// Rotational acceleration, determains how quickly the bird changes direction
///
/// - 2 * PI = 1 step to 360 rotation
/// - PI = 2 steps to 360 rotaion
const ROTATION_ACCEL: f32 = FRAC_PI_2;

/// How many steps have to occur before we push data to genetic algo
///
/// Value too low may prevent birds from evolving
/// Value too high may make the simulation very slow
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection, 
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3,),
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// Performs a single step in simulation
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position(), &food.position());

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position, 
                animal.rotation,
                &self.world.foods
            );

            let response = animal.brain.propogate(vision);

            // Limit number to ranges
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;
        
        // Step 1: prepare to send birds into genetic algo
        let current_population = todo!();

        // Step 2: evolve birds
        let evolved_population = self.ga.evolve(rng, &current_population);
       
        // Step 3: bring birds back from algo
        self.world.animals = todo!();

        // Step 4: restart foods
        // for visual feedback (not neccesary)
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }
}
