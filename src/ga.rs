#![allow(unused)]

extern crate rand;

use rand::Rng;

fn random() -> f64 {
    rand::thread_rng().gen()
}

fn radint(num: usize) -> usize {
    ((num as f64) * random()) as usize
}

fn rex(start: usize, end: usize) -> usize {
    start + radint(end - start)
}

/// Population struct that stores the population of genomes
pub struct Population<T: Genome> {
    genomes: Vec<T>,
    prob: Vec<f64>
}

impl<T: Genome> Population<T> {
    /// Creates a new Population with given size and Gene type.
    /// 
    ///  # Examples
    /// ```
    /// // Gene type need to implement the Genome trait for this to work
    /// let mut population: Population<Gene> = Population::new(size);
    /// ```
    pub fn new(size: usize) -> Self {
        Self {
            genomes: (0..size).map(|_| T::new()).collect(),
            prob: Vec::new()
        }
    }

    /// Returns the size of population
    pub fn size(&self) -> usize {
        self.genomes.len()
    }

    /// Calculates the fitness of all the members of population and displays the fittest
    /// member by calling the display method implemented in Genome trait.
    pub fn live(&mut self) {
        let mut total_score = 0.;
        let mut scores = Vec::with_capacity(self.size());
        let mut fittest: &T = &self.genomes[0];
        let mut max_score = 0.;
        
        for genome in &self.genomes {
            let score = genome.fitness();
            scores.push(score);
            total_score += score;

            if score > max_score {
                max_score = score;
                fittest = genome;
            }
        }

        scores.iter_mut().for_each(|val| *val = *val / total_score);
        self.prob = scores;
        fittest.display();
    }

    fn pick_one(&self) -> &T {
        let mut r = random();
        let mut index = 0;
        while r > 0. {
            r -= self.prob[index];
            index += 1;
        }

        &self.genomes[index - 1]
    }

    /// Replaces the current generation with a new generation
    /// 
    /// # Examples
    /// 
    /// ```
    /// for _ in 0..num_generations {
    ///     population.live();
    ///     population.next_generation();
    /// }
    /// ```
    /// 
    pub fn next_generation(&mut self) {
        self.genomes = (0..self.size())
            .map(|_| self.pick_one().cross(self.pick_one()).mutate())
            .collect();
    }
}

/// Trait required for a DNA to ge added to Population
pub trait Genome {
    /// Creates a new Genome
    fn new() -> Self;
    /// Computes the fitness of the Genome
    fn fitness(&self) -> f64;
    /// Creates a new Genome by crossing over 2 Genomes
    fn cross(&self, other: &Self) -> Self;
    /// Mutates a Genome
    fn mutate(self) -> Self;
    /// Displays the Genome
    fn display(&self);
}