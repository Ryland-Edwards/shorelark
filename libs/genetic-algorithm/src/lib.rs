

pub struct GeneticAlgorithm;

pub trait Individual {
    fn fitness(&self) -> f32;
}


impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
        where
            I: Individual,
        {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                todo!("Implement selection, \
                crossover, \
                and mutation")
            })
            .collect()
    }
}

pub trait SelectionMethod{
    fn select<'a,I>(&self, population: &'a [I]) -> &'a I
        where
            I: Individual;
}