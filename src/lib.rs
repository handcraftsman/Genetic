extern crate rand;

pub mod genetic {

    use rand::{thread_rng, sample, Rng};

    pub fn get_best<F,D>(
        get_fitness: F, 
        display: D, 
        length: usize, 
        optimal_fitness_value: usize,
        gene_set: &str) -> String
        where F : Fn(&String)->usize,
        D : Fn(&String)
    {
        let mut best_parent = generate_parent(gene_set, length);
        let mut best_fitness = get_fitness(&best_parent);
        display(&best_parent);

        while best_fitness < optimal_fitness_value {
            let child = mutate_parent(&best_parent, gene_set);
            let fitness = get_fitness(&child);
            if fitness > best_fitness {
                best_fitness = fitness;
                best_parent = child;
                display(&best_parent);
            }
        }

        best_parent 
    }

    fn generate_parent(gene_set: &str, length: usize) -> String {
        let mut rng = thread_rng();
        let sample = sample(&mut rng, gene_set.chars(), length);
        sample.into_iter().collect()
    }

    fn mutate_parent(parent: &String, gene_set: &str) -> String {
        let mut rng = thread_rng();
        let gene_index = rng.gen::<usize>() % gene_set.len();
        let parent_index = rng.gen::<usize>() % parent.len();
        let mut candidate = String::with_capacity(parent.len());

        if parent_index > 0 {
            candidate.push_str(&parent[..parent_index]);
        }
        candidate.push_str(&gene_set[gene_index..(1+gene_index)]);
        if parent_index+1 < parent.len() {
            candidate.push_str(&parent[parent_index+1..]);
        }
        candidate
    }
}