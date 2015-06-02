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
            let child = mutate_parent(&best_parent, &get_fitness, gene_set);
            if child.fitness > best_parent.fitness {
                best_parent = child;
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
    
    pub struct Individual {
        pub genes: String,
        pub fitness: usize
    }
}
