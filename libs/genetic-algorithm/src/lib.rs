

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn evolve<I>(
        &self,
        population: &[I],
        evaluate_fitness: &dyn Fn(&I) -> f32,
    ) -> Vec<I> {
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