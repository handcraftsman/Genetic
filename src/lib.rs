extern crate time;
extern crate rand;

pub mod genetic {
    use rand::{thread_rng, sample, Rng};
    use time::PreciseTime;

    pub fn get_best(get_fitness: fn(&String,&str) -> usize, display: fn(&String, &str, start: PreciseTime), target: &str, gene_set: &str, length: usize, start: PreciseTime) -> String {
        let mut best_parent = generate_parent(gene_set, length);
        let mut best_fitness = get_fitness(&best_parent, target);

        while best_fitness < length {
            let child = mutate_parent(&best_parent, gene_set);
            let fitness = get_fitness(&child, target);
            if fitness > best_fitness {
                best_fitness = fitness;
                best_parent = child;
                display(&best_parent, target, start);
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