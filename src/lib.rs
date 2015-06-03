extern crate rand;

pub mod genetic {

    use rand::{thread_rng, sample, Rng};

    pub fn get_best<F,D>(
        get_fitness: F, 
        display: D, 
        length: usize, 
        optimal_fitness_value: usize,
        gene_set: &str) -> Individual
        where F : Fn(&String)->usize,
        D : Fn(&Individual)
    {
        let mut best_parent = generate_parent(&get_fitness, gene_set, length);
        display(&best_parent);

        while best_parent.fitness < optimal_fitness_value {
            let mut candidate = generate_parent(&get_fitness, gene_set, length);
            let mut attempts_since_last_improvement = 0;
            while attempts_since_last_improvement < 100 {
                let child = match attempts_since_last_improvement % 3 {
                    0 => generate_parent(&get_fitness, gene_set, length),
                    1 => mutate_parent(&candidate, &get_fitness, gene_set),
                    _ => crossover(&candidate, &best_parent, &get_fitness)
                };

                if child.fitness > candidate.fitness { 
                    candidate = child;
                    attempts_since_last_improvement = 0;
                }
                attempts_since_last_improvement = attempts_since_last_improvement + 1;
            }
            if candidate.fitness > best_parent.fitness { 
                best_parent = candidate;
                display(&best_parent);
            }
        }

        best_parent 
    }

    fn generate_parent<F>(get_fitness: F, gene_set: &str, length: usize) -> Individual 
    where F : Fn(&String)->usize 
    {
        let mut rng = thread_rng();
        let sample = sample(&mut rng, gene_set.chars(), length);
        let genes = sample.into_iter().collect();
        let fitness = get_fitness(&genes);
        Individual {genes: genes, fitness: fitness }
    }

    fn mutate_parent<F>(parent1: &Individual, get_fitness: F, gene_set: &str) -> Individual
    where F : Fn(&String)->usize
    {
        let mut rng = thread_rng();
        let gene_index = rng.gen::<usize>() % gene_set.len();
        let parent_index = rng.gen::<usize>() % parent1.genes.len();
        let mut candidate = String::with_capacity(parent1.genes.len());

        if parent_index > 0 {
            candidate.push_str(&parent1.genes[..parent_index]);
        }
        candidate.push_str(&gene_set[gene_index..(1+gene_index)]);
        if parent_index+1 < parent1.genes.len() {
            candidate.push_str(&parent1.genes[parent_index+1..]);
        }
        let fitness = get_fitness(&candidate);
        Individual { genes: candidate, fitness: fitness }
    }
    
    fn crossover<F>(parent1: &Individual, parent2: &Individual, get_fitness: F) -> Individual
    where F : Fn(&String)->usize
    {
        let mut rng = thread_rng();
        let parent_index = rng.gen::<usize>() % parent1.genes.len();
        let mut candidate = String::with_capacity(parent1.genes.len());

        if parent_index > 0 {
            candidate.push_str(&parent1.genes[..parent_index]);
        }
        candidate.push_str(&parent2.genes[parent_index..(1+parent_index)]); // replace 1 gene
        if parent_index+1 < parent1.genes.len() {
            candidate.push_str(&parent1.genes[parent_index+1..]);
        }

        let fitness = get_fitness(&candidate);
        Individual { genes: candidate, fitness: fitness }
    }
    
    pub struct Individual {
        pub genes: String,
        pub fitness: usize
    }
}
