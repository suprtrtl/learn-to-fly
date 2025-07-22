use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        todo!()
    }

    fn chromosome(&self) -> &ga::Chromosome {
        todo!()
    }

    fn fitness(&self) -> f32 {
        todo!()
    }
}
