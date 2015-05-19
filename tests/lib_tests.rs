extern crate genetic;
extern crate time;

#[cfg(test)]
mod tests {

    use time::PreciseTime;    
    use genetic::*;

    
    #[test]
    fn test_duplicate_string() {
        let start = PreciseTime::now();
        let gene_set = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!.";
        let target = "Not all those who wander are lost.";
        let best = genetic::get_best(get_fitness, display, target, gene_set, target.len(), start);
        println!("{}", best);
        println!("Total time: {}", start.to(PreciseTime::now()));
        assert!(best == target);
    }
    
    fn get_fitness(candidate: &String, target: &str) -> usize {
        let different_count = target.chars()
            .zip(candidate.chars())
            .filter(|&(a, b)| a != b)
            .count();

        target.len() - different_count
    }

    fn display(candidate: &String, target: &str, start: PreciseTime) {
        let now = PreciseTime::now();
        let elapsed = start.to(now);
        println!("{}\t{}\t{}", candidate, get_fitness(&candidate, target),elapsed);
    }
}
