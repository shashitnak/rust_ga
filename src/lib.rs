#![allow(unused)]
#![deny(missing_docs)]

//! # Rust GA
//! 
//! A simple framework for testing genetic algorithms.

/// Module containing Population struct and Genome trait
pub mod ga;

#[cfg(test)]
mod tests {

    use super::ga::*;

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

    const TARGET: [char; 20] = ['T', 'o', ' ', 'b', 'e', ',',
        ' ', 'o', 'r', ' ', 'n', 'o', 't', ' ', 't', 'o', ' ', 'b', 'e', '.'];

    use std::iter::once;
    
    fn random_char() -> char {
        (rex(32, 122) as u8) as char
    }

    struct Gene {
        text: [char; 20]
    }

    impl Genome for Gene {
        fn new() -> Self {
            let mut text = [0u8 as char; 20];
            for i in 0..20 {
                text[i] = random_char();
            }
            Self { text }
        }

        fn fitness(&self) -> f64 {
            2f64.powf((0..20).map(|i| ((self.text[i] == TARGET[i]) as usize) as f64).sum())
        }

        fn cross(&self, other: &Self) -> Self {
            let mut text = [0u8 as char; 20];
            for i in 0..20 {
                text[i] = match radint(2) {
                    0 => self.text[i],
                    1 => other.text[i],
                    _ => random_char()
                }
            }

            Self { text }
        }

        fn mutate(mut self) -> Self {
            for i in 0..20 {
                if random() < 0.01 {
                    self.text[i] = random_char();
                }
            }
            self
        }

        fn display(&self) {
            let display_text: String = self.text.iter().cloned().collect();
            println!("{}", display_text);
        }
    }

    #[test]
    fn test_shakespear() {
        let mut population: Population<Gene> = Population::new(100);
        for i in 0..1000 {
            population.live();
            population.next_generation();
        }
    }
}
